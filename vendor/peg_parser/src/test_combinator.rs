#[allow(unused_imports)]
use super::combinator::*;
use super::*;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::{stderr, Write};

impl ParserData for () {
    fn string(_: String) -> Self {
        ()
    }
    fn null() -> Self {
        ()
    }
    fn data(_: String, _: &mut Parser<()>) -> Self {
        ()
    }
    fn is_null(&self) -> bool {
        true
    }
}

#[test]
fn test_parse_str() {
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule("Start".to_string(), parse_str("\u{3042}".to_string()));
        match test_parser.parse("あああ") {
            Ok(str) => {
                assert_eq!(str, "あ");
            }
            Err(()) => {
                panic!("Parse Failed.")
            }
        }
    }
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule("Start".to_string(), parse_str("aaa".to_string()));
        match test_parser.parse("aaa") {
            Ok(str) => {
                assert_eq!(str, "aaa");
            }
            Err(()) => {
                panic!("Parse Failed.")
            }
        }
    }
}

#[test]
fn test_parse_any() {
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule("Start".to_string(), parse_any());
        match test_parser.parse("あああ") {
            Ok(str) => {
                assert_eq!(str, "あ");
            }
            Err(()) => {
                panic!("Parse Failed.")
            }
        }
    }
}

#[test]
fn test_parse_any_should_fail() {
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule("Start".to_string(), parse_any());
        match test_parser.parse("") {
            Ok(_) => {
                panic!("unexpected parse successful");
            }
            Err(()) => {
                assert_eq!(1, 1);
            }
        }
    }
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule("Start".to_string(), parse_any());
        match test_parser.parse("\n") {
            Ok(_) => {
                panic!("unexpected parse successful");
            }
            Err(()) => {
                assert_eq!(1, 1);
            }
        }
    }
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule("Start".to_string(), parse_str("aaa".to_string()));
        match test_parser.parse("b") {
            Ok(_) => {
                panic!("unexpected parse successful");
            }
            Err(()) => {
                assert_eq!(1, 1);
            }
        }
    }
}

#[test]
fn test_parse_range() {
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule("Start".to_string(), parse_range("a-c".to_string()));
        match test_parser.parse("c") {
            Ok(str) => {
                assert_eq!(str, "c");
            }
            Err(()) => {
                panic!("Parse Failed.")
            }
        }
    }
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule("Start".to_string(), parse_range("ab-c".to_string()));
        match test_parser.parse("b") {
            Ok(str) => {
                assert_eq!(str, "b");
            }
            Err(()) => {
                panic!("Parse Failed.")
            }
        }
    }
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule("Start".to_string(), parse_range("ab-cde-f".to_string()));
        match test_parser.parse("d") {
            Ok(str) => {
                assert_eq!(str, "d");
            }
            Err(()) => {
                panic!("Parse Failed.")
            }
        }
    }
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule("Start".to_string(), parse_range(r"\--/".to_string()));
        match test_parser.parse(".") {
            Ok(str) => {
                assert_eq!(str, ".");
            }
            Err(()) => {
                panic!("Parse Failed.")
            }
        }
    }
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule("Start".to_string(), parse_range("a-zあ-ん".to_string()));
        match test_parser.parse("か") {
            Ok(str) => {
                assert_eq!(str, "か");
            }
            Err(()) => {
                panic!("Parse Failed.")
            }
        }
    }
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule("Start".to_string(), parse_range("㐀-龯".to_string()));
        match test_parser.parse("成田") {
            Ok(str) => {
                assert_eq!(str, "成");
            }
            Err(()) => {
                panic!("Parse Failed.")
            }
        }
    }
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule(
            "Start".to_string(),
            parse_range("㐀-龯ぁ-んァ-ヶa-zA-Z_ー".to_string()),
        );
        match test_parser.parse("_成田") {
            Ok(str) => {
                assert_eq!(str, "_");
            }
            Err(()) => {
                panic!("Parse Failed.")
            }
        }
    }
}

#[test]
fn test_parse_range_should_fail() {
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule("Start".to_string(), parse_range("a-".to_string()));
        match test_parser.parse("a") {
            Ok(_) => {
                panic!("unexpected parse successful");
            }
            Err(()) => {
                assert_eq!(1, 1);
            }
        }
    }
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule("Start".to_string(), parse_range("あいう".to_string()));
        match test_parser.parse("") {
            Ok(_) => {
                panic!("unexpected parse successful");
            }
            Err(()) => {
                assert_eq!(1, 1);
            }
        }
    }
}

