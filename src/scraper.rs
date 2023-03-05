use crate::{
    Parser,
    types::{
        TimeFrame,
        Faculty,
    },
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("reqwest::get failed")]
    GetRequestFailed,

    #[error("failed to get text from response")]
    DecodingError,
}

#[derive(Debug, Clone, Default)]
pub struct Scraper {
    parser: Parser,
}

impl Scraper {
    pub async fn scrap_faculties(&self, when: TimeFrame) -> Result<Vec<Faculty>, Error> {
        let when = match when {
            TimeFrame::Previous => "old",
            TimeFrame::Current  => "current",
            TimeFrame::Preview  => "new",
        };
        let html = reqwest::get(format!("http://i-klgtu.ru/{}", when)).await;
        if let Ok(html) = html {
            if let Ok(text) = html.text().await {
                Ok(self.parser.parse_faculties(text))
            } else {
                Err(Error::DecodingError)
            }
        } else {
            Err(Error::GetRequestFailed)
        }
    }
}
