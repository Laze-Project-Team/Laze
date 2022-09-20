use std::io::{stderr, Write};

use crate::ParserData;

use super::{combinator::Matcher, peg_matcher::PegMatcher};

pub fn extract_string_data<T: ParserData + Clone>(
    data: Option<PegMatcher<T>>,
    name: &str,
    rule: &str,
) -> String {
    match data {
        Some(data) => data.get_string_data(name, rule),
        None => {
            let _ = writeln!(
                stderr(),
                "Could not find \"{}\" in the grammar to parse \"{}\"",
                name,
                rule
            );
            "".to_string()
        }
    }
}
pub fn extract_matcher_data<T: Clone + ParserData>(
    data: Option<PegMatcher<T>>,
    name: &str,
    rule: &str,
) -> Matcher<T> {
    match data {
        Some(data) => data.get_matcher_data(name, rule),
        None => {
            panic!(
                "Could not find \"{}\" in the grammar to parse \"{}\"",
                name, rule
            );
        }
    }
}
pub fn extract_matchers_data<T: ParserData + Clone>(
    data: Option<PegMatcher<T>>,
    name: &str,
    rule: &str,
) -> Vec<Matcher<T>> {
    match data {
        Some(data) => data.get_matchers_data(name, rule),
        None => {
            panic!(
                "Could not find \"{}\" in the grammar to parse \"{}\"",
                name, rule
            );
        }
    }
}
pub fn extract_rules_data<T: ParserData + Clone>(
    data: Option<PegMatcher<T>>,
    name: &str,
    rule: &str,
) -> Vec<(String, Matcher<T>)> {
    match data {
        Some(data) => data.get_rules_data(name, rule),
        None => {
            panic!(
                "Could not find \"{}\" in the grammar to parse \"{}\"",
                name, rule
            );
        }
    }
}
