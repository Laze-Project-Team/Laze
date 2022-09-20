mod combinator;
mod extracter;
mod parser;
mod peg_matcher;
mod test_combinator;
mod test_peg_parser;

use std::collections::HashMap;
use std::io::{stderr, Write};
use std::u32;

use parser::init_peg_parser;

use crate::combinator::Matcher;

pub struct PegParser<T: Clone + ParserData + 'static> {
    parser: Parser<T>,
}

impl<T: Clone + ParserData + 'static> PegParser<T> {
    pub fn new() -> Self {
        PegParser {
            parser: init_peg_parser(),
        }
    }
    // pub fn parse() -> Result<T, String> {

    // }
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

impl<T: Clone + ParserData> Parser<T> {
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
    pub fn get_data(&mut self, name: String) -> Option<T> {
        // println!("{}", size_of::<HashMap<&str, T>>());
        println!(
            "{:?}, {}",
            self.data.last().unwrap().keys(),
            self.data.len()
        );
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
        println!("eaten {}", str);
        self.pos += str.chars().count() as u32;
        for _ in 0..str.chars().count() {
            self.input.remove(0);
        }
    }
    pub fn parse(&mut self, string: &str) -> Result<String, ()> {
        self.input = string.clone().to_string();
        self.data.push(HashMap::new());
        let grammar_list = self.grammar_list.clone();
        match grammar_list.get("Start") {
            Some(matcher) => match matcher(self) {
                Ok(str) => {
                    println!("{:?}", self.data.last().unwrap().keys());
                    Ok(str)
                }
                Err(()) => Err(()),
            },
            None => {
                let _ = writeln!(stderr(), "Rule Start does not exist in the grammar.");
                Err(())
            }
        }
    }
}
