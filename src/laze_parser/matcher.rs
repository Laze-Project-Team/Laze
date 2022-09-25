use std::io::{stderr, Write};

use peg_parser::Parser;

use crate::ast::{
    ast::ASTNode,
    dec::{ClassMemberList, Dec_, MemberList, MemberSpecifier},
    exp::Exp_,
    field::Field_,
    ifelse::IfElse_,
    op::{string_to_oper, Oper},
    stm::{AssignType, Stm_},
    suffix::ExpSuffix_,
    ty::Type_,
    var::Var_,
};

use super::extracter::*;

pub fn extract_ast(pos: (usize, usize), name: &str, parser: &mut Parser<ASTNode>) -> ASTNode {
    // println!("Reducing: {}", name);
    match name {
        "String" => {
            let mut content = extract_string_data(pos, parser.get_data("string"), "string", name);
            if content.len() >= 2 {
                content.remove(0);
                content.remove(content.len() - 1);
            }
            let newcontent = match content.as_str() {
                "\\\"" => "\"".to_string(),
                "\\\\" => "\\".to_string(),
                str => str.to_string(),
            };
            // println!("StringContent: {newcontent}");
            ASTNode::String(newcontent)
        }
        "Real" => ASTNode::String(extract_string_data(
            pos,
            parser.get_data("real"),
            "real",
            name,
        )),
        "Integer" => ASTNode::String(extract_string_data(
            pos,
            parser.get_data("int"),
            "int",
            name,
        )),
        "ID" => {
            let id_ = extract_string_data(pos, parser.get_data("id"), "id", name);
            let id = id_.trim().to_string();
            match parser.get_data_from_parent_scope("ID") {
                Some(node) => match node {
                    ASTNode::String(str) => ASTNode::StringList(vec![str, id]),
                    ASTNode::StringList(strlist) => {
                        let mut list = strlist;
                        list.push(id);
                        ASTNode::StringList(list)
                    }
                    _ => {
                        panic!("ID is not string.");
                    }
                },
                None => ASTNode::String(id),
            }
        }
        "IDList" => ASTNode::StringList(extract_stringlist_data(
            pos,
            parser.get_data("ID"),
            "ID",
            name,
        )),
        "Var" => ASTNode::Var(extract_var_data(pos, parser.get_data("var"), "var", name)),
        "SimpleVar" => ASTNode::Var(Var_::simple_var(
            pos,
            extract_string_data(pos, parser.get_data("ID"), "ID", name),
        )),
        "CallVar" => match parser.get_data("exp") {
            Some(node) => ASTNode::Var(Var_::call_var(
                pos,
                extract_var_data(pos, parser.get_data("SimpleVar"), "SimpleVar", name),
                node.get_explist_data(pos, "CallVar", name),
            )),
            None => parser.get_data("SimpleVar").expect("SimpleVar in CallVar"),
        },
        "ArrayVar" => match parser.get_data("exp") {
            Some(exp) => ASTNode::Var(Var_::array_var(
                pos,
                extract_var_data(pos, parser.get_data("CallVar"), "CallVar", name),
                exp.get_explist_data(pos, "exp", name),
            )),
            None => parser.get_data("CallVar").expect("CallVar in ArrayVar"),
        },
        "PointerVar" => match parser.get_data("pointer") {
            Some(node) => {
                if let ASTNode::String(str) = node {
                    if str.starts_with("*") {
                        ASTNode::Var(Var_::pointer_var(
                            pos,
                            extract_var_data(pos, parser.get_data("ArrayVar"), "ArrayVar", name),
                        ))
                    } else {
                        parser.get_data("ArrayVar").expect("ArrayVar")
                    }
                } else {
                    ASTNode::None
                }
            }
            None => ASTNode::None,
        },
        "PointerType" => ASTNode::Type(Type_::pointer_type(
            pos,
            extract_ty_data(pos, parser.get_data("PrimaryType"), "ParenType", name),
        )),
        "ParenType" => parser.get_data("Type").expect("ParenType"),
        "NameType" => ASTNode::Type(Type_::name_type(
            pos,
            extract_string_data(pos, parser.get_data("type"), "type", name),
        )),
        "GenericsType" => ASTNode::Type(Type_::template_type(
            pos,
            extract_string_data(pos, parser.get_data("ID"), "ID", name),
            extract_tylist_data(pos, parser.get_data("IDList"), "IDList", name),
        )),
        "ArrayType" => match parser.get_data("exp") {
            Some(exp) => ASTNode::Type(Type_::array_type(
                pos,
                extract_ty_data(pos, parser.get_data("PrimaryType"), "PrimaryType", name),
                exp.get_exp_data(pos, "exp", name),
            )),
            None => parser
                .get_data("PrimaryType")
                .expect("PrimaryType in ArrayType"),
        },
        "PrimaryType" => parser.get_data("type").expect("PrimaryType"),
        "Type" => parser.get_data("type").expect("Type"),
        "If" | "ElseIf" => {
            let test = extract_exp_data(pos, parser.get_data("Exp"), "Exp", name);
            let body = extract_stm_data(pos, parser.get_data("Stm"), "Stm", name);
            if name == "If" {
                ASTNode::IfElseList(vec![IfElse_::if_(pos, test, body)])
            } else if name == "ElseIf" {
                match parser.get_data_from_parent_scope("ifelse") {
                    Some(node) => match node {
                        ASTNode::IfElseList(mut list) => {
                            list.push(IfElse_::else_if(pos, test, body));
                            return ASTNode::IfElseList(list);
                        }
                        _ => ASTNode::None,
                    },
                    None => ASTNode::IfElseList(vec![IfElse_::else_if(pos, test, body)]),
                }
            } else {
                ASTNode::None
            }
        }
        "Else" => match parser.get_data_from_parent_scope("ifelse") {
            Some(node) => match node {
                ASTNode::IfElseList(mut list) => {
                    list.push(IfElse_::else_(
                        pos,
                        extract_stm_data(pos, parser.get_data("Stm"), "Stm", name),
                    ));
                    return ASTNode::IfElseList(list);
                }
                _ => ASTNode::None,
            },
            None => ASTNode::IfElseList(vec![IfElse_::else_(
                pos,
                extract_stm_data(pos, parser.get_data("Stm"), "Stm", name),
            )]),
        },
        "IfElseList" => parser.get_data("ifelse").expect("IfElseList"),
        "LoopStm" => ASTNode::Stm(Stm_::loop_stm(
            pos,
            extract_stm_data(pos, parser.get_data("Stm"), "Stm", name),
        )),
        "ReturnStm" => ASTNode::Stm(Stm_::return_stm(
            pos,
            match parser.get_data("Exp") {
                Some(exp) => match exp {
                    ASTNode::Exp(e) => e,
                    _ => Exp_::none_exp(pos),
                },
                None => Exp_::none_exp(pos),
            },
        )),
        "ContinueStm" => ASTNode::Stm(Stm_::continue_stm(pos)),
        "BreakStm" => ASTNode::Stm(Stm_::break_stm(pos)),
        "RepeatStm" => ASTNode::Stm(Stm_::repeat_stm(
            pos,
            extract_exp_data(pos, parser.get_data("Exp"), "Exp", name),
            extract_stm_data(pos, parser.get_data("Stm"), "Stm", name),
        )),
        "UntilStm" => ASTNode::Stm(Stm_::while_stm(
            pos,
            Exp_::unaryop_exp(
                pos,
                vec![Oper::Not],
                extract_exp_data(pos, parser.get_data("Exp"), "Exp", name),
            ),
            extract_stm_data(pos, parser.get_data("Stm"), "Stm", name),
        )),
        "WhileStm" => ASTNode::Stm(Stm_::while_stm(
            pos,
            extract_exp_data(pos, parser.get_data("Exp"), "Exp", name),
            extract_stm_data(pos, parser.get_data("Stm"), "Stm", name),
        )),
        "IfStm" => ASTNode::Stm(Stm_::ifelse_stm(
            pos,
            extract_ifelselist_data(pos, parser.get_data("IfElseList"), "IfElseList", name),
        )),
        "AssignStm" => parser.get_data("stm").expect("stm in AssignStm"),
        "NormalAssign" | "AddAssign" | "SubAssign" | "MulAssign" | "DivAssign" => {
            ASTNode::Stm(Stm_::assign_stm(
                pos,
                extract_var_data(pos, parser.get_data("Var"), "Var", name),
                extract_exp_data(pos, parser.get_data("Exp"), "Exp", name),
                match name {
                    "NormalAssign" => AssignType::Normal,
                    "AddAssign" => AssignType::Add,
                    "SubAssign" => AssignType::Sub,
                    "MulAssign" => AssignType::Mul,
                    "DivAssign" => AssignType::Div,
                    _ => AssignType::Normal,
                },
            ))
        }
        "DecStm" => ASTNode::Stm(Stm_::dec_stm(
            pos,
            extract_dec_data(pos, parser.get_data("Dec"), "Dec", name),
        )),
        "ExpStm" => ASTNode::Stm(Stm_::exp_stm(
            pos,
            extract_exp_data(pos, parser.get_data("Exp"), "Exp", name),
        )),
        "Stm" => match parser.get_data_from_parent_scope("Stm") {
            Some(node) => match node {
                ASTNode::Stm(stm) => ASTNode::StmList(vec![
                    stm,
                    extract_stm_data(pos, parser.get_data("stm"), "stm", name),
                ]),
                ASTNode::StmList(mut stmlist) => {
                    stmlist.push(extract_stm_data(pos, parser.get_data("stm"), "stm", name));
                    ASTNode::StmList(stmlist)
                }
                _ => {
                    let _ = writeln!(stderr(), "stm is not a statement or a statement list.");
                    ASTNode::None
                }
            },
            None => parser.get_data("stm").expect("Stm"),
        },
        "StmList" => match parser.get_data("Stm") {
            Some(node) => match node {
                ASTNode::Stm(stm) => ASTNode::StmList(vec![stm]),
                ASTNode::StmList(stmlist) => ASTNode::StmList(stmlist),
                _ => ASTNode::None,
            },
            None => ASTNode::StmList(vec![]),
        },
        "CompoundStm" => ASTNode::Stm(Stm_::compound_stm(
            pos,
            extract_stmlist_data(pos, parser.get_data("StmList"), "StmList", name),
        )),
        "IntExp" => ASTNode::Exp(Exp_::int_exp(
            pos,
            extract_string_data(pos, parser.get_data("Integer"), "Integer", name),
        )),
        "RealExp" => ASTNode::Exp(Exp_::real_exp(
            pos,
            extract_string_data(pos, parser.get_data("Real"), "Real", name),
        )),
        "StringExp" => ASTNode::Exp(Exp_::string_exp(
            pos,
            extract_string_data(pos, parser.get_data("String"), "String", name),
        )),
        "IDExp" => ASTNode::Exp(Exp_::var_exp(
            pos,
            extract_string_data(pos, parser.get_data("ID"), "ID", name),
        )),
        "ConstantExp" | "PrimaryExp" => {
            let exp = parser.get_data("exp").expect("ConstantExp / PrimaryExp");
            match parser.get_data_from_parent_scope("exp") {
                Some(node) => match node {
                    ASTNode::ExpList(mut explist) => {
                        explist.push(exp.get_exp_data(pos, "exp", name));
                        ASTNode::ExpList(explist)
                    }
                    ASTNode::Exp(e) => {
                        ASTNode::ExpList(vec![e, exp.get_exp_data(pos, "exp", name)])
                    }
                    _ => ASTNode::ExpList(vec![exp.get_exp_data(pos, "exp", name)]),
                },
                None => ASTNode::ExpList(vec![exp.get_exp_data(pos, "exp", name)]),
            }
        }
        "ParenExp" => ASTNode::Exp(Exp_::paren_exp(
            pos,
            extract_exp_data(pos, parser.get_data("exp"), "exp", name),
        )),
        "SizeOfExp" => ASTNode::Exp(Exp_::sizeof_exp(
            pos,
            extract_exp_data(pos, parser.get_data("exp"), "exp", name),
        )),
        "ArrayExp" => ASTNode::Exp(Exp_::array_exp(
            pos,
            extract_explist_data(pos, parser.get_data("ExpList"), "ExpList", name),
        )),
        "FuncExp" => ASTNode::Exp(Exp_::func_exp(
            pos,
            extract_fieldlist_data(pos, parser.get_data("params"), "params", name),
            extract_fieldlist_data(pos, parser.get_data("result"), "result", name),
            extract_stm_data(pos, parser.get_data("Stm"), "Stm", name),
        )),
        "PostfixExp" => {
            let new_exp = extract_exp_data(pos, parser.get_data("exp"), "exp", name);
            let exp = match parser.get_data("suffix") {
                Some(node) => match node {
                    ASTNode::ExpSuffixList(suffix) => {
                        ASTNode::Exp(Exp_::suffix_exp(pos, new_exp, suffix))
                    }
                    _ => ASTNode::Exp(new_exp),
                },
                None => ASTNode::Exp(new_exp),
            };
            match parser.get_data_from_parent_scope("exp") {
                Some(node) => match node {
                    ASTNode::ExpList(mut explist) => {
                        explist.push(exp.get_exp_data(pos, "exp", name));
                        ASTNode::ExpList(explist)
                    }
                    ASTNode::Exp(e) => {
                        ASTNode::ExpList(vec![e, exp.get_exp_data(pos, "exp", name)])
                    }
                    _ => ASTNode::ExpList(vec![exp.get_exp_data(pos, "exp", name)]),
                },
                None => ASTNode::ExpList(vec![exp.get_exp_data(pos, "exp", name)]),
            }
        }
        "CallSuffix" | "DotSuffix" | "ArrowSuffix" | "SubscriptSuffix" => {
            let data = if name == "CallSuffix" {
                ExpSuffix_::call_suffix(
                    pos,
                    extract_explist_data(pos, parser.get_data("explist"), "explist", name),
                )
            } else {
                let data = extract_exp_data(pos, parser.get_data("exp"), "exp", name);
                if name == "DotSuffix" {
                    ExpSuffix_::dot_suffix(pos, data)
                } else if name == "ArrowSuffix" {
                    ExpSuffix_::arrow_suffix(pos, data)
                } else if name == "SubscriptSuffix" {
                    ExpSuffix_::subscript_suffix(pos, data)
                } else {
                    panic!("suffix is not dot nor arrow nor subscript.");
                }
            };
            match parser.get_data_from_parent_scope("suffix") {
                Some(node) => match node {
                    ASTNode::ExpSuffixList(mut list) => {
                        list.push(data);
                        ASTNode::ExpSuffixList(list)
                    }
                    _ => {
                        let _ = writeln!(stderr(), "suffix is not ExpSuffixList.");
                        ASTNode::ExpSuffixList(vec![data])
                    }
                },
                None => ASTNode::ExpSuffixList(vec![data]),
            }
        }
        "AndOp" | "OrOp" | "EqOp" | "NeOp" | "LtOp" | "LeOp" | "GtOp" | "GeOp" | "AddOp"
        | "SubOp" | "MulOp" | "DivOp" | "DerefOp" | "AddressOp" | "NotOp" => {
            match parser.get_data_from_parent_scope("op") {
                Some(oplist) => match oplist {
                    ASTNode::OperList(mut list) => {
                        list.push(string_to_oper(name));
                        ASTNode::OperList(list)
                    }
                    _ => {
                        let _ = writeln!(stderr(), "\"op\" is not an operator list.");
                        ASTNode::None
                    }
                },
                None => ASTNode::OperList(vec![string_to_oper(name)]),
            }
        }
        "Exp" => {
            let new_exp = extract_exp_data(pos, parser.get_data("exp"), "exp", name);
            match parser.get_data_from_parent_scope("exp") {
                Some(node) => match node {
                    ASTNode::Exp(exp) => ASTNode::ExpList(vec![exp, new_exp]),
                    ASTNode::ExpList(mut explist) => {
                        explist.push(new_exp);
                        ASTNode::ExpList(explist)
                    }
                    _ => ASTNode::ExpList(vec![new_exp]),
                },
                None => ASTNode::ExpList(vec![new_exp]),
            }
        }
        "UnaryOpExp" | "ProdExp" | "SumExp" | "CompOpExp" | "BinOpExp" => {
            let handled_exp = match parser.get_data("op") {
                Some(node) => {
                    let oplist = node.get_operlist_data(pos, "op", name);
                    if name == "UnaryOpExp" {
                        Exp_::unaryop_exp(
                            pos,
                            oplist,
                            extract_exp_data(pos, parser.get_data("exp"), "exp", name),
                        )
                    } else {
                        Exp_::binop_exp(
                            pos,
                            oplist,
                            extract_explist_data(pos, parser.get_data("exp"), "exp", name),
                        )
                    }
                }
                None => extract_exp_data(pos, parser.get_data("exp"), "exp", name),
            };
            match parser.get_data_from_parent_scope("exp") {
                Some(node) => match node {
                    ASTNode::ExpList(mut list) => {
                        list.push(handled_exp);
                        ASTNode::ExpList(list)
                    }
                    ASTNode::Exp(exp) => ASTNode::ExpList(vec![exp, handled_exp]),
                    _ => {
                        let _ = writeln!(stderr(), "exp is not an explist. {:?}", node);
                        ASTNode::None
                    }
                },
                None => ASTNode::ExpList(vec![handled_exp]),
            }
        }
        "ExpList" => match parser.get_data("exp") {
            Some(explist) => explist,
            None => ASTNode::ExpList(vec![]),
        },
        "Field" => {
            let new_node = Field_::new(
                pos,
                extract_var_data(pos, parser.get_data("Var"), "Var", name),
                extract_ty_data(pos, parser.get_data("Type"), "Type", name),
            );
            match parser.get_data_from_parent_scope("Field") {
                Some(node) => match node {
                    ASTNode::FieldList(mut list) => {
                        list.push(new_node);
                        ASTNode::FieldList(list)
                    }
                    _ => {
                        let _ = writeln!(stderr(), "Field is not a Fieldlist");
                        ASTNode::None
                    }
                },
                None => ASTNode::FieldList(vec![new_node]),
            }
        }
        "FieldList" => match parser.get_data("Field") {
            Some(fieldlist) => fieldlist,
            None => ASTNode::FieldList(vec![]),
        },
        "PublicMembers" | "PrivateMembers" => {
            let new_list = ClassMemberList::new_list(
                extract_declist_data(pos, parser.get_data("DecList"), "DecList", name),
                if name == "PublicMembers" {
                    MemberSpecifier::Public
                } else {
                    MemberSpecifier::Private
                },
            );
            match parser.get_data_from_parent_scope("members") {
                Some(node) => match node {
                    ASTNode::ClassMemberList(mut list) => {
                        list.append_list(new_list);
                        ASTNode::ClassMemberList(list)
                    }
                    _ => {
                        let _ = writeln!(stderr(), "members is not a class member list.");
                        ASTNode::ClassMemberList(new_list)
                    }
                },
                None => ASTNode::ClassMemberList(new_list),
            }
        }
        "ClassMemberList" => parser.get_data("members").expect("ClassMemberList"),
        "ClassDec" => ASTNode::Dec(Dec_::class_dec(
            pos,
            extract_string_data(pos, parser.get_data("ID"), "ID", name),
            extract_classmembers_data(
                pos,
                parser.get_data("ClassMemberList"),
                "ClassMemberList",
                name,
            ),
            extract_stringlist_data(pos, parser.get_data("IDList"), "IDList", name),
        )),
        "OperDec" | "FuncDec" | "JsImportDec" => {
            let id = extract_string_data(pos, parser.get_data("ID"), "ID", name);
            let params = extract_fieldlist_data(pos, parser.get_data("params"), "params", name);
            let result = extract_fieldlist_data(pos, parser.get_data("result"), "result", name);

            if name == "OperDec" || name == "FuncDec" {
                let body = Stm_::compound_stm(
                    pos,
                    extract_stmlist_data(pos, parser.get_data("StmList"), "StmList", name),
                );
                if name == "OperDec" {
                    ASTNode::Dec(Dec_::oper_dec(pos, id, params, result, body))
                } else {
                    ASTNode::Dec(Dec_::func_dec(pos, id, params, result, body))
                }
            } else {
                let module = extract_string_data(pos, parser.get_data("module"), "module", name);
                let name = extract_string_data(pos, parser.get_data("name"), "name", name);
                ASTNode::Dec(Dec_::js_import_dec(pos, id, params, result, module, name))
            }
        }
        "JsExportDec" => ASTNode::Dec(Dec_::js_export_dec(
            pos,
            extract_string_data(pos, parser.get_data("ID"), "ID", name),
            extract_string_data(pos, parser.get_data("String"), "String", name),
        )),
        "TemplateDec" => ASTNode::Dec(Dec_::template_dec(
            pos,
            extract_dec_data(pos, parser.get_data("Dec"), "Dec", name),
            extract_stringlist_data(pos, parser.get_data("IDList"), "IDList", name),
        )),
        "VarDecInit" | "VarDecNoInit" => {
            let var = extract_var_data(pos, parser.get_data("Var"), "Var", name);
            let ty = extract_ty_data(pos, parser.get_data("Type"), "Type", name);
            if name == "VarDecInit" {
                ASTNode::Dec(Dec_::var_dec(
                    pos,
                    var,
                    ty,
                    extract_exp_data(pos, parser.get_data("Exp"), "Exp", name),
                ))
            } else {
                ASTNode::Dec(Dec_::var_dec(pos, var, ty, Exp_::none_exp(pos)))
            }
        }
        "VarDec" => parser.get_data("vardec").expect("VarDec"),
        "Dec" => {
            let new_dec = extract_dec_data(pos, parser.get_data("dec"), "dec", name);
            match parser.get_data_from_parent_scope("Dec") {
                Some(dec) => match dec {
                    ASTNode::DecList(mut list) => {
                        list.push(new_dec);
                        ASTNode::DecList(list)
                    }
                    _ => {
                        let _ = writeln!(stderr(), "dec is not a declaration list.");
                        ASTNode::DecList(vec![new_dec])
                    }
                },
                None => ASTNode::DecList(vec![new_dec]),
            }
        }
        "DecList" => parser.get_data("Dec").expect("DecList"),
        "Start" => parser.get_data("DecList").expect("Start"),
        _ => {
            let _ = writeln!(stderr(), "What is this token: {name}");
            ASTNode::None
        }
    }
}
