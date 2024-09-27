use log::error;
use sqlx::{Pool, Postgres};

pub struct Article {
    pub title: String,
    pub url: String,
    pub links_to: Vec<String>,
}

pub async fn connect() -> Pool<Postgres> {
    let connect_result = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:wikiscrape@localhost:5432")
        .await;

    match connect_result {
        Ok(pool) => {
            init_tables(&pool).await;
            return pool;
        }
        Err(e) => {
            error!("Failed to connect to postgres: {}", e);
            panic!();
        }
    }
}

async fn init_tables(pool: &Pool<Postgres>) {
    let create_articles_table = r#"
        CREATE TABLE IF NOT EXISTS articles (
            id SERIAL PRIMARY KEY,
            title TEXT NOT NULL,
            url TEXT NOT NULL,
            links_to TEXT[] NOT NULL
        )
    "#;

    let create_articles_table_result = sqlx::query(create_articles_table).execute(pool).await;

    match create_articles_table_result {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to create articles table: {}", e);
            panic!();
        }
    }
}

pub async fn insert_article(pool: &Pool<Postgres>, article: &Article) {
    let insert_article = r#"
        INSERT INTO articles (title, url, links_to)
        VALUES ($1, $2, $3)
    "#;

    let insert_article_result = sqlx::query(insert_article)
        .bind(&article.title)
        .bind(&article.url)
        .bind(&article.links_to)
        .execute(pool)
        .await;

    match insert_article_result {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to insert article: {}", e);
            panic!();
        }
    }
}
