use crate::{
    Parser,
    types::{
        TimeFrame,
        Faculty,
        Group,
    },
};
use isahc::AsyncReadResponseExt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] isahc::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone, Default)]
pub struct Scraper {
    parser: Parser,
}

async fn scrap_text(url: &str) -> Result<String, Error> {
    let mut response = isahc::get_async(url).await?;
    let text = response.text().await?;
    Ok(text)
}

impl Scraper {
    pub async fn scrap_faculties(&self, when: TimeFrame) -> Result<Vec<Faculty>, Error> {
        let url = format!("https://i-klgtu.ru/{}/", when.as_url_part());
        let text = scrap_text(url.as_str()).await?;
        Ok(self.parser.parse_faculties(text))
    }

    pub async fn scrap_groups(&self, faculty: Faculty) -> Result<Vec<Group>, Error> {
        let text = scrap_text(faculty.url.as_str()).await?;
        Ok(self.parser.parse_groups(text))
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
