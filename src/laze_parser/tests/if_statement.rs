use std::fmt;
use std::path::Path;

use crate::laze_parser::parser::LazeParser;

#[test]
fn only_if() {
    let mut test_parser = LazeParser::new(Path::new("./parser_files/ja.peg"));
    let ast = test_parser.parse(Path::new("./laze_tests/if_statement/if_only_if.laze"));
    let mut ast_string = String::new();
    let _ = fmt::write(&mut ast_string, format_args!("{:?}", ast));
    assert_eq!(
        ast_string,
        r##"DecList([Dec_ { pos: 96, data: Func("実行", [], [], Stm_ { pos: 96, data: Compound([Stm_ { pos: 37, data: Dec(Dec_ { pos: 37, data: Var(Var_ { pos: 27, data: Simple("a") }, Type_ { pos: 23, data: Name("整数") }, Exp_ { pos: 30, data: String("4") }) }) }, Stm_ { pos: 95, data: IfElse([IfElse_ { pos: 95, data: If(Exp_ { pos: 46, data: BinOp([Eq], [Exp_ { pos: 42, data: Var("a") }, Exp_ { pos: 46, data: String("5") }]) }, Stm_ { pos: 95, data: Compound([Stm_ { pos: 92, data: Assign(Var_ { pos: 63, data: Simple("a") }, Exp_ { pos: 85, data: Suffix(Exp_ { pos: 67, data: Var("表示") }, [ExpSuffix_ { pos: 85, data: Call([Exp_ { pos: 75, data: String("こんにちは") }, Exp_ { pos: 84, data: String("こんばんは") }]) }]) }) }]) }) }]) }]) }) }])"##
    );
}

#[test]
fn elseif_else() {
    let mut test_parser = LazeParser::new(Path::new("./parser_files/ja.peg"));
    let ast = test_parser.parse(Path::new("./laze_tests/if_statement/if_elseif_else.laze"));
    let mut ast_string = String::new();
    let _ = fmt::write(&mut ast_string, format_args!("{:?}", ast));
    assert_eq!(
        ast_string,
        r##"DecList([Dec_ { pos: 234, data: Func("実行", [], [], Stm_ { pos: 234, data: Compound([Stm_ { pos: 37, data: Dec(Dec_ { pos: 37, data: Var(Var_ { pos: 27, data: Simple("a") }, Type_ { pos: 23, data: Name("整数") }, Exp_ { pos: 30, data: String("4") }) }) }, Stm_ { pos: 233, data: IfElse([IfElse_ { pos: 85, data: If(Exp_ { pos: 46, data: BinOp([Eq], [Exp_ { pos: 42, data: Var("a") }, Exp_ { pos: 46, data: String("5") }]) }, Stm_ { pos: 85, data: Compound([Stm_ { pos: 83, data: Assign(Var_ { pos: 63, data: Simple("a") }, Exp_ { pos: 76, data: Suffix(Exp_ { pos: 67, data: Var("表示") }, [ExpSuffix_ { pos: 76, data: Call([Exp_ { pos: 75, data: String("こんにちは") }]) }]) }) }]) }) }, IfElse_ { pos: 139, data: ElseIf(Exp_ { pos: 99, data: BinOp([Eq], [Exp_ { pos: 95, data: Var("a") }, Exp_ { pos: 99, data: String("4") }]) }, Stm_ { pos: 139, data: Compound([Stm_ { pos: 137, data: Assign(Var_ { pos: 117, data: Simple("a") }, Exp_ { pos: 130, data: Suffix(Exp_ { pos: 121, data: Var("表示") }, [ExpSuffix_ { pos: 130, data: Call([Exp_ { pos: 129, data: String("こんばんは") }]) }]) }) }]) }) }, IfElse_ { pos: 192, data: ElseIf(Exp_ { pos: 153, data: BinOp([Eq], [Exp_ { pos: 149, data: Var("a") }, Exp_ { pos: 153, data: String("3") }]) }, Stm_ { pos: 192, data: Compound([Stm_ { pos: 190, data: Assign(Var_ { pos: 171, data: Simple("a") }, Exp_ { pos: 183, data: Suffix(Exp_ { pos: 175, data: Var("表示") }, [ExpSuffix_ { pos: 183, data: Call([Exp_ { pos: 182, data: String("おはよう") }]) }]) }) }]) }) }, IfElse_ { pos: 233, data: Else(Stm_ { pos: 233, data: Compound([Stm_ { pos: 230, data: Assign(Var_ { pos: 211, data: Simple("a") }, Exp_ { pos: 223, data: Suffix(Exp_ { pos: 215, data: Var("表示") }, [ExpSuffix_ { pos: 223, data: Call([Exp_ { pos: 222, data: String("ばいばい") }]) }]) }) }]) }) }]) }]) }) }])"##
    );
}

#[test]
fn if_else() {
    let mut test_parser = LazeParser::new(Path::new("./parser_files/ja.peg"));
    let ast = test_parser.parse(Path::new("./laze_tests/if_statement/if_else.laze"));
    let mut ast_string = String::new();
    let _ = fmt::write(&mut ast_string, format_args!("{:?}", ast));
    assert_eq!(
        ast_string,
        r##"DecList([Dec_ { pos: 127, data: Func("実行", [], [], Stm_ { pos: 127, data: Compound([Stm_ { pos: 37, data: Dec(Dec_ { pos: 37, data: Var(Var_ { pos: 27, data: Simple("a") }, Type_ { pos: 23, data: Name("整数") }, Exp_ { pos: 30, data: String("4") }) }) }, Stm_ { pos: 126, data: IfElse([IfElse_ { pos: 85, data: If(Exp_ { pos: 46, data: BinOp([Eq], [Exp_ { pos: 42, data: Var("a") }, Exp_ { pos: 46, data: String("5") }]) }, Stm_ { pos: 85, data: Compound([Stm_ { pos: 83, data: Assign(Var_ { pos: 63, data: Simple("a") }, Exp_ { pos: 76, data: Suffix(Exp_ { pos: 67, data: Var("表示") }, [ExpSuffix_ { pos: 76, data: Call([Exp_ { pos: 75, data: String("こんにちは") }]) }]) }) }]) }) }, IfElse_ { pos: 126, data: Else(Stm_ { pos: 126, data: Compound([Stm_ { pos: 123, data: Assign(Var_ { pos: 104, data: Simple("a") }, Exp_ { pos: 116, data: Suffix(Exp_ { pos: 108, data: Var("表示") }, [ExpSuffix_ { pos: 116, data: Call([Exp_ { pos: 115, data: String("ばいばい") }]) }]) }) }]) }) }]) }]) }) }])"##
    );
}
