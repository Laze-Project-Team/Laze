use super::combinator::*;
use super::parser::*;

#[test]
fn test_parse_str<'a>() {
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule("Start", parse_str("\u{3042}"));
        match test_parser.parse("あああ") {
            Some(parser) => {
                assert_eq!(parser.input, "ああ");
            }
            None => {
                panic!("Parse Failed.")
            }
        }
    }
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule("Start", parse_str("aaa"));
        match test_parser.parse("aaa") {
            Some(parser) => {
                assert_eq!(parser.input, "");
            }
            None => {
                panic!("Parse Failed.")
            }
        }
    }
}

#[test]
fn test_parse_any<'a>() {
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule("Start", parse_any());
        match test_parser.parse("あああ") {
            Some(parser) => {
                assert_eq!(parser.input, "ああ");
            }
            None => {
                panic!("Parse Failed.")
            }
        }
    }
}

#[test]
fn test_parse_any_should_fail<'a>() {
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule("Start", parse_any());
        match test_parser.parse("") {
            Some(_) => {
                panic!("unexpected parse successful");
            }
            None => {
                assert_eq!(1, 1);
            }
        }
    }
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule("Start", parse_str("aaa"));
        match test_parser.parse("b") {
            Some(_) => {
                panic!("unexpected parse successful");
            }
            None => {
                assert_eq!(1, 1);
            }
        }
    }
}

#[test]
fn test_parse_range<'a>() {
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule("Start", parse_range("a-c"));
        match test_parser.parse("c") {
            Some(parser) => {
                assert_eq!(parser.input, "");
            }
            None => {
                panic!("Parse Failed.")
            }
        }
    }
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule("Start", parse_range("ab-c"));
        match test_parser.parse("b") {
            Some(parser) => {
                assert_eq!(parser.input, "");
            }
            None => {
                panic!("Parse Failed.")
            }
        }
    }
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule("Start", parse_range("ab-cde-f"));
        match test_parser.parse("d") {
            Some(parser) => {
                assert_eq!(parser.input, "");
            }
            None => {
                panic!("Parse Failed.")
            }
        }
    }
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule("Start", parse_range(r"\--/"));
        match test_parser.parse(".") {
            Some(parser) => {
                assert_eq!(parser.input, "");
            }
            None => {
                panic!("Parse Failed.")
            }
        }
    }
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule("Start", parse_range("a-zあ-ん"));
        match test_parser.parse("か") {
            Some(parser) => {
                assert_eq!(parser.input, "");
            }
            None => {
                panic!("Parse Failed.")
            }
        }
    }
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule("Start", parse_range("㐀-龯"));
        match test_parser.parse("成田") {
            Some(parser) => {
                assert_eq!(parser.input, "田");
            }
            None => {
                panic!("Parse Failed.")
            }
        }
    }
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule("Start", parse_range("㐀-龯ぁ-んァ-ヶa-zA-Z_ー"));
        match test_parser.parse("_成田") {
            Some(parser) => {
                assert_eq!(parser.input, "成田");
            }
            None => {
                panic!("Parse Failed.")
            }
        }
    }
}

#[test]
fn test_parse_range_should_fail<'a>() {
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule("Start", parse_range("a-"));
        match test_parser.parse("a") {
            Some(_) => {
                panic!("unexpected parse successful");
            }
            None => {
                assert_eq!(1, 1);
            }
        }
    }
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule("Start", parse_range("あいう"));
        match test_parser.parse("") {
            Some(_) => {
                panic!("unexpected parse successful");
            }
            None => {
                assert_eq!(1, 1);
            }
        }
    }
}

#[test]
fn test_parse_many<'a>() {
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule("Start", parse_many(parse_range("㐀-龯ぁ-んァ-ヶa-zA-Z_ー")));
        match test_parser.parse("_成田") {
            Some(parser) => {
                assert_eq!(parser.input, "");
            }
            None => {
                panic!("Parse Failed.")
            }
        }
    }
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule(
            "Start",
            parse_many(parse_range("㐀-龯ぁ-んァ-ヶａ-ｚＡ-Ｚa-zA-Z_ー")),
        );
        match test_parser.parse("成田fdsfsfdojiｌｋじょい") {
            Some(parser) => {
                assert_eq!(parser.input, "");
            }
            None => {
                panic!("Parse Failed.")
            }
        }
    }
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule("Start", parse_many(parse_range("0-9")));
        match test_parser.parse("456789") {
            Some(parser) => {
                assert_eq!(parser.input, "");
            }
            None => {
                panic!("Parse Failed.")
            }
        }
    }
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule("Start", parse_many(parse_range("㐀-龯ぁ-んァ-ヶa-zA-Z_ー")));
        match test_parser.parse("hello world") {
            Some(parser) => {
                assert_eq!(parser.input, " world");
            }
            None => {
                panic!("Parse Failed.")
            }
        }
    }
}

