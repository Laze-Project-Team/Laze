use std::fmt;
use std::path::Path;

use crate::laze_parser::parser::LazeParser;

#[test]
fn only_if() {
    let mut test_parser = LazeParser::new(Path::new("./parser_files/ja.peg"));
    let ast = test_parser.parse(Path::new("./laze_tests/if_statement/if_only_if.laze"));
    let mut ast_string = String::new();
    let _ = fmt::write(&mut ast_string, format_args!("{:?}", ast));
    // assert_eq!(ast_string, )
}
