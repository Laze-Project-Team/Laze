use super::combinator::Matcher;
use std::collections::HashMap;
use std::io::{stderr, Write};

#[derive(Clone)]
pub struct Parser<'a, T: Clone> {
    pub grammar_list: HashMap<&'a str, Matcher<'a, T>>,
    pub data: HashMap<&'a str, T>,
    pub pos: i32,
    pub error_pos: i32,
    pub input: String,
}

impl<'a, T: Clone> Parser<'a, T> {
    pub fn new() -> Parser<'a, T> {
        Parser {
            grammar_list: HashMap::new(),
            data: HashMap::new(),
            pos: 0,
            error_pos: 0,
            input: "".to_string(),
        }
    }
    pub fn add_rule(&mut self, name: &'a str, rule: Matcher<'a, T>) {
        self.grammar_list.insert(name, rule);
    }
    pub fn parse(mut self, string: &'a str) -> Option<Self> {
        self.input = string.clone().to_string();
        let grammar_list = self.grammar_list.clone();
        match grammar_list.get("Start") {
            Some(matcher) => matcher(self),
            None => {
                let _ = writeln!(stderr(), "Rule Start does not exist in the grammar.");
                None
            }
        }
    }
}
