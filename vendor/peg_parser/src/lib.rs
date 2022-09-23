mod combinator;
mod extracter;
mod peg_matcher;
mod peg_rules;
mod test_combinator;
pub mod test_peg_parser;

use std::collections::HashMap;
use std::u32;

use combinator::parse_ref;
use peg_matcher::PegMatcher;
use peg_rules::init_peg_parser;

use crate::combinator::Matcher;

pub struct PegParser<T: Clone + ParserData + 'static> {
    peg_parser: Parser<PegMatcher<T>>,
}

impl<T: Clone + ParserData + 'static> PegParser<T> {
    pub fn new() -> Self {
        PegParser {
            peg_parser: init_peg_parser::<PegMatcher<T>>(),
        }
    }
    pub fn parse_parser(&mut self, parser_rules: &str) -> Result<Parser<T>, &str> {
        let rules = match self.peg_parser.parse(&parser_rules) {
            Ok(rules) => match rules {
                PegMatcher::Rules(a) => a,
                _ => {
                    panic!("Parse failed.");
                }
            },
            Err(str) => {
                return Err(str);
            }
        };
        let mut output_parser = Parser::new();
        for rule in rules {
            output_parser.add_rule(rule.0.clone(), rule.1.clone());
        }
        Ok(output_parser)
    }
}

pub trait ParserData: Sized + Clone {
    fn string(str: String) -> Self;
    fn null() -> Self;
    fn data(name: String, parser: &mut Parser<Self>) -> Self;
    fn is_null(&self) -> bool;
}

#[derive(Clone)]
pub struct Parser<T: Clone + ParserData> {
    pub grammar_list: HashMap<String, Matcher<T>>,
    // pub data: HashMap<String, T>,
    pub data: Vec<HashMap<String, T>>,
    pub pos: u32,
    pub error_pos: u32,
    pub input: String,
}

impl<T: Clone + ParserData + 'static> Parser<T> {
    pub fn new() -> Parser<T> {
        Parser {
            grammar_list: HashMap::new(),
            // data: HashMap::new(),
            data: Vec::new(),
            pos: 0,
            error_pos: 0,
            input: "".to_string(),
        }
    }
    pub fn add_rule(&mut self, name: String, rule: Matcher<T>) {
        self.grammar_list.insert(name, rule);
    }
    pub fn enter_scope(&mut self) {
        self.data.push(HashMap::new());
    }
    pub fn exit_scope(&mut self) {
        self.data.pop();
    }
    pub fn add_data(&mut self, name: String, data: T) {
        if !data.is_null() {
            // self.data
            //     .insert(self.scopes.last().unwrap().clone() + ":" + &name, data);
            let len = self.data.len();
            if len >= 1 {
                self.data[len - 1].insert(name, data);
            } else {
                panic!("Parser Stack does not exist.");
            }
        }
    }
    // filter is_null
    pub fn get_data(&mut self, name: String) -> Option<T> {
        // println!("{}", size_of::<HashMap<&str, T>>());
        // println!(
        //     "{:?}, {}",
        //     self.data.last().unwrap().keys(),
        //     self.data.len()
        // );
        match self.data.last() {
            Some(map) => {
                return match map.get(&name) {
                    Some(data) => Some(data.clone()),
                    None => None,
                };
            }
            None => {
                panic!("Parser stack does not exist.");
            }
        }
    }
    pub fn get_data_from_parent_scope(&mut self, name: String) -> Option<T> {
        // println!("{}", size_of::<HashMap<&str, T>>());
        // println!("{:?}", self.data.keys());
        let val = if self.data.len() >= 2 {
            match self.data[self.data.len() - 2].get(&name) {
                Some(data) => Some(data.clone()),
                None => None,
            }
        } else {
            None
        };
        val
    }
    pub fn eat(&mut self, str: &str) {
        // println!("eaten {}", str);
        self.pos += str.chars().count() as u32;
        for _ in 0..str.chars().count() {
            self.input.remove(0);
        }
        // println!("Remaining::: {}", self.input);
    }
    pub fn parse(&mut self, string: &str) -> Result<T, &str> {
        self.input = string.clone().to_string();
        self.data.push(HashMap::new());
        match parse_ref("Start".to_string(), None)(self) {
            Err(()) => {
                println!("Could not parse: {}", self.input);
                return Err("Start Parse failed.");
            }
            _ => {}
        }
        if self.data.len() == 1 {
            match self.data[0].get("Start") {
                Some(data) => {
                    return Ok(data.clone());
                }
                None => {
                    return Err("Parse failed: Could not get Start item.");
                }
            }
        } else {
            return Err("Parse failed: Stack does not exist.");
        }
    }
}
