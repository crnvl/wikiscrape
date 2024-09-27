use std::thread::JoinHandle;

use log::{error, info};
use sqlx::{Pool, Postgres};

use crate::db::{self, Article};

pub async fn run(pool: Pool<Postgres>) {
    let mut page_url = "https://en.wikipedia.org/wiki/Special:AllPages".to_string();
    let mut page_number = 0; // used only for printing
    let max_pages = 6888069 / 315; // 6888069 is the number of articles on English Wikipedia, each page has 315 articles

    loop {
        if page_number > max_pages {
            break;
        }
        info!("Page {}: Scraping {}", page_number + 1, page_url);
        let new_page_url = get_all_articles(pool.clone(), &page_url, page_number).await;

        page_number += 1;
        page_url = format!("https://en.wikipedia.org{}", new_page_url);
    }
}

async fn get_all_articles(pool: Pool<Postgres>, page_url: &str, page_number: i32) -> String {
    let response = reqwest::get(page_url).await.unwrap();

    if response.status() != 200 {
        error!("Failed to get page: {}", response.status());
    }

    let document = scraper::Html::parse_document(&response.text().await.unwrap());

    let next_page_binding = scraper::Selector::parse(
        format!(
            "div.mw-allpages-nav:nth-child(3) > a:nth-child({})",
            if page_number == 0 { 1 } else { 2 } // if article_number is 0, we are on the first page
        )
        .as_str(),
    )
    .unwrap();
    let next_page_url = document
        .select(&next_page_binding)
        .next()
        .unwrap()
        .value()
        .attr("href")
        .unwrap();

    let articles_list_binding = scraper::Selector::parse("li.allpagesredirect").unwrap();
    let articles_list = document.select(&articles_list_binding);

    let articles_count = articles_list.clone().count();
    info!("Deploying {} worker threads", articles_count);

    let mut handles = vec![];
    for article in articles_list {
        let article_url = format!(
            "https://en.wikipedia.org{}",
            article
                .select(&scraper::Selector::parse("a").unwrap())
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap()
        );
        let article_name = article.text().collect::<String>();

        let handle = std::thread::spawn(move || {
            info!(
                "Page {}: Looking up {} ({})",
                page_number + 1,
                article_name,
                article_url
            );

            let result = tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(lookup_article(&article_url, article_name));

            return result;
        });
        handles.push(handle);
    }

    info!("Waiting for worker threads to finish");
    for handle in handles {
        let article = handle.join().unwrap();
        db::insert_article(&pool, &article).await;
    }

    next_page_url.to_string()
}

async fn lookup_article(article_url: &str, article_name: String) -> Article {
    let response = reqwest::get(article_url).await.unwrap();

    if response.status() != 200 {
        error!("Failed to get article: {}", response.status());
    }

    let document = scraper::Html::parse_document(&response.text().await.unwrap());

    let article_binding = scraper::Selector::parse("#mw-content-text").unwrap();

    let article = document.select(&article_binding).next().unwrap();

    // get all links starting with /wiki/ without duplicates
    let links_binding = scraper::Selector::parse("a[href^=\"/wiki/\"]").unwrap();
    let unique_links = article
        .select(&links_binding)
        .map(|link| link.value().attr("href").unwrap())
        .collect::<std::collections::HashSet<_>>();

    db::Article {
        title: article_name,
        url: article_url.to_string(),
        links_to: unique_links.iter().map(|link| link.to_string()).collect(),
    }
}
