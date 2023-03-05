fn main() {
    tracing_subscriber::fmt::init();
    let scraper = kstu::Scraper::default();
    let future = scraper.scrap_all_groups(Default::default());
    let groups = futures_executor::block_on(future).unwrap();
    for group in groups {
        println!(" - {}", group.name);
    }
}
