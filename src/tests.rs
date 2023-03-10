use crate::parser::types::*;

#[test]
fn parsing_faculties_01() {
    macro_rules! faculty {
        ($name:literal, $url:literal) => {
            Faculty{ name: $name.to_string(), url: $url.to_string() }
        }
    }
    let parser = crate::Parser::default();
    let faculties = parser.parse_faculties(include_str!("../test_data/faculties_01.html"));
    let faculties_exp = vec![
        faculty!("Институт цифровых технологий", "http://online.i-klgtu.ru/fulltime/current/10/"),
        faculty!("Институт морских технологий, энергетики и строительства", "http://online.i-klgtu.ru/fulltime/current/20/"),
        faculty!("Институт агроинженерии и пищевых систем", "http://online.i-klgtu.ru/fulltime/current/30/"),
        faculty!("Институт рыболовства и аквакультуры", "http://online.i-klgtu.ru/fulltime/current/40/"),
        faculty!("Институт отраслевой экономики и управления", "http://online.i-klgtu.ru/fulltime/current/50/"),
    ];
    assert_eq!(faculties, faculties_exp);
}

#[test]
fn parsing_groups_01() {
    macro_rules! group {
        ($name:literal, $url:literal) => {
            Group{ name: $name.to_string(), url: $url.to_string() }
        }
    }
    let parser = crate::Parser::default();
    let groups = parser.parse_groups(include_str!("../test_data/groups_01.html"));
    let groups_exp = vec![
        group!("18-ИБ(оиб)", "http://online.i-klgtu.ru/fulltime/current/10/18-%D0%98%D0%91(%D0%BE%D0%B8%D0%B1).html"),
        group!("19-АП(эс)", "http://online.i-klgtu.ru/fulltime/current/10/19-%D0%90%D0%9F(%D1%8D%D1%81).html"),
        group!("19-ВТ(рас)", "http://online.i-klgtu.ru/fulltime/current/10/19-%D0%92%D0%A2(%D1%80%D0%B0%D1%81).html"),
        group!("19-ИБ(оиб)", "http://online.i-klgtu.ru/fulltime/current/10/19-%D0%98%D0%91(%D0%BE%D0%B8%D0%B1).html"),
        group!("19-ИЭ(кис)", "http://online.i-klgtu.ru/fulltime/current/10/19-%D0%98%D0%AD(%D0%BA%D0%B8%D1%81).html"),
        group!("20-АП(эс)", "http://online.i-klgtu.ru/fulltime/current/10/20-%D0%90%D0%9F(%D1%8D%D1%81).html"),
        group!("20-ВТ-1", "http://online.i-klgtu.ru/fulltime/current/10/20-%D0%92%D0%A2-1.html"),
        group!("20-ВТ-2", "http://online.i-klgtu.ru/fulltime/current/10/20-%D0%92%D0%A2-2.html"),
        group!("20-ИБ(оиб)", "http://online.i-klgtu.ru/fulltime/current/10/20-%D0%98%D0%91(%D0%BE%D0%B8%D0%B1).html"),
        group!("20-ИЭ-1", "http://online.i-klgtu.ru/fulltime/current/10/20-%D0%98%D0%AD-1.html"),
        group!("20-ИЭ-2", "http://online.i-klgtu.ru/fulltime/current/10/20-%D0%98%D0%AD-2.html"),
        group!("21-АП", "http://online.i-klgtu.ru/fulltime/current/10/21-%D0%90%D0%9F.html"),
        group!("21-ВТ-1", "http://online.i-klgtu.ru/fulltime/current/10/21-%D0%92%D0%A2-1.html"),
        group!("21-ВТ-2", "http://online.i-klgtu.ru/fulltime/current/10/21-%D0%92%D0%A2-2.html"),
        group!("21-ИБ(бис)", "http://online.i-klgtu.ru/fulltime/current/10/21-%D0%98%D0%91(%D0%B1%D0%B8%D1%81).html"),
        group!("21-ИЭ-1", "http://online.i-klgtu.ru/fulltime/current/10/21-%D0%98%D0%AD-1.html"),
        group!("21-ИЭ-2", "http://online.i-klgtu.ru/fulltime/current/10/21-%D0%98%D0%AD-2.html"),
        group!("21-ИЭ-3", "http://online.i-klgtu.ru/fulltime/current/10/21-%D0%98%D0%AD-3.html"),
        group!("22-АП", "http://online.i-klgtu.ru/fulltime/current/10/22-%D0%90%D0%9F.html"),
        group!("22-ВТ-1", "http://online.i-klgtu.ru/fulltime/current/10/22-%D0%92%D0%A2-1.html"),
        group!("22-ВТ-2", "http://online.i-klgtu.ru/fulltime/current/10/22-%D0%92%D0%A2-2.html"),
        group!("22-ВТм", "http://online.i-klgtu.ru/fulltime/current/10/22-%D0%92%D0%A2%D0%BC.html"),
        group!("22-ИБ(бис)-1", "http://online.i-klgtu.ru/fulltime/current/10/22-%D0%98%D0%91(%D0%B1%D0%B8%D1%81)-1.html"),
        group!("22-ИБ(бис)-2", "http://online.i-klgtu.ru/fulltime/current/10/22-%D0%98%D0%91(%D0%B1%D0%B8%D1%81)-2.html"),
        group!("22-ИЭ-1", "http://online.i-klgtu.ru/fulltime/current/10/22-%D0%98%D0%AD-1.html"),
        group!("22-ИЭ-2", "http://online.i-klgtu.ru/fulltime/current/10/22-%D0%98%D0%AD-2.html"),
    ];
    assert_eq!(groups, groups_exp);
}


