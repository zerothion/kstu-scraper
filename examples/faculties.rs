#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let _ = kstu_scraper::scrap_faculties(Default::default()).await;
}
