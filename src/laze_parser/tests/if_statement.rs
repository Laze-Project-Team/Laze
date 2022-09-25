use std::fmt;
use std::path::Path;

use crate::laze_parser::parser::LazeParser;

#[test]
fn only_if() {
    let mut test_parser = LazeParser::new(Path::new("./parser_files/ja.peg"));
    let ast = test_parser.parse(Path::new("./laze_tests/stm/if_stm/if_only_if.laze"));
    let mut ast_string = String::new();
    let _ = fmt::write(&mut ast_string, format_args!("{:?}", ast));
    assert_eq!(
        ast_string,
        r##"DecList([Dec_ { pos: 91, data: Func("実行", [], [], Stm_ { pos: 91, data: Compound([Stm_ { pos: 35, data: Dec(Dec_ { pos: 35, data: Var(Var_ { pos: 26, data: Simple("a") }, Type_ { pos: 22, data: Name("整数") }, Exp_ { pos: 29, data: String("4") }) }) }, Stm_ { pos: 90, data: IfElse([IfElse_ { pos: 90, data: If(Exp_ { pos: 44, data: BinOp([Eq], [Exp_ { pos: 40, data: Var("a") }, Exp_ { pos: 44, data: String("5") }]) }, Stm_ { pos: 90, data: Compound([Stm_ { pos: 88, data: Assign(Var_ { pos: 60, data: Simple("a") }, Exp_ { pos: 82, data: Suffix(Exp_ { pos: 64, data: Var("表示") }, [ExpSuffix_ { pos: 82, data: Call([Exp_ { pos: 72, data: String("こんにちは") }, Exp_ { pos: 81, data: String("こんばんは") }]) }]) }, Normal) }]) }) }]) }]) }) }])"##
    );
}

#[test]
fn elseif_else() {
    let mut test_parser = LazeParser::new(Path::new("./parser_files/ja.peg"));
    let ast = test_parser.parse(Path::new("./laze_tests/stm/if_stm/if_elseif_else.laze"));
    let mut ast_string = String::new();
    let _ = fmt::write(&mut ast_string, format_args!("{:?}", ast));
    assert_eq!(
        ast_string,
        r##"DecList([Dec_ { pos: 223, data: Func("実行", [], [], Stm_ { pos: 223, data: Compound([Stm_ { pos: 35, data: Dec(Dec_ { pos: 35, data: Var(Var_ { pos: 26, data: Simple("a") }, Type_ { pos: 22, data: Name("整数") }, Exp_ { pos: 29, data: String("4") }) }) }, Stm_ { pos: 222, data: IfElse([IfElse_ { pos: 81, data: If(Exp_ { pos: 44, data: BinOp([Eq], [Exp_ { pos: 40, data: Var("a") }, Exp_ { pos: 44, data: String("5") }]) }, Stm_ { pos: 81, data: Compound([Stm_ { pos: 79, data: Assign(Var_ { pos: 60, data: Simple("a") }, Exp_ { pos: 73, data: Suffix(Exp_ { pos: 64, data: Var("表示") }, [ExpSuffix_ { pos: 73, data: Call([Exp_ { pos: 72, data: String("こんにちは") }]) }]) }, Normal) }]) }) }, IfElse_ { pos: 133, data: ElseIf(Exp_ { pos: 95, data: BinOp([Eq], [Exp_ { pos: 91, data: Var("a") }, Exp_ { pos: 95, data: String("4") }]) }, Stm_ { pos: 133, data: Compound([Stm_ { pos: 131, data: Assign(Var_ { pos: 112, data: Simple("a") }, Exp_ { pos: 125, data: Suffix(Exp_ { pos: 116, data: Var("表示") }, [ExpSuffix_ { pos: 125, data: Call([Exp_ { pos: 124, data: String("こんばんは") }]) }]) }, Normal) }]) }) }, IfElse_ { pos: 184, data: ElseIf(Exp_ { pos: 147, data: BinOp([Eq], [Exp_ { pos: 143, data: Var("a") }, Exp_ { pos: 147, data: String("3") }]) }, Stm_ { pos: 184, data: Compound([Stm_ { pos: 182, data: Assign(Var_ { pos: 164, data: Simple("a") }, Exp_ { pos: 176, data: Suffix(Exp_ { pos: 168, data: Var("表示") }, [ExpSuffix_ { pos: 176, data: Call([Exp_ { pos: 175, data: String("おはよう") }]) }]) }, Normal) }]) }) }, IfElse_ { pos: 222, data: Else(Stm_ { pos: 222, data: Compound([Stm_ { pos: 220, data: Assign(Var_ { pos: 202, data: Simple("a") }, Exp_ { pos: 214, data: Suffix(Exp_ { pos: 206, data: Var("表示") }, [ExpSuffix_ { pos: 214, data: Call([Exp_ { pos: 213, data: String("ばいばい") }]) }]) }, Normal) }]) }) }]) }]) }) }])"##
    );
}

#[test]
fn if_else() {
    let mut test_parser = LazeParser::new(Path::new("./parser_files/ja.peg"));
    let ast = test_parser.parse(Path::new("./laze_tests/stm/if_stm/if_else.laze"));
    let mut ast_string = String::new();
    let _ = fmt::write(&mut ast_string, format_args!("{:?}", ast));
    assert_eq!(
        ast_string,
        r##"DecList([Dec_ { pos: 120, data: Func("実行", [], [], Stm_ { pos: 120, data: Compound([Stm_ { pos: 35, data: Dec(Dec_ { pos: 35, data: Var(Var_ { pos: 26, data: Simple("a") }, Type_ { pos: 22, data: Name("整数") }, Exp_ { pos: 29, data: String("4") }) }) }, Stm_ { pos: 119, data: IfElse([IfElse_ { pos: 81, data: If(Exp_ { pos: 44, data: BinOp([Eq], [Exp_ { pos: 40, data: Var("a") }, Exp_ { pos: 44, data: String("5") }]) }, Stm_ { pos: 81, data: Compound([Stm_ { pos: 79, data: Assign(Var_ { pos: 60, data: Simple("a") }, Exp_ { pos: 73, data: Suffix(Exp_ { pos: 64, data: Var("表示") }, [ExpSuffix_ { pos: 73, data: Call([Exp_ { pos: 72, data: String("こんにちは") }]) }]) }, Normal) }]) }) }, IfElse_ { pos: 119, data: Else(Stm_ { pos: 119, data: Compound([Stm_ { pos: 117, data: Assign(Var_ { pos: 99, data: Simple("a") }, Exp_ { pos: 111, data: Suffix(Exp_ { pos: 103, data: Var("表示") }, [ExpSuffix_ { pos: 111, data: Call([Exp_ { pos: 110, data: String("ばいばい") }]) }]) }, Normal) }]) }) }]) }]) }) }])"##
    );
}
