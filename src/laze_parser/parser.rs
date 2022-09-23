use std::{
    io::{stderr, Write},
    path::Path,
    process::exit,
};

use peg_parser::Parser;

use crate::{ast::ast::ASTNode, util::file_opener::open_file};

use super::init::init_laze_parser;

pub struct LazeParser {
    parser: Parser<ASTNode>,
}

impl LazeParser {
    pub fn new(parser_file_path: &Path) -> Self {
        Self {
            parser: init_laze_parser(parser_file_path),
        }
    }
    pub fn parse(&mut self, program_path: &Path) -> ASTNode {
        let content = open_file(program_path);
        match self.parser.parse(&content) {
            Ok(node) => node,
            Err(mes) => {
                let _ = writeln!(stderr(), "{mes}");
                exit(1);
            }
        }
    }
}
