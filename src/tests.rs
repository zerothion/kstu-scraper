use crate::types::{Faculty, Group};

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
