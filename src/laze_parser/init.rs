use std::{
    fs::File,
    io::{stderr, Read, Write},
};

use peg_parser::{Parser, PegParser};

use crate::ast::ast::ASTNode;

pub fn init_laze_parser(parser_file_path: &str) -> Parser<ASTNode> {
    let parser_rules = match File::open(parser_file_path) {
        Ok(mut file) => {
            let mut file_content = String::new();
            let _ = file
                .read_to_string(&mut file_content)
                .expect("Reading file");
            file_content
        }
        Err(error) => {
            let _ = writeln!(stderr(), "Could not open file: {}", parser_file_path);
            panic!("{:?}", error);
        }
    };
    let mut laze_parser = PegParser::<ASTNode>::new();
    laze_parser
        .parse_parser(&parser_rules)
        .expect("Parsing parser: ")
}
