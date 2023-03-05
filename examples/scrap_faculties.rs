fn main() {
    let scraper = kstu::Scraper::default();
    let future = scraper.scrap_faculties(Default::default());
    let faculties = futures_executor::block_on(future).unwrap();
    for faculty in faculties {
        println!(" - {}", faculty.name);
    }
}
