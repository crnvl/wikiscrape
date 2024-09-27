use log::info;
use tokio;

mod db;
mod scrape;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let db_username = std::env::var("DB_USERNAME").unwrap_or("postgres".to_string());
    let db_password = std::env::var("DB_PASSWORD").unwrap_or("postgres".to_string());
    let db_host = std::env::var("DB_HOST").unwrap_or("localhost".to_string());
    let db_port = std::env::var("DB_PORT").unwrap_or("5432".to_string());

    let log_level = std::env::var("RUST_LOG").unwrap_or("info".to_string());
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    info!("Connecting to postgres at {}:{}", db_host, db_port);
    let pool = db::connect(db_username, db_password, db_host, db_port).await;

    info!("Running scraper");
    let now = std::time::Instant::now();
    scrape::run(pool).await;
    info!("Scraping took {:?}", now.elapsed());

    info!("Done");
}
