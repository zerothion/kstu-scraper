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
    let groups = scraper.scrap_all_groups(Default::default())
        .await
        .unwrap();
    println!("groups - {:?}", groups);
}
