use super::combinator::*;
use crate::ast::ast::ASTNode;
use std::{collections::HashMap, rc::Rc};

pub type LazeMatcher<'a> = Matcher<'a, Rc<dyn ASTNode>>;

pub fn init_peg_parser<'a>() -> HashMap<&'a str, Matcher<'a, LazeMatcher<'a>>> {
    let mut peg_table = HashMap::new();

    peg_table.insert(
        "StringContent",
        parse_many(parse_seq(vec![parse_not(parse_str("\"")), parse_any()])),
    );
    peg_table.insert(
        "String",
        parse_seq(vec![
            parse_str("\""),
            parse_ref(peg_table.clone(), "StringContent"),
            parse_str("\""),
        ]),
    );
    peg_table.insert(
        "RangeContent",
        parse_more_than_one(parse_seq(vec![
            parse_not(parse_or(vec![parse_str("["), parse_str("]")])),
            parse_any(),
        ])),
    );
    peg_table.insert(
        "Range",
        parse_seq(vec![
            parse_str("["),
            parse_ref(peg_table.clone(), "RangeContent"),
            parse_str("]"),
        ]),
    );
    peg_table.insert(
        "Token",
        parse_or(vec![
            parse_ref(peg_table.clone(), "String"),
            parse_ref(peg_table.clone(), "Range"),
            parse_more_than_one(parse_range("a-zA-Z")),
        ]),
    );
    peg_table.insert(
        "Tokens",
        parse_seq(vec![parse_more_than_one(parse_ref(
            peg_table.clone(),
            "Token",
        ))]),
    );
    peg_table.insert(
        "Rule",
        parse_seq(vec![
            parse_many(parse_str(" ")),
            parse_more_than_one(parse_range("a-zA-Z")),
            parse_many(parse_str(" ")),
            parse_str("="),
            parse_many(parse_str(" ")),
            parse_ref(peg_table.clone(), "Tokens"),
            parse_or(vec![parse_str("\r\n"), parse_str("\n")]),
        ]),
    );
    peg_table.insert(
        "Rules",
        parse_more_than_one(parse_ref(peg_table.clone(), "Rule")),
    );
    peg_table
}