#[test]
fn test_parse_many() {
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule(
            "Start".to_string(),
            parse_many(parse_range("㐀-龯ぁ-んァ-ヶa-zA-Z_ー".to_string())),
        );
        match test_parser.parse("_") {
            Ok(str) => {
                assert_eq!(str, "_");
            }
            Err(()) => {
                panic!("Parse Failed.")
            }
        }
    }
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule(
            "Start".to_string(),
            parse_many(parse_range("㐀-龯ぁ-んァ-ヶa-zA-Z_ー".to_string())),
        );
        match test_parser.parse("") {
            Ok(str) => {
                assert_eq!(str, "");
            }
            Err(()) => {
                panic!("Parse Failed.")
            }
        }
    }
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule(
            "Start".to_string(),
            parse_many(parse_range(
                "㐀-龯ぁ-んァ-ヶａ-ｚＡ-Ｚa-zA-Z_ー".to_string(),
            )),
        );
        match test_parser.parse("成田fdsfsfdojiｌｋじょい") {
            Ok(str) => {
                assert_eq!(str, "成田fdsfsfdojiｌｋじょい");
            }
            Err(()) => {
                panic!("Parse Failed.")
            }
        }
    }
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule(
            "Start".to_string(),
            parse_many(parse_range("0-9".to_string())),
        );
        match test_parser.parse("456789") {
            Ok(str) => {
                assert_eq!(str, "456789");
            }
            Err(()) => {
                panic!("Parse Failed.")
            }
        }
    }
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule(
            "Start".to_string(),
            parse_many(parse_range("㐀-龯ぁ-んァ-ヶa-zA-Z_ー".to_string())),
        );
        match test_parser.parse("hello world") {
            Ok(str) => {
                assert_eq!(str, "hello");
            }
            Err(()) => {
                panic!("Parse Failed.")
            }
        }
    }
}

#[test]
fn test_parse_many_should_fail() {
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule("Start".to_string(), parse_range("0-9".to_string()));
        match test_parser.parse("abcd") {
            Ok(_) => {
                panic!("unexpected parse successful");
            }
            Err(()) => {
                assert_eq!(1, 1);
            }
        }
    }
}

#[test]
fn test_parse_more_than_one() {
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule(
            "Start".to_string(),
            parse_more_than_one(parse_range("0-9".to_string())),
        );
        match test_parser.parse("1234567") {
            Ok(str) => {
                assert_eq!(str, "1234567");
            }
            Err(()) => {
                panic!("Parse Failed.")
            }
        }
    }
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule(
            "Start".to_string(),
            parse_more_than_one(parse_str(" ".to_string())),
        );
        match test_parser.parse("     ") {
            Ok(str) => {
                assert_eq!(str, "     ");
            }
            Err(()) => {
                panic!("Parse Failed.")
            }
        }
    }
}

#[test]
fn test_parse_more_than_one_should_fail() {
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule(
            "Start".to_string(),
            parse_more_than_one(parse_range("0-9".to_string())),
        );
        match test_parser.parse("") {
            Ok(_) => {
                panic!("unexpected parse successful");
            }
            Err(()) => {
                assert_eq!(1, 1);
            }
        }
    }
}

#[test]
fn test_parse_not() {
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule("Start".to_string(), parse_not(parse_str("a".to_string())));
        match test_parser.parse("bbb") {
            Ok(str) => {
                assert_eq!(str, "");
            }
            Err(()) => {
                panic!("Parse Failed.")
            }
        }
    }
}

#[test]
fn test_parse_not_should_fail() {
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule("Start".to_string(), parse_not(parse_str("a".to_string())));
        match test_parser.parse("abb") {
            Ok(_) => {
                panic!("unexpected parse successful")
            }
            Err(()) => {
                assert_eq!(1, 1);
            }
        }
    }
}

