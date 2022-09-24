use std::io::{stderr, Write};

use crate::{combinator::*, Parser, ParserData};

use super::{combinator::Matcher, extracter::*};

#[derive(Clone)]
pub enum PegMatcher<T: ParserData + Clone + 'static> {
    Rules(Vec<(String, Matcher<T>)>),
    #[allow(dead_code)]
    Rule((String, Matcher<T>)),
    Matcher(Matcher<T>),
    Matchers(Vec<Matcher<T>>),
    String(String),
    None,
}

impl<T: ParserData + Clone> PegMatcher<T> {
    pub fn get_string_data(&self, name: &str, rule: &str) -> String {
        if let Self::String(str) = self {
            str.clone()
        } else {
            let _ = writeln!(stderr(), "{} in {} is not a string.", name, rule);
            "".to_string()
        }
    }
    pub fn get_matcher_data(&self, name: &str, rule: &str) -> Matcher<T> {
        if let Self::Matcher(matcher) = self {
            matcher.clone()
        } else {
            panic!("{} in {} is not a matcher.", name, rule);
        }
    }
    pub fn get_matchers_data(&self, name: &str, rule: &str) -> Vec<Matcher<T>> {
        if let Self::Matchers(matcher) = self {
            matcher.clone()
        } else {
            panic!("{} in {} is not a matcher.", name, rule);
        }
    }
    pub fn get_rules_data(&self, name: &str, rule: &str) -> Vec<(String, Matcher<T>)> {
        if let Self::Rules(matcher) = self {
            matcher.clone()
        } else {
            panic!("{} in {} is not a matcher.", name, rule);
        }
    }
}

