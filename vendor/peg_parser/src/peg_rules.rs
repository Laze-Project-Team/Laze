use crate::{Parser, ParserData};

use super::combinator::*;

pub fn init_peg_parser<'a, T: Clone + ParserData + 'static>() -> Parser<T> {
    let mut peg_parser = Parser::<T>::new();

    // need to handle escape sequences
    peg_parser.add_rule(
        "StringContent".to_string(),
        capture_string(
            "content".to_string(),
            parse_many(parse_or(vec![
                parse_seq(vec![
                    parse_not(parse_or(vec![
                        parse_str("\\".to_string()),
                        parse_str("\"".to_string()),
                    ])),
                    parse_any(),
                ]),
                parse_str("\\\"".to_string()),
                parse_str("\\n".to_string()),
                parse_str("\\\\".to_string()),
            ])),
        ),
    );
    peg_parser.add_rule(
        "String".to_string(),
        parse_seq(vec![
            parse_str('"'.to_string()),
            parse_ref("StringContent".to_string(), None),
            parse_str('"'.to_string()),
        ]),
    );
    peg_parser.add_rule(
        "RangeContent".to_string(),
        capture_string(
            "content".to_string(),
            parse_more_than_one(parse_seq(vec![
                parse_not(parse_or(vec![
                    parse_str("[".to_string()),
                    parse_str("]".to_string()),
                ])),
                parse_any(),
            ])),
        ),
    );
    peg_parser.add_rule(
        "Range".to_string(),
        parse_seq(vec![
            parse_str("[".to_string()),
            parse_ref("RangeContent".to_string(), None),
            parse_str("]".to_string()),
        ]),
    );
    peg_parser.add_rule(
        "NonTerminal".to_string(),
        capture_string(
            "name".to_string(),
            parse_more_than_one(parse_range("a-zA-Z0-9".to_string())),
        ),
    );
    peg_parser.add_rule(
        "NonTerminalToken".to_string(),
        parse_or(vec![
            parse_seq(vec![
                parse_ref("NonTerminal".to_string(), None),
                parse_str("::".to_string()),
                parse_ref("NonTerminal".to_string(), Some("Rename".to_string())),
            ]),
            parse_ref("NonTerminal".to_string(), None),
        ]),
    );
    peg_parser.add_rule("AnyToken".to_string(), parse_str(".".to_string()));
    peg_parser.add_rule(
        "RawToken".to_string(),
        parse_or(vec![
            parse_ref("AnyToken".to_string(), Some("tokendata".to_string())),
            parse_ref("ParenTokens".to_string(), Some("tokendata".to_string())),
            parse_ref("CaptureString".to_string(), Some("tokendata".to_string())),
            parse_ref(
                "NonTerminalToken".to_string(),
                Some("tokendata".to_string()),
            ),
            parse_ref("String".to_string(), Some("tokendata".to_string())),
            parse_ref("Range".to_string(), Some("tokendata".to_string())),
        ]),
    );
    peg_parser.add_rule(
        "ManyToken".to_string(),
        parse_seq(vec![
            parse_ref("RawToken".to_string(), None),
            parse_str("*".to_string()),
        ]),
    );
    peg_parser.add_rule(
        "MoreThanOneToken".to_string(),
        parse_seq(vec![
            parse_ref("RawToken".to_string(), None),
            parse_str("+".to_string()),
        ]),
    );
    peg_parser.add_rule(
        "NotToken".to_string(),
        parse_seq(vec![
            parse_str("!".to_string()),
            parse_ref("RawToken".to_string(), None),
        ]),
    );
    peg_parser.add_rule(
        "Token".to_string(),
        parse_or(vec![
            parse_ref("NotToken".to_string(), Some("tokendata".to_string())),
            parse_ref("ManyToken".to_string(), Some("tokendata".to_string())),
            parse_ref(
                "MoreThanOneToken".to_string(),
                Some("tokendata".to_string()),
            ),
            parse_ref("RawToken".to_string(), Some("tokendata".to_string())),
        ]),
    );
    peg_parser.add_rule(
        "Tokens".to_string(),
        parse_seq(vec![
            parse_ref("Token".to_string(), None),
            parse_many(parse_seq(vec![
                parse_more_than_one(parse_str(" ".to_string())),
                parse_ref("Token".to_string(), None),
            ])),
        ]),
    );
    peg_parser.add_rule(
        "ParenTokens".to_string(),
        parse_seq(vec![
            parse_str("(".to_string()),
            parse_many(parse_str(" ".to_string())),
            parse_ref("OrTokens".to_string(), None),
            parse_many(parse_str(" ".to_string())),
            parse_str(")".to_string()),
        ]),
    );
    peg_parser.add_rule(
        "OrTokens".to_string(),
        parse_seq(vec![
            parse_ref("Tokens".to_string(), None),
            parse_many(parse_seq(vec![
                parse_many(parse_str(" ".to_string())),
                parse_str("/".to_string()),
                parse_many(parse_str(" ".to_string())),
                parse_ref("Tokens".to_string(), None),
            ])),
        ]),
    );
    peg_parser.add_rule(
        "CaptureString".to_string(),
        parse_seq(vec![
            parse_str("{".to_string()),
            parse_many(parse_str(" ".to_string())),
            parse_ref("OrTokens".to_string(), None),
            parse_many(parse_str(" ".to_string())),
            parse_str(":".to_string()),
            parse_many(parse_str(" ".to_string())),
            parse_ref("NonTerminal".to_string(), None),
            parse_many(parse_str(" ".to_string())),
            parse_str("}".to_string()),
        ]),
    );
    peg_parser.add_rule(
        "Rule".to_string(),
        parse_seq(vec![
            parse_ref("NonTerminal".to_string(), None),
            parse_many(parse_str(" ".to_string())),
            parse_str("=".to_string()),
            parse_many(parse_str(" ".to_string())),
            parse_or(vec![
                parse_ref("OrTokens".to_string(), None),
                parse_ref("CaptureString".to_string(), Some("OrTokens".to_string())),
            ]),
        ]),
    );
    peg_parser.add_rule(
        "Rules".to_string(),
        parse_seq(vec![
            parse_ref("Rule".to_string(), None),
            parse_many(parse_seq(vec![
                parse_more_than_one(parse_or(vec![
                    parse_str("\r\n".to_string()),
                    parse_str("\n".to_string()),
                ])),
                parse_ref("Rule".to_string(), None),
            ])),
        ]),
    );
    peg_parser.add_rule("Start".to_string(), parse_ref("Rules".to_string(), None));
    peg_parser
}
