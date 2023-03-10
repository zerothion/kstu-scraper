#[macro_use] extern crate tracing;

pub mod parser;
pub mod scraper;

#[cfg(test)]
mod tests;

pub use crate::{
    parser::Parser,
    scraper::Scraper,
};

// https://crates.io/crates/scraper
// http://online.i-klgtu.ru/fulltime/current/10/
// http://online.i-klgtu.ru/fulltime/current/10/21-%D0%92%D0%A2-1.html
// http://online.i-klgtu.ru/fulltime/current/10/20-%D0%90%D0%9F(%D1%8D%D1%81).html
