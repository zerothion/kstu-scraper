use crate::{
    Parser,
    types::{
        TimeFrame,
        Faculty, Group,
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

async fn scrap_text(url: &str) -> Result<String, Error> {
    let html = reqwest::get(url).await;
    if let Ok(html) = html {
        if let Ok(text) = html.text().await {
            Ok(text)
        } else {
            Err(Error::DecodingError)
        }
    } else {
        Err(Error::GetRequestFailed)
    }
}

impl Scraper {
    pub async fn scrap_faculties(&self, when: TimeFrame) -> Result<Vec<Faculty>, Error> {
        let url = format!("http://i-klgtu.ru/{}", when.as_url_part());
        let text = scrap_text(url.as_str()).await?;
        Ok(self.parser.parse_faculties(text))
    }

    pub async fn scrap_groups(&self, faculty: Faculty) -> Result<Vec<Group>, Error> {
        let text = scrap_text(faculty.url.as_str()).await?;
        let url = url::Url::parse(faculty.url.as_str())
            .expect("faculty.url should be a valid url");
        let base = url.host_str()
            .expect("faculty.url should always have a host");
        Ok(self.parser.parse_groups(text, &format!("http://{base}")))
    }

    pub async fn scrap_all_groups(&self, when: TimeFrame) -> Result<Vec<Group>, Error> {
        let mut groups = vec![];
        for faculty in self.scrap_faculties(when).await? {
            groups.append(
                &mut self.scrap_groups(faculty)
                    .await
                    .unwrap_or(vec![])
            );
        }
        Ok(groups)
    }
}