impl<T: ParserData + Clone + 'static> ParserData for PegMatcher<T> {
    fn string(str: String) -> Self {
        Self::String(str)
    }
    fn null() -> Self {
        Self::None
    }
    fn data(name: String, parser: &mut Parser<Self>) -> Self {
        // println!("Reducing: {}", name);
        match name.as_str() {
            "StringContent" => {
                let content = extract_string_data(
                    parser.get_data("content".to_string()),
                    "content",
                    name.as_str(),
                );
                let newcontent = match content.as_str() {
                    "\\\"" => "\"".to_string(),
                    "\\\\" => "\\".to_string(),
                    "\\n" => "\n".to_string(),
                    str => str.to_string(),
                };
                // println!("StringContent: {newcontent}");
                Self::String(newcontent)
            }
            "String" => Self::Matcher(parse_seq(vec![
                parse_str(
                    extract_string_data(
                        parser.get_data("StringContent".to_string()),
                        "StringContent",
                        name.as_str(),
                    )
                    .clone(),
                ),
                parse_many(parse_or(vec![
                    parse_str(" ".to_string()),
                    parse_str("\n".to_string()),
                    parse_str("\t".to_string()),
                    parse_str("\r\n".to_string()),
                ])),
            ])),
            "RangeContent" => Self::String(extract_string_data(
                parser.get_data("content".to_string()),
                "content",
                name.as_str(),
            )),
            "Range" => Self::Matcher(parse_range(extract_string_data(
                parser.get_data("RangeContent".to_string()),
                "RangeContent",
                name.as_str(),
            ))),
            "NonTerminal" => Self::String(extract_string_data(
                parser.get_data("name".to_string()),
                "name",
                name.as_str(),
            )),
            "NonTerminalToken" => Self::Matcher(parse_ref(
                extract_string_data(
                    parser.get_data("NonTerminal".to_string()),
                    "NonTerminal",
                    name.as_str(),
                ),
                match parser.get_data("Rename".to_string()) {
                    Some(matcher) => match matcher {
                        Self::String(str) => Some(str),
                        _ => {
                            panic!("Rename is not matcher or null.");
                        }
                    },
                    None => None,
                },
            )),
            "Token" => match parser.get_data_from_parent_scope("Token".to_string()) {
                Some(matcher) => match matcher {
                    PegMatcher::Matcher(m) => Self::Matchers(vec![
                        m,
                        extract_matcher_data(
                            parser.get_data("tokendata".to_string()),
                            "tokendata",
                            name.as_str(),
                        ),
                    ]),
                    PegMatcher::Matchers(mut m) => {
                        m.push(extract_matcher_data(
                            parser.get_data("tokendata".to_string()),
                            "tokendata",
                            name.as_str(),
                        ));
                        Self::Matchers(m)
                    }
                    _ => {
                        panic!("The last token is not a matcher.");
                    }
                },
                None => Self::Matchers(vec![extract_matcher_data(
                    parser.get_data("tokendata".to_string()),
                    "tokendata",
                    name.as_str(),
                )]),
            },
            "AnyToken" => Self::Matcher(parse_any()),
            "RawToken" => Self::Matcher(extract_matcher_data(
                parser.get_data("tokendata".to_string()),
                "tokendata",
                name.as_str(),
            )),
            "ManyToken" => Self::Matcher(parse_many(extract_matcher_data(
                parser.get_data("RawToken".to_string()),
                "RawToken",
                name.as_str(),
            ))),
            "MoreThanOneToken" => Self::Matcher(parse_more_than_one(extract_matcher_data(
                parser.get_data("RawToken".to_string()),
                "RawToken",
                name.as_str(),
            ))),
            "NotToken" => Self::Matcher(parse_not(extract_matcher_data(
                parser.get_data("RawToken".to_string()),
                "RawToken",
                name.as_str(),
            ))),
            "Tokens" => match parser.get_data_from_parent_scope("Tokens".to_string()) {
                Some(matcher) => match matcher {
                    PegMatcher::Matcher(m) => {
                        let mut matchers = vec![m];
                        matchers.push(parse_seq(extract_matchers_data(
                            parser.get_data("Token".to_string()),
                            "Token",
                            name.as_str(),
                        )));
                        Self::Matchers(matchers)
                    }
                    PegMatcher::Matchers(mut m) => {
                        m.push(parse_seq(extract_matchers_data(
                            parser.get_data("Token".to_string()),
                            "Token",
                            name.as_str(),
                        )));
                        Self::Matchers(m)
                    }
                    _ => {
                        panic!("The last token is not a matcher.");
                    }
                },
                None => {
                    let matchers = extract_matchers_data(
                        parser.get_data("Token".to_string()),
                        "Token",
                        name.as_str(),
                    );
                    Self::Matcher(parse_seq(matchers))
                }
            },
            "ParenTokens" => Self::Matcher(extract_matcher_data(
                parser.get_data("OrTokens".to_string()),
                "OrTokens",
                name.as_str(),
            )),
            "OrTokens" => match parser.get_data("Tokens".to_string()) {
                Some(m) => match m {
                    PegMatcher::Matcher(m) => PegMatcher::Matcher(m),
                    PegMatcher::Matchers(m) => PegMatcher::Matcher(parse_or(m)),
                    _ => {
                        panic!("Tokens is not Matcher or Matchers in OrTokens.");
                    }
                },
                None => {
                    panic!("Could not find Tokens in OrTokens.");
                }
            },
            "CaptureString" => Self::Matcher(capture_string(
                extract_string_data(
                    parser.get_data("NonTerminal".to_string()),
                    "NonTerminal",
                    name.as_str(),
                ),
                extract_matcher_data(
                    parser.get_data("OrTokens".to_string()),
                    "OrTokens",
                    name.as_str(),
                ),
            )),
            // look in parent scope
            "Rule" => match parser.get_data_from_parent_scope("Rule".to_string()) {
                Some(matcher) => match matcher {
                    PegMatcher::Rule(r) => Self::Rules(vec![
                        r,
                        (
                            extract_string_data(
                                parser.get_data("NonTerminal".to_string()),
                                "NonTerminal",
                                name.as_str(),
                            ),
                            extract_matcher_data(
                                parser.get_data("OrTokens".to_string()),
                                "OrTokens",
                                name.as_str(),
                            ),
                        ),
                    ]),
                    PegMatcher::Rules(mut r) => {
                        r.push((
                            extract_string_data(
                                parser.get_data("NonTerminal".to_string()),
                                "NonTerminal",
                                name.as_str(),
                            ),
                            extract_matcher_data(
                                parser.get_data("OrTokens".to_string()),
                                "OrTokens",
                                name.as_str(),
                            ),
                        ));
                        Self::Rules(r)
                    }
                    _ => {
                        panic!("The last token is not a matcher.");
                    }
                },
                None => Self::Rules(vec![(
                    extract_string_data(
                        parser.get_data("NonTerminal".to_string()),
                        "NonTerminal",
                        "Rule",
                    ),
                    extract_matcher_data(
                        parser.get_data("OrTokens".to_string()),
                        "OrTokens",
                        name.as_str(),
                    ),
                )]),
            },
            "Rules" => Self::Rules(extract_rules_data(
                parser.get_data("Rule".to_string()),
                "Rule",
                name.as_str(),
            )),
            "Start" => Self::Rules(extract_rules_data(
                parser.get_data("Rules".to_string()),
                "Rules",
                name.as_str(),
            )),
            str => {
                let _ = writeln!(stderr(), "What is this token: {}.", str);
                Self::None
            }
        }
    }
    fn is_null(&self) -> bool {
        if let Self::None = self {
            true
        } else {
            false
        }
    }
}
