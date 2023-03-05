fn main() {
    tracing_subscriber::fmt::init();
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            run().await;
        })
}

async fn run() {
    let scraper = kstu::Scraper::default();
    let faculties = scraper.scrap_faculties(Default::default()).await.unwrap();
    println!("faculties - {:?}", faculties);
}