macro_rules! group_timetable {
    ($name:literal, $timetable:expr) => {
        GroupTimetable {
            name: $name.to_string(),
            kind: $timetable,
        }
    }
}

macro_rules! dual_timetable {
    ($name_a:literal, $timetable_a:expr, $name_b:literal, $timetable_b:expr) => {
        GroupTimetableKind::Dual(DualGroupTimetable {
            names: ($name_a.to_string(), $name_b.to_string()),
            timetables: ($timetable_a, $timetable_b),
        })
    };
}

macro_rules! timetable {
    ($($schedules:expr),+) => {
        Timetable {
            schedules: vec![$($schedules),+].try_into().unwrap()
        }
    };
}

macro_rules! schedule {
    ($date:literal, $($classes:expr),+) => {
        Schedule {
            date: $date.to_string(),
            classes: vec![$($classes),+].try_into().unwrap(),
        }
    };
}

macro_rules! class {
    ($when:literal, $what:literal, $who:literal, $where:literal) => {
        Some(Class::new($when.to_string(), $what.to_string(), $who.to_string(), $where.to_string()))
    }
}

#[test]
fn parsing_schedule_dual_01() {
    // "21-ВТ-1" dual_01
    let parser = crate::Parser::default();
    let html = include_str!("../test_data/schedule_dual_01.html");
    let timetable = parser.parse_timetable(html);
    let timetable_exp = group_timetable!(
        "21-ВТ-1",
        dual_timetable!(
            "21-ВТ-1-1",
            timetable!(
                schedule!(
                    "27.02.2023",
                    None,
                    None,
                    class!("12:10-13:35", "Математическое и имитационное моделирование (Лабораторные)", "Романов М.А.", "261.8"),
                    None,
                    None,
                    None
                ),
                schedule!(
                    "28.02.2023",
                    class!("9:00-10:25", "Электроника (Лабораторные)", "Капустин В.В.", "261.6"),
                    class!("10:35-12:00", "Электроника (Лабораторные)", "Капустин В.В.", "261.6"),
                    class!("12:10-13:35", "Методы научных исследований (лекции)", "Тугаринова Е.В.", "156"),
                    None,
                    None,
                    None
                ),
                schedule!(
                    "01.03.2023",
                    class!("9:00-10:25", "Математическая логика и теория алгоритмов (лекции)", "Топоркова О.М.", "142"),
                    class!("10:35-12:00", "Математическая логика и теория алгоритмов (Практические)", "Топоркова О.М.", "256"),
                    class!("12:10-13:35", "Методы научных исследований (Практические)", "Тугаринова Е.В.", "143"),
                    None,
                    class!("15:50-17:15", "Практическая подготовка по физической культуре и занятия спортом (элективные курсы) (Практические)", "Преподаватели кафедры ФКС", "СК"),
                    None
                ),
                schedule!(
                    "02.03.2023",
                    class!("9:00-10:25", "Операционные системы (Лабораторные)", "Лутовинова А. М.", "261.8"),
                    class!("10:35-12:00", "Иностранный язык (Практические)", "Плива Е.П.", "306"),
                    class!("12:10-13:35", "Вычислительная техника (лекции)", "Капустин В.В.", "266"),
                    None,
                    None,
                    None
                ),
                schedule!(
                    "03.03.2023",
                    class!("9:00-10:25", "Высокоуровневые технологии программирования (Лабораторные)", "Высоцкий Л.Г.", "261.16"),
                    class!("10:35-12:00", "Методы научных исследований (Лабораторные)", "Заболотнова Е.Ю.", "306 Г"),
                    class!("12:10-13:35", "Электроника (лекции)", "Капустин В.В.", "156"),
                    None,
                    class!("15:50-17:15", "Практическая подготовка по физической культуре и занятия спортом (элективные курсы) (Практические)", "Преподаватели кафедры ФКС", "СК"),
                    None
                ),
                schedule!(
                    "04.03.2023",
                    None,
                    None,
                    None,
                    None,
                    None,
                    None
                ),
                schedule!(
                    "06.03.2023",
                    class!("9:00-10:25", "Математическая логика и теория алгоритмов (лекции)", "Топоркова О.М.", "266"),
                    class!("10:35-12:00", "Высокоуровневые технологии программирования (лекции)", "Высоцкий Л.Г.", "156"),
                    None,
                    None,
                    None,
                    None
                ),
                schedule!(
                    "07.03.2023",
                    None,
                    class!("10:35-12:00", "Вычислительная техника (Лабораторные)", "Капустин В.В.", "261.7"),
                    class!("12:10-13:35", "Математическая логика и теория алгоритмов (Практические)", "Топоркова О.М.", "334"),
                    None,
                    None,
                    None
                ),
                schedule!(
                    "08.03.2023",
                    None,
                    None,
                    None,
                    None,
                    None,
                    None
                ),
                schedule!(
                    "09.03.2023",
                    None,
                    class!("10:35-12:00", "Операционные системы (лекции)", "Мацула В.Ф.", "142"),
                    class!("12:10-13:35", "Математическое и имитационное моделирование (лекции)", "Тристанов А.Б.", "142"),
                    class!("14:15-15:40", "Иностранный язык (Практические)", "Плива Е.П.", "471"),
                    None,
                    None
                ),
                schedule!(
                    "10.03.2023",
                    class!("9:00-10:25", "Электроника (лекции)", "Капустин В.В.", "156"),
                    None,
                    None,
                    None,
                    class!("15:50-17:15", "Практическая подготовка по физической культуре и занятия спортом (элективные курсы) (Практические)", "Преподаватели кафедры ФКС", "СК"),
                    None
                ),
                schedule!(
                    "11.03.2023",
                    None,
                    None,
                    None,
                    None,
                    None,
                    None
                )
            ),
            "21-ВТ-1-2",
            timetable!(
                schedule!(
                    "27.02.2023",
                    None,
                    None,
                    None,
                    None,
                    None,
                    None
                ),
                schedule!(
                    "28.02.2023",
                    class!("9:00-10:25", "Высокоуровневые технологии программирования (Лабораторные)", "Высоцкий Л.Г.", "261.16"),
                    class!("10:35-12:00", "Математическое и имитационное моделирование (Лабораторные)", "Романов М.А.", "261.8"),
                    class!("12:10-13:35", "Методы научных исследований (лекции)", "Тугаринова Е.В.", "156"),
                    class!("14:15-15:40", "Иностранный язык (Практические)", "Пахалюк В.Г.", "481"),
                    None,
                    None
                ),
                schedule!(
                    "01.03.2023",
                    class!("9:00-10:25", "Математическая логика и теория алгоритмов (лекции)", "Топоркова О.М.", "142"),
                    class!("10:35-12:00", "Математическая логика и теория алгоритмов (Практические)", "Топоркова О.М.", "256"),
                    class!("12:10-13:35", "Методы научных исследований (Практические)", "Тугаринова Е.В.", "143"),
                    None,
                    class!("15:50-17:15", "Практическая подготовка по физической культуре и занятия спортом (элективные курсы) (Практические)", "Преподаватели кафедры ФКС", "СК"),
                    None
                ),
                schedule!(
                    "02.03.2023",
                    class!("9:00-10:25", "Электроника (Лабораторные)", "Капустин В.В.", "261.6"),
                    class!("10:35-12:00", "Электроника (Лабораторные)", "Капустин В.В.", "261.6"),
                    class!("12:10-13:35", "Вычислительная техника (лекции)", "Капустин В.В.", "266"),
                    None,
                    None,
                    None
                ),
                schedule!(
                    "03.03.2023",
                    class!("9:00-10:25", "Методы научных исследований (Лабораторные)", "Заболотнова Е.Ю.", "261.17"),
                    class!("10:35-12:00", "Операционные системы (Лабораторные)", "Лутовинова А. М.", "261.8"),
                    class!("12:10-13:35", "Электроника (лекции)", "Капустин В.В.", "156"),
                    None,
                    class!("15:50-17:15", "Практическая подготовка по физической культуре и занятия спортом (элективные курсы) (Практические)", "Преподаватели кафедры ФКС", "СК"),
                    None
                ),
                schedule!(
                    "04.03.2023",
                    None,
                    None,
                    None,
                    None,
                    None,
                    None
                ),
                schedule!(
                    "06.03.2023",
                    class!("9:00-10:25", "Математическая логика и теория алгоритмов (лекции)", "Топоркова О.М.", "266"),
                    class!("10:35-12:00", "Высокоуровневые технологии программирования (лекции)", "Высоцкий Л.Г.", "156"),
                    None,
                    None,
                    None,
                    None
                ),
                schedule!(
                    "07.03.2023",
                    class!("9:00-10:25", "Вычислительная техника (Лабораторные)", "Капустин В.В.", "261.7"),
                    class!("10:35-12:00", "Иностранный язык (Практические)", "Пахалюк В.Г.", "474"),
                    class!("12:10-13:35", "Математическая логика и теория алгоритмов (Практические)", "Топоркова О.М.", "334"),
                    None,
                    None,
                    None
                ),
                schedule!(
                    "08.03.2023",
                    None,
                    None,
                    None,
                    None,
                    None,
                    None
                ),
                schedule!(
                    "09.03.2023",
                    None,
                    class!("10:35-12:00", "Операционные системы (лекции)", "Мацула В.Ф.", "142"),
                    class!("12:10-13:35", "Математическое и имитационное моделирование (лекции)", "Тристанов А.Б.", "142"),
                    class!("14:15-15:40", "Математическое и имитационное моделирование (Лабораторные)", "Романов М.А.", "261.8"),
                    None,
                    None
                ),
                schedule!(
                    "10.03.2023",
                    class!("9:00-10:25", "Электроника (лекции)", "Капустин В.В.", "156"),
                    class!("10:35-12:00", "Иностранный язык (Практические)", "Пахалюк В.Г.", "461"),
                    None,
                    None,
                    class!("15:50-17:15", "Практическая подготовка по физической культуре и занятия спортом (элективные курсы) (Практические)", "Преподаватели кафедры ФКС", "СК"),
                    None
                ),
                schedule!(
                    "11.03.2023",
                    None,
                    None,
                    None,
                    None,
                    None,
                    None
                )
            )
        )
    );

    assert_eq!(timetable, timetable_exp);
}

#[test]
fn parsing_schedule_mono_01() {
    // "21-АП" mono_01
    let parser = crate::Parser::default();
    let html = include_str!("../test_data/schedule_mono_01.html");
    let timetable = parser.parse_timetable(html);
    info!("{timetable:?}");
    assert!(false)
}
