use crate::types::Faculty;
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

        let a_selector = Selector::parse("a[href]").unwrap();
        let mut a = html.select(&a_selector);

        let mut faculties = vec![];
        while let Some(elem) = a.next() {
            let href = elem.value().attr("href").unwrap();
            if href.starts_with("http://online.i-klgtu.ru/fulltime/") {
                let faculty = Faculty {
                    name: elem.text().collect::<String>(),
                    url: href.to_string(),
                };

                if self.faculties_blacklist.contains(&faculty.name) {
                    continue;
                }

                info!("found faculty({}) --> {}", faculty.url, faculty.name);
                faculties.push(faculty);
            }
        }
        faculties
    }
}
