#[derive(Debug, Clone, Copy, Default)]
pub enum TimeFrame {
    Previous,
    #[default] Current,
    Preview,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Faculty {
    pub name: String, // e.g. Институт цифровых технологий
    pub url: String,  // e.g. http://online.i-klgtu.ru/fulltime/current/10/
}
