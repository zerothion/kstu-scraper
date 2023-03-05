#[derive(Debug, Clone, Copy, Default)]
pub enum TimeFrame {
    Previous,
    #[default] Current,
    Preview,
}

impl TimeFrame {
    pub fn as_url_part(&self) -> &'static str {
        match self {
            TimeFrame::Previous => "old",
            TimeFrame::Current => "current",
            TimeFrame::Preview => "new",
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Faculty {
    pub name: String, // e.g. Институт цифровых технологий
    pub url: String,  // e.g. http://online.i-klgtu.ru/fulltime/current/10/
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Group {
    pub name: String, // e.g. 21-ВТ-1
    pub url: String,  // e.g. http://online.i-klgtu.ru/fulltime/current/10/21-%D0%92%D0%A2-1.html
}