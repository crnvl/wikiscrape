use log::info;
use tokio;

mod db;
mod scrape;

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Info).init();
    
    info!("Connecting to postgres");
    let pool = db::connect().await;

    info!("Running scraper");
    let now = std::time::Instant::now();
    scrape::run(pool).await;
    info!("Scraping took {:?}", now.elapsed());

    info!("Done");
}