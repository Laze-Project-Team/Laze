use std::path::Path;

use crate::laze_parser::parser::LazeParser;

#[test]
fn simple() {
    let mut test_parser = LazeParser::new(Path::new("./parser_files/ja.peg"));
    let ast = test_parser.parse(Path::new("./laze_tests/stm/while_stm/while_simple.laze"));
    let mut ast_string = String::new();
    let _ = std::fmt::write(&mut ast_string, format_args!("{:?}", ast));
    assert_eq!(
        ast_string,
        r##"DecList([Dec_ { pos: 72, data: Func("実行", [], [], Stm_ { pos: 72, data: Compound([Stm_ { pos: 36, data: Dec(Dec_ { pos: 36, data: Var(Var_ { pos: 27, data: Simple("a") }, Type_ { pos: 23, data: Name("整数") }, Exp_ { pos: 30, data: String("0") }) }) }, Stm_ { pos: 71, data: While(Exp_ { pos: 42, data: BinOp([Lt], [Exp_ { pos: 39, data: Var("a") }, Exp_ { pos: 42, data: String("5") }]) }, Stm_ { pos: 71, data: Compound([Stm_ { pos: 69, data: Assign(Var_ { pos: 59, data: Simple("a") }, Exp_ { pos: 63, data: String("1") }, Add) }]) }) }]) }) }])"##
    );
}
