use crate::parser::types::{
    Faculty, Group, Class, Schedule, Timetable, DualGroupTimetable, GroupTimetableKind,
};
use itertools::Itertools;
use scraper::{Html, Selector};

use self::types::GroupTimetable;

pub mod types;

#[derive(Debug, Clone)]
pub struct Parser {
    pub faculties_blacklist: Vec<String>,
    pub base_url: String,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            faculties_blacklist: vec!["Преподаватели".to_string(), "Аудитории".to_string()],
            base_url: "http://online.i-klgtu.ru".to_string(),
        }
    }
}

impl Parser {
    pub fn parse_faculties<S: AsRef<str>>(&self, html: S) -> Vec<Faculty> {
        let html = Html::parse_document(html.as_ref());

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
                            url: url.as_str().to_string(),
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

    pub fn parse_groups<S: AsRef<str>>(&self, html: S) -> Vec<Group> {
        let html = Html::parse_document(html.as_ref());
        let url_base = url::Url::parse(self.base_url.as_ref())
            .expect("base_url should be a valid url");

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
                        url: url.as_str().to_string(),
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

    pub fn parse_timetable<S: AsRef<str>>(&self, html: S) -> GroupTimetable {
        let html = Html::parse_document(html.as_ref());
        // 7th row -> group name
        // 8th row -> subgroup names (opt., only present if two subgroups)

        let row_sel = Selector::parse("table > tbody > tr").unwrap(); // extra specific just in case.
        let mut row_iter = html
            .select(&row_sel)
            .skip(6); // Skips useless "whitespace"

        let data_sel = Selector::parse("td[colspan]").unwrap();
        let group_name = row_iter
            .next().unwrap()
            .select(&data_sel)
            .skip(2)
            .next().unwrap()
            .text().collect::<String>();
        info!("group name - {group_name}");

        let subgroup_names = {
            let mut iter = row_iter
                .next().unwrap()
                .select(&data_sel)
                .skip(2); // Skips first two rows

            let sg_a = iter.next();
            let sg_b = iter.skip(1).next();

            match (sg_a, sg_b) {
                (Some(a), Some(b)) => Some((a.text().collect::<String>(), b.text().collect::<String>())),
                _ => None,
            }
        };
        info!("subgroups - {subgroup_names:?}");

        let schedules = row_iter.tuples::<(_, _)>().chunks(6).into_iter().map(|day_rows| {
            let mut date = None;
            let classes = day_rows.zip(0..).map(|((class, lecturer), i)| {
                let mut class_data = class.select(&data_sel);
                let mut lecturer_data = lecturer.select(&data_sel);

                if i == 0 {
                    date = class_data.next().map(|date| {
                        date.text().collect::<String>()
                    });
                }

                let hours = class_data.next()
                    .map_or("<missing>".to_string(), |x| x.text().collect::<String>());

                let left = class_data.next().map(|x| x.text().collect::<String>());
                let aud = class_data.next().map(|x| x.text().collect::<String>()).unwrap_or_default();
                let teacher = lecturer_data.next().map(|x| x.text().collect::<String>()).unwrap_or_default();
                let left =
                    left.map(|x|
                        Some(x)
                            .filter(|x| !x.is_empty())
                            .map(|name| Class::new(hours.clone(), name, teacher, aud))
                        );

                let right = class_data.next().map(|x| x.text().collect::<String>());
                let aud = class_data.next().map(|x| x.text().collect::<String>()).unwrap_or_default();
                let teacher = lecturer_data.next().map(|x| x.text().collect::<String>()).unwrap_or_default();
                let right =
                    right.map(|x|
                        Some(x)
                            .filter(|x| !x.is_empty())
                            .map(|name| Class::new(hours.clone(), name, teacher, aud))
                        );

                match (left, right) {
                    (Some(left), Some(right)) => {
                        (left, right)
                    },
                    (Some(dual), None) => {
                        (dual.clone(), dual)
                    },
                    (None, _) => (None, None),
                }
            }).collect_vec();

            let date = date.expect("date should always be present");
            let date = date.split_whitespace().next().unwrap();
            Schedule::new2(classes, date.to_string())
        }).collect_vec();
        let timetables = Timetable::new2(schedules);

        GroupTimetable {
            name: group_name,
            kind: match subgroup_names {
                Some(names) => GroupTimetableKind::Dual(
                    DualGroupTimetable { names, timetables, }
                ),
                None => GroupTimetableKind::Mono(timetables.0)
            },
        }
    }
}
