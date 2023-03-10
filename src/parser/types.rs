use itertools::Itertools;

// -- Faculty parsing --
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Faculty {
    pub name: String, // e.g. Институт цифровых технологий
    pub url: String,  // e.g. http://online.i-klgtu.ru/fulltime/current/10/
}

// -- Group parsing --
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Group {
    pub name: String, // e.g. 21-ВТ-1
    pub url: String,  // e.g. http://online.i-klgtu.ru/fulltime/current/10/21-%D0%92%D0%A2-1.html
}

// -- Timetable parsing --
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GroupTimetable {
    pub name: String,
    pub kind: GroupTimetableKind,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum GroupTimetableKind {
    Mono(Timetable),
    Dual(DualGroupTimetable),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DualGroupTimetable {
    pub names: (String, String),
    pub timetables: (Timetable, Timetable),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timetable {
    pub schedules: [Schedule; 12],
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Schedule {
    pub date: String,
    pub classes: [Option<Class>; 6],
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Class {
    pub name: String,
    pub time: String,
    pub lecturer: String,
    pub kind: ClassKind,
    pub room: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClassKind {
    Lecture,
    Practice,
    Lab,
}

// -- Impl blocks --
impl Class {
    pub(crate) fn new(time: String, name_raw: String, lecturer: String, room: String) -> Self {
        let mut name = name_raw.split_whitespace().collect_vec();
        let kind = name.pop().unwrap().to_lowercase();
        let kind = if kind.contains("лаб") {
            ClassKind::Lab
        } else if kind.contains("пра") {
            ClassKind::Practice
        } else if kind.contains("лек") {
            ClassKind::Lecture
        } else {
            error!("unmatched class kind: {kind}");
            ClassKind::Lecture
        };

        let name = name.join(" ");

        Self {
            name,
            time,
            lecturer,
            kind,
            room,
        }
    }
}

impl Schedule {
    pub(crate) fn new(classes: Vec<Option<Class>>, date: String) -> Self {
        Self {
            date,
            classes: classes.try_into().expect("unsupported number of classes in a single day!"),
        }
    }

    pub(crate) fn new2(classes: Vec<(Option<Class>, Option<Class>)>, date: String) -> (Self, Self) {
        let (a, b) = classes.into_iter().unzip();
        (Self::new(a, date.clone()), Self::new(b, date))
    }
}

impl Timetable {
    pub(crate) fn new(schedule: Vec<Schedule>) -> Self {
        Self {
            schedules: schedule.try_into().expect("unsupported number of days in a timetable"),
        }
    }

    pub(crate) fn new2(schedule: Vec<(Schedule, Schedule)>) -> (Self, Self) {
        let (a, b) = schedule.into_iter().unzip();
        (Self::new(a), Self::new(b))
    }
}

// pub enum Room {

// }
/*
без буквенных обозначений* - главный учебный корпус, Советский проспект, д. 1.
Б - учебный корпус № 1, ул. Профессора Баранова, 43
М - учебный корпус № 2, Малый переулок, 32
К - учебный корпус № 3 ул. Калязинская, 4
СК - Спортивный комплекс, учебный корпус № 2, Малый переулок, 32
А - корпус № 1 БГАРФ, ул. Молодежная, 6
А2 - корпус № 2 БГАРФ, ул. Озерная, 30
А3 - корпус № 3 БГАРФ, ул. Озерная, 32

*401Г - главный учебный корпус, Советский проспект, д. 1.
*/