#[test]
fn test_parse_seq() {
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule(
            "Start".to_string(),
            parse_seq(vec![
                parse_str("hello".to_string()),
                parse_many(parse_str(" ".to_string())),
                parse_str("world".to_string()),
            ]),
        );
        match test_parser.parse("hello world") {
            Ok(str) => {
                assert_eq!(str, "hello world");
            }
            Err(()) => {
                panic!("Parse Failed.")
            }
        }
    }
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule(
            "Start".to_string(),
            parse_seq(vec![
                parse_str("hello".to_string()),
                parse_many(parse_str(" ".to_string())),
                parse_many(parse_range("㐀-龯ぁ-んァ-ヶa-zA-Z_ー".to_string())),
                parse_or(vec![
                    parse_many(parse_str(" ".to_string())),
                    parse_str("".to_string()),
                ]),
                parse_str("!".to_string()),
            ]),
        );
        match test_parser.parse("hello 永田!") {
            Ok(str) => {
                assert_eq!(str, "hello 永田!");
            }
            Err(()) => {
                panic!("Parse Failed.")
            }
        }
    }
}

#[test]
fn test_parse_or() {
    {
        let mut test_parser = Parser::<()>::new();
        test_parser.add_rule(
            "Start".to_string(),
            parse_or(vec![
                parse_str("good bye".to_string()),
                parse_str("hello".to_string()),
                parse_str("good morning".to_string()),
            ]),
        );
        match test_parser.parse("good morning world") {
            Ok(str) => {
                assert_eq!(str, "good morning");
            }
            Err(()) => {
                panic!("Parse Failed.")
            }
        }
    }
}

#[test]
fn test_combinators() {
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
                            "Could not find \"{}\" in the grammar to parse \"{}\"",
                            name,
                            rule
                        );
                        "".to_string()
                    }
                }
            }
            match name.as_str() {
                "Greeting" => match parser.get_data("Greeting".to_string()) {
                    Some(data) => match data {
                        Self::Greeting(greeting) => Self::Greetings(vec![
                            greeting,
                            (
                                extract_string_data(
                                    parser.get_data("name".to_string()),
                                    "name",
                                    "Greeting",
                                ),
                                extract_string_data(
                                    parser.get_data("greetword".to_string()),
                                    "greetword",
                                    "Greeting",
                                ),
                            ),
                        ]),
                        Self::Greetings(mut greetings) => {
                            greetings.push((
                                extract_string_data(
                                    parser.get_data("name".to_string()),
                                    "name",
                                    "Greeting",
                                ),
                                extract_string_data(
                                    parser.get_data("greetword".to_string()),
                                    "greetword",
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
                        extract_string_data(
                            parser.get_data("name".to_string()),
                            "name",
                            "Greeting",
                        ),
                        extract_string_data(
                            parser.get_data("greetword".to_string()),
                            "greetword",
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
                _ => Self::None,
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
    let mut test_parser = Parser::<GreetingData>::new();
    test_parser.add_rule(
        "ID".to_string(),
        parse_more_than_one(parse_range("㐀-龯ぁ-んァ-ヶa-zA-Z_ー".to_string())),
    );
    test_parser.add_rule(
        "GreetWord".to_string(),
        parse_or(vec![
            parse_str("Hi".to_string()),
            parse_str("Hello".to_string()),
            parse_str("Good morning".to_string()),
        ]),
    );
    test_parser.add_rule(
        "Greeting".to_string(),
        parse_seq(vec![
            capture_string(
                "greetword".to_string(),
                parse_ref("GreetWord".to_string(), None),
            ),
            parse_more_than_one(parse_str(" ".to_string())),
            capture_string("name".to_string(), parse_ref("ID".to_string(), None)),
            parse_many(parse_str(" ".to_string())),
            parse_str("!".to_string()),
        ]),
    );
    test_parser.add_rule(
        "Greetings".to_string(),
        parse_more_than_one(parse_seq(vec![
            parse_ref("Greeting".to_string(), None),
            parse_str("\n".to_string()),
        ])),
    );
    test_parser.add_rule(
        "Start".to_string(),
        parse_ref("Greetings".to_string(), None),
    );
    match test_parser.parse("Hi 永田!\nGood morning 成田!\n") {
        Ok(str) => {
            assert_eq!(str, "Hi 永田!\nGood morning 成田!\n");
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
            // assert_eq!(test_parser.get_data("Greeting").get_greeting_data().1, "Hi");
        }
        Err(()) => {
            panic!("Parse Failed at {}", test_parser.pos);
        }
    }
}
