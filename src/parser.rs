use crate::types::{Faculty, Group};
use scraper::{Html, Selector};

#[derive(Debug, Clone)]
pub struct Parser {
    pub faculties_blacklist: Vec<String>,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            faculties_blacklist: vec!["Преподаватели".to_string(), "Аудитории".to_string()],
        }
    }
}

impl Parser {
    pub fn parse_faculties(&self, html: String) -> Vec<Faculty> {
        let html = Html::parse_document(html.as_str());

        let sel = Selector::parse("a[href]").unwrap();
        let mut sel = html.select(&sel);

        let mut faculties = vec![];
        while let Some(elem) = sel.next() {
            let href = elem.value().attr("href").unwrap();
            if href.starts_with("http://online.i-klgtu.ru/fulltime/") {
                match url::Url::parse(href) {
                    Ok(url) => {
                        let faculty = Faculty {
                            name: elem.text().collect::<String>(),
                            url: url.to_string(),
                        };

                        if self.faculties_blacklist.contains(&faculty.name) {
                            continue;
                        }

                        debug!("found faculty({}) --> {}", faculty.url, faculty.name);
                        faculties.push(faculty);
                    },
                    Err(err) => {
                        error!("invalid url: {}", err);
                    },
                }
            }
        }
        faculties
    }

    pub fn parse_groups(&self, html: String, url_base: &str) -> Vec<Group> {
        let html = Html::parse_document(html.as_str());
        let url_base = url::Url::parse(url_base)
            .expect("url_base should be a valid url");

        let sel = Selector::parse("a ~ a[href]").unwrap();
        let mut sel = html.select(&sel);

        let mut groups = vec![];
        while let Some(elem) = sel.next() {
            let href = elem.value().attr("href").unwrap();

            let url = url_base.join(href);                    // relative (always)
            let url = url.or_else(|_| url::Url::parse(href)); // absolute, just in case they messed up, or if it changes

            match url {
                Ok(url) => {
                    let group = Group {
                        name: elem.text().collect::<String>(),
                        url: url.to_string(),
                    };

                    debug!("found group {}", group.name);
                    groups.push(group);
                },
                Err(err) => {
                    error!("invalid url: {}", err);
                },
            }
        }
        groups
    }
}
