#[allow(unused_imports)]
use std::io::{stderr, Write};

#[allow(unused_imports)]
use super::{parser::init_peg_parser, Parser};
#[allow(unused_imports)]
use crate::combinator::parse_or;
#[allow(unused_imports)]
use crate::peg_matcher::PegMatcher;
#[allow(unused_imports)]
use crate::ParserData;
#[allow(dead_code)]
const PEG: &str = r#"GreetWord = {"Hi" / "Hello" / "Goodbye": word}
ID = {[㐀-龯ぁ-んァ-ヶa-zA-Z_ー]+: id}
Greeting = GreetWord " "+ ID "!" / GreetWord " "+ ID " "+ "and" " "+ ID "!"
Greetings = Greeting ( "\n" Greeting )*
Start = Greetings"#;

// need to make capture string function

#[allow(dead_code)]
const GREETING: &str = "Hi 成田 and 成田!
Hello 永田!
Goodbye 永田!";

#[test]
fn test_peg_parser() {
    #[derive(Clone)]
    enum GreetingData {
        StringData(String),
        Greeting((String, String)),
        Greetings(Vec<(String, String)>),
        None,
    }
    impl GreetingData {
        fn get_string_data(&self) -> String {
            if let Self::StringData(str) = self {
                str.clone()
            } else {
                "".to_string()
            }
        }
    }
    impl ParserData for GreetingData {
        fn string(str: String) -> Self {
            Self::StringData(str)
        }
        fn null() -> Self {
            Self::None
        }
        fn data(name: String, parser: &mut Parser<GreetingData>) -> Self {
            fn extract_string_data(data: Option<GreetingData>, name: &str, rule: &str) -> String {
                match data {
                    Some(data) => data.get_string_data(),
                    None => {
                        let _ = writeln!(
                            stderr(),
                            "Could not find \"{}\" in the grammar to reduce \"{}\"",
                            name,
                            rule
                        );
                        "".to_string()
                    }
                }
            }
            println!("Reducing: {}", name.as_str());
            match name.as_str() {
                "GreetWord" => Self::StringData(extract_string_data(
                    parser.get_data("word".to_string()),
                    "word",
                    "GreetWord",
                )),
                "ID" => Self::StringData(extract_string_data(
                    parser.get_data("id".to_string()),
                    "id",
                    "ID",
                )),
                "Greeting" => match parser.get_data_from_parent_scope("Greeting".to_string()) {
                    Some(data) => match data {
                        Self::Greeting(greeting) => Self::Greetings(vec![
                            greeting,
                            (
                                extract_string_data(
                                    parser.get_data("ID".to_string()),
                                    "ID",
                                    "Greeting",
                                ),
                                extract_string_data(
                                    parser.get_data("GreetWord".to_string()),
                                    "GreetWord",
                                    "Greeting",
                                ),
                            ),
                        ]),
                        Self::Greetings(mut greetings) => {
                            greetings.push((
                                extract_string_data(
                                    parser.get_data("ID".to_string()),
                                    "ID",
                                    "Greeting",
                                ),
                                extract_string_data(
                                    parser.get_data("GreetWord".to_string()),
                                    "GreetWord",
                                    "Greeting",
                                ),
                            ));
                            Self::Greetings(greetings)
                        }
                        _ => {
                            let _ = writeln!(
                                stderr(),
                                "Greeting does not have type Greeting or Greetings."
                            );
                            Self::None
                        }
                    },
                    None => Self::Greeting((
                        extract_string_data(parser.get_data("ID".to_string()), "ID", "Greeting"),
                        extract_string_data(
                            parser.get_data("GreetWord".to_string()),
                            "GreetWord",
                            "Greeting",
                        ),
                    )),
                },
                "Greetings" => match parser.get_data("Greeting".to_string()) {
                    Some(greeting) => match greeting {
                        Self::Greeting(data) => Self::Greetings(vec![data]),
                        Self::Greetings(data) => Self::Greetings(data),
                        _ => Self::Greetings(vec![]),
                    },
                    None => Self::Greetings(vec![]),
                },
                str => {
                    let _ = writeln!(stderr(), "What is this token: {}", str);
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
    let mut peg_parser = init_peg_parser::<PegMatcher<GreetingData>>();
    let rules = match peg_parser.parse(PEG) {
        Ok(str) => match peg_parser.data.last() {
            Some(r) => match &r["Rules"] {
                PegMatcher::Rules(a) => a,
                _ => {
                    panic!("Parse failed.");
                }
            },
            None => {
                panic!("Parse failed.");
            }
        },
        Err(()) => {
            panic!("Parse failed at position {}.", peg_parser.pos);
        }
    };
    let mut test_parser: Parser<GreetingData> = Parser::new();
    for rule in rules {
        test_parser.add_rule(rule.0.clone(), rule.1.clone());
    }
    match test_parser.parse(GREETING) {
        Ok(str) => {
            assert_eq!(str, GREETING);
            println!("Parsed: \n{}", str);
        }
        Err(()) => {
            panic!("Parse failed at position {}.", test_parser.pos);
        }
    }
    match test_parser.get_data("Greetings".to_string()) {
        Some(greetings) => match greetings {
            GreetingData::Greetings(data) => {
                for s in &data {
                    println!("Name: {}, Greeting: {}", s.0, s.1);
                }
            }
            _ => {
                panic!("Greetings is not of right type -> Parse Failed.")
            }
        },
        None => panic!("Greetings could not be found. -> Parse Failed."),
    }
}
