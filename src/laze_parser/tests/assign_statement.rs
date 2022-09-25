use std::path::Path;

use crate::laze_parser::parser::LazeParser;

#[test]
fn all() {
    let mut test_parser = LazeParser::new(Path::new("./parser_files/ja.peg"));
    let ast = test_parser.parse(Path::new("./laze_tests/stm/assign_stm/assign_all.laze"));
    let mut ast_string = String::new();
    let _ = std::fmt::write(&mut ast_string, format_args!("{:?}", ast));
    assert_eq!(
        ast_string,
        r##"DecList([Dec_ { pos: 92, data: Func("実行", [], [], Stm_ { pos: 92, data: Compound([Stm_ { pos: 36, data: Dec(Dec_ { pos: 36, data: Var(Var_ { pos: 27, data: Simple("a") }, Type_ { pos: 23, data: Name("整数") }, Exp_ { pos: 30, data: String("0") }) }) }, Stm_ { pos: 47, data: Assign(Var_ { pos: 38, data: Simple("a") }, Exp_ { pos: 41, data: String("1") }, Normal) }, Stm_ { pos: 59, data: Assign(Var_ { pos: 49, data: Simple("a") }, Exp_ { pos: 53, data: String("1") }, Add) }, Stm_ { pos: 71, data: Assign(Var_ { pos: 61, data: Simple("a") }, Exp_ { pos: 65, data: String("1") }, Sub) }, Stm_ { pos: 83, data: Assign(Var_ { pos: 73, data: Simple("a") }, Exp_ { pos: 77, data: String("1") }, Mul) }, Stm_ { pos: 91, data: Assign(Var_ { pos: 85, data: Simple("a") }, Exp_ { pos: 89, data: String("1") }, Div) }]) }) }])"##
    );
}