#[test]
fn test_parse_many_should_fail<'a>() {
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule("Start", parse_range("0-9"));
        match test_parser.parse("abcd") {
            Some(_) => {
                panic!("unexpected parse successful");
            }
            None => {
                assert_eq!(1, 1);
            }
        }
    }
}

#[test]
fn test_parse_more_than_one<'a>() {
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule("Start", parse_more_than_one(parse_range("0-9")));
        match test_parser.parse("1234567") {
            Some(parser) => {
                assert_eq!(parser.input, "");
            }
            None => {
                panic!("Parse Failed.")
            }
        }
    }
}

#[test]
fn test_parse_more_than_one_should_fail<'a>() {
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule("Start", parse_more_than_one(parse_range("0-9")));
        match test_parser.parse("") {
            Some(_) => {
                panic!("unexpected parse successful");
            }
            None => {
                assert_eq!(1, 1);
            }
        }
    }
}

#[test]
fn test_parse_not<'a>() {
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule("Start", parse_not(parse_str("a")));
        match test_parser.parse("bbb") {
            Some(parser) => {
                assert_eq!(parser.input, "bbb");
            }
            None => {
                panic!("Parse Failed.")
            }
        }
    }
}

#[test]
fn test_parse_not_should_fail<'a>() {
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule("Start", parse_not(parse_str("a")));
        match test_parser.parse("abb") {
            Some(_) => {
                panic!("unexpected parse successful")
            }
            None => {
                assert_eq!(1, 1);
            }
        }
    }
}

#[test]
fn test_parse_seq<'a>() {
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule(
            "Start",
            parse_seq(vec![
                parse_str("hello"),
                parse_many(parse_str(" ")),
                parse_str("world"),
            ]),
        );
        match test_parser.parse("hello world") {
            Some(parser) => {
                assert_eq!(parser.input, "");
            }
            None => {
                panic!("Parse Failed.")
            }
        }
    }
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule(
            "Start",
            parse_seq(vec![
                parse_str("hello"),
                parse_many(parse_str(" ")),
                parse_many(parse_range("㐀-龯ぁ-んァ-ヶa-zA-Z_ー")),
                parse_or(vec![parse_many(parse_str(" ")), parse_str("")]),
                parse_str("!"),
            ]),
        );
        match test_parser.parse("hello 永田!") {
            Some(parser) => {
                assert_eq!(parser.input, "");
            }
            None => {
                panic!("Parse Failed.")
            }
        }
    }
}

#[test]
fn test_parse_or<'a>() {
    {
        let mut test_parser = Parser::<'a, ()>::new();
        test_parser.add_rule(
            "Start",
            parse_or(vec![
                parse_str("goodbye"),
                parse_str("hello"),
                parse_str("good morning"),
            ]),
        );
        match test_parser.parse("hello world") {
            Some(parser) => {
                assert_eq!(parser.input, " world");
            }
            None => {
                panic!("Parse Failed.")
            }
        }
    }
}

#[test]
fn test_combinators<'a>() {
    let mut test_parser = Parser::<'a, ()>::new();
    test_parser.add_rule(
        "ID",
        parse_more_than_one(parse_range("㐀-龯ぁ-んァ-ヶa-zA-Z_ー")),
    );
    test_parser.add_rule(
        "GreetWord",
        parse_or(vec![
            parse_str("Hi"),
            parse_str("Hello"),
            parse_str("Good morning"),
        ]),
    );
    test_parser.add_rule(
        "Start",
        parse_seq(vec![
            parse_ref(test_parser.grammar_list.clone(), "GreetWord"),
            parse_more_than_one(parse_str(" ")),
            parse_ref(test_parser.grammar_list.clone(), "ID"),
            parse_many(parse_str(" ")),
            parse_str("!"),
        ]),
    );
    match test_parser.parse("Hi 永田!") {
        Some(parser) => {
            assert_eq!(parser.input, "");
        }
        None => {
            panic!("Parse Failed.")
        }
    }
}
