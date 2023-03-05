#[macro_use] extern crate tracing;

pub mod types;

use scraper::{Html, Selector};
use types::TimeFrame;
pub use types::{Faculty};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScrapingError {
    #[error("reqwest::get failed")]
    GetRequestFailed,

    #[error("failed to get text from response")]
    DecodingError,
}

pub async fn scrap_faculties(when: TimeFrame) -> Result<Vec<Faculty>, ScrapingError> {
    let when = match when {
        TimeFrame::Previous => "old",
        TimeFrame::Current => "current",
        TimeFrame::Preview => "new",
    };
    let html = reqwest::get(format!("http://i-klgtu.ru/{}", when)).await;
    if let Ok(html) = html {
        if let Ok(text) = html.text().await {
            Ok(parse_faculties(text))
        } else {
            Err(ScrapingError::DecodingError)
        }
    } else {
        Err(ScrapingError::GetRequestFailed)
    }
}

pub fn parse_faculties(html: String) -> Vec<Faculty> {
    let html = Html::parse_document(html.as_str());

    let a_selector = Selector::parse("a[href]").unwrap();
    let mut a = html.select(&a_selector);

    let mut faculties = vec![];
    while let Some(elem) = a.next() {
        let href = elem.value().attr("href").unwrap();
        if href.starts_with("http://online.i-klgtu.ru/fulltime/") {
            let blacklist = ["Преподаватели", "Аудитории"];

            let faculty = Faculty {
                name: elem.text().collect::<String>(),
                url: href.to_string(),
            };

            if blacklist.contains(&faculty.name.as_str()) {
                continue;
            }

            info!("found faculty({}) --> {}", faculty.url, faculty.name);
            faculties.push(faculty);
        }
    }
    faculties
}
