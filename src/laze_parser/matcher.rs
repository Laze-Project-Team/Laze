use std::io::{stderr, Write};

use peg_parser::Parser;

use crate::ast::{
    ast::ASTNode,
    dec::{ClassMemberList, Dec_, MemberList, MemberSpecifier},
    exp::{ExpData, Exp_},
    field::Field_,
    ifelse::IfElse_,
    op::{string_to_oper, Oper},
    stm::Stm_,
    suffix::ExpSuffix_,
    ty::Type_,
    var::Var_,
};

use super::extracter::*;

pub fn extract_ast(name: &str, parser: &mut Parser<ASTNode>) -> ASTNode {
    match name {
        "String" => {
            let mut content = extract_string_data(
                parser.pos,
                parser.get_data("string".to_string()),
                "string",
                name,
            );
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
            parser.pos,
            parser.get_data("real".to_string()),
            "real",
            name,
        )),
        "Integer" => ASTNode::String(extract_string_data(
            parser.pos,
            parser.get_data("int".to_string()),
            "int",
            name,
        )),
        "ID" => {
            let id_ =
                extract_string_data(parser.pos, parser.get_data("id".to_string()), "id", name);
            let id = id_.trim().to_string();
            match parser.get_data_from_parent_scope("ID".to_string()) {
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
            parser.pos,
            parser.get_data("ID".to_string()),
            "ID",
            name,
        )),
        "Var" => ASTNode::Var(extract_var_data(
            parser.pos,
            parser.get_data("var".to_string()),
            "var",
            name,
        )),
        "SimpleVar" => ASTNode::Var(Var_::simple_var(
            parser.pos,
            extract_string_data(parser.pos, parser.get_data("ID".to_string()), "ID", name),
        )),
        "CallVar" => match parser.get_data("exp".to_string()) {
            Some(node) => ASTNode::Var(Var_::call_var(
                parser.pos,
                extract_var_data(
                    parser.pos,
                    parser.get_data("SimpleVar".to_string()),
                    "SimpleVar",
                    name,
                ),
                node.get_explist_data(parser.pos, "CallVar", name),
            )),
            None => parser
                .get_data("SimpleVar".to_string())
                .expect("SimpleVar in CallVar"),
        },
        "ArrayVar" => match parser.get_data("exp".to_string()) {
            Some(exp) => ASTNode::Var(Var_::array_var(
                parser.pos,
                extract_var_data(
                    parser.pos,
                    parser.get_data("CallVar".to_string()),
                    "CallVar",
                    name,
                ),
                exp.get_exp_data(parser.pos, "exp", name),
            )),
            None => parser
                .get_data("CallVar".to_string())
                .expect("CallVar in ArrayVar"),
        },
        "PointerVar" => match parser.get_data("pointer".to_string()) {
            Some(node) => {
                if let ASTNode::String(str) = node {
                    if str.starts_with("*") {
                        ASTNode::Var(Var_::pointer_var(
                            parser.pos,
                            extract_var_data(
                                parser.pos,
                                parser.get_data("ArrayVar".to_string()),
                                "ArrayVar",
                                name,
                            ),
                        ))
                    } else {
                        parser.get_data("ArrayVar".to_string()).expect("ArrayVar")
                    }
                } else {
                    ASTNode::None
                }
            }
            None => ASTNode::None,
        },
        "PointerType" => ASTNode::Type(Type_::pointer_type(
            parser.pos,
            extract_ty_data(
                parser.pos,
                parser.get_data("PrimaryType".to_string()),
                "ParenType",
                name,
            ),
        )),
        "ParenType" => parser.get_data("Type".to_string()).expect("ParenType"),
        "NameType" => ASTNode::Type(Type_::name_type(
            parser.pos,
            extract_string_data(
                parser.pos,
                parser.get_data("type".to_string()),
                "type",
                name,
            ),
        )),
        "GenericsType" => ASTNode::Type(Type_::template_type(
            parser.pos,
            extract_string_data(parser.pos, parser.get_data("ID".to_string()), "ID", name),
            extract_tylist_data(
                parser.pos,
                parser.get_data("IDList".to_string()),
                "IDList",
                name,
            ),
        )),
        "ArrayType" => match parser.get_data("exp".to_string()) {
            Some(exp) => ASTNode::Type(Type_::array_type(
                parser.pos,
                extract_ty_data(
                    parser.pos,
                    parser.get_data("PrimaryType".to_string()),
                    "PrimaryType",
                    name,
                ),
                exp.get_exp_data(parser.pos, "exp", name),
            )),
            None => parser
                .get_data("PrimaryType".to_string())
                .expect("PrimaryType in ArrayType"),
        },
        "PrimaryType" => parser.get_data("type".to_string()).expect("PrimaryType"),
        "Type" => parser.get_data("type".to_string()).expect("Type"),
        "If" | "ElseIf" => {
            let test =
                extract_exp_data(parser.pos, parser.get_data("Exp".to_string()), "Exp", name);
            let body =
                extract_stm_data(parser.pos, parser.get_data("Stm".to_string()), "Stm", name);
            if name == "If" {
                ASTNode::IfElseList(vec![IfElse_::if_(parser.pos, test, body)])
            } else if name == "ElseIf" {
                match parser.get_data_from_parent_scope("ifelse".to_string()) {
                    Some(node) => match node {
                        ASTNode::IfElseList(mut list) => {
                            list.push(IfElse_::else_if(parser.pos, test, body));
                            return ASTNode::IfElseList(list);
                        }
                        _ => ASTNode::None,
                    },
                    None => ASTNode::IfElseList(vec![IfElse_::else_if(parser.pos, test, body)]),
                }
            } else {
                ASTNode::None
            }
        }
        "Else" => match parser.get_data_from_parent_scope("ifelse".to_string()) {
            Some(node) => match node {
                ASTNode::IfElseList(mut list) => {
                    list.push(IfElse_::else_(
                        parser.pos,
                        extract_stm_data(
                            parser.pos,
                            parser.get_data("Stm".to_string()),
                            "Stm",
                            name,
                        ),
                    ));
                    return ASTNode::IfElseList(list);
                }
                _ => ASTNode::None,
            },
            None => ASTNode::IfElseList(vec![IfElse_::else_(
                parser.pos,
                extract_stm_data(parser.pos, parser.get_data("Stm".to_string()), "Stm", name),
            )]),
        },
        "IfElseList" => parser.get_data("ifelse".to_string()).expect("IfElseList"),
        "LoopStm" => ASTNode::Stm(Stm_::loop_stm(
            parser.pos,
            extract_stm_data(parser.pos, parser.get_data("Stm".to_string()), "Stm", name),
        )),
        "ReturnStm" => ASTNode::Stm(Stm_::return_stm(
            parser.pos,
            match parser.get_data("Exp".to_string()) {
                Some(exp) => match exp {
                    ASTNode::Exp(e) => e,
                    _ => Box::new(Exp_ {
                        pos: parser.pos,
                        data: ExpData::None,
                    }),
                },
                None => Box::new(Exp_ {
                    pos: parser.pos,
                    data: ExpData::None,
                }),
            },
        )),
        "ContinueStm" => ASTNode::Stm(Stm_::continue_stm(parser.pos)),
        "BreakStm" => ASTNode::Stm(Stm_::break_stm(parser.pos)),
        "RepeatStm" => ASTNode::Stm(Stm_::repeat_stm(
            parser.pos,
            extract_exp_data(parser.pos, parser.get_data("Exp".to_string()), "Exp", name),
            extract_stm_data(parser.pos, parser.get_data("Stm".to_string()), "Stm", name),
        )),
        "UntilStm" => ASTNode::Stm(Stm_::while_stm(
            parser.pos,
            Exp_::unaryop_exp(
                parser.pos,
                vec![Oper::Not],
                extract_exp_data(parser.pos, parser.get_data("Exp".to_string()), "Exp", name),
            ),
            extract_stm_data(parser.pos, parser.get_data("Stm".to_string()), "Stm", name),
        )),
        "WhileStm" => ASTNode::Stm(Stm_::while_stm(
            parser.pos,
            extract_exp_data(parser.pos, parser.get_data("Exp".to_string()), "Exp", name),
            extract_stm_data(parser.pos, parser.get_data("Stm".to_string()), "Stm", name),
        )),
        "IfStm" => ASTNode::Stm(Stm_::ifelse_stm(
            parser.pos,
            extract_ifelselist_data(
                parser.pos,
                parser.get_data("IfElseList".to_string()),
                "IfElseList",
                name,
            ),
        )),
        "AssignStm" => ASTNode::Stm(Stm_::assign_stm(
            parser.pos,
            extract_var_data(parser.pos, parser.get_data("Var".to_string()), "Var", name),
            extract_exp_data(parser.pos, parser.get_data("Exp".to_string()), "Exp", name),
        )),
        "DecStm" => ASTNode::Stm(Stm_::dec_stm(
            parser.pos,
            extract_dec_data(parser.pos, parser.get_data("Dec".to_string()), "Dec", name),
        )),
        "ExpStm" => ASTNode::Stm(Stm_::exp_stm(
            parser.pos,
            extract_exp_data(parser.pos, parser.get_data("Exp".to_string()), "Exp", name),
        )),
        "Stm" => match parser.get_data_from_parent_scope("Stm".to_string()) {
            Some(node) => match node {
                ASTNode::Stm(stm) => ASTNode::StmList(vec![
                    stm,
                    extract_stm_data(parser.pos, parser.get_data("stm".to_string()), "stm", name),
                ]),
                ASTNode::StmList(mut stmlist) => {
                    stmlist.push(extract_stm_data(
                        parser.pos,
                        parser.get_data("stm".to_string()),
                        "stm",
                        name,
                    ));
                    ASTNode::StmList(stmlist)
                }
                _ => {
                    let _ = writeln!(stderr(), "stm is not a statement or a statement list.");
                    ASTNode::None
                }
            },
            None => parser.get_data("stm".to_string()).expect("Stm"),
        },
        "StmList" => match parser.get_data("Stm".to_string()) {
            Some(node) => match node {
                ASTNode::Stm(stm) => ASTNode::StmList(vec![stm]),
                ASTNode::StmList(stmlist) => ASTNode::StmList(stmlist),
                _ => ASTNode::None,
            },
            None => ASTNode::StmList(vec![]),
        },
        "CompoundStm" => ASTNode::Stm(Stm_::compound_stm(
            parser.pos,
            extract_stmlist_data(
                parser.pos,
                parser.get_data("StmList".to_string()),
                "StmList",
                name,
            ),
        )),
        "IntExp" => ASTNode::Exp(Exp_::int_exp(
            parser.pos,
            extract_string_data(
                parser.pos,
                parser.get_data("Integer".to_string()),
                "Integer",
                name,
            ),
        )),
        "RealExp" => ASTNode::Exp(Exp_::real_exp(
            parser.pos,
            extract_string_data(
                parser.pos,
                parser.get_data("Real".to_string()),
                "Real",
                name,
            ),
        )),
        "StringExp" => ASTNode::Exp(Exp_::string_exp(
            parser.pos,
            extract_string_data(
                parser.pos,
                parser.get_data("String".to_string()),
                "String",
                name,
            ),
        )),
        "IDExp" => ASTNode::Exp(Exp_::var_exp(
            parser.pos,
            extract_string_data(parser.pos, parser.get_data("ID".to_string()), "ID", name),
        )),
        "ConstantExp" | "PrimaryExp" => {
            let exp = parser
                .get_data("exp".to_string())
                .expect("ConstantExp / PrimaryExp");
            match parser.get_data_from_parent_scope("exp".to_string()) {
                Some(node) => match node {
                    ASTNode::ExpList(mut explist) => {
                        explist.push(exp.get_exp_data(parser.pos, "exp", name));
                        ASTNode::ExpList(explist)
                    }
                    ASTNode::Exp(e) => {
                        ASTNode::ExpList(vec![e, exp.get_exp_data(parser.pos, "exp", name)])
                    }
                    _ => ASTNode::ExpList(vec![exp.get_exp_data(parser.pos, "exp", name)]),
                },
                None => ASTNode::ExpList(vec![exp.get_exp_data(parser.pos, "exp", name)]),
            }
        }
        "ParenExp" => ASTNode::Exp(Exp_::paren_exp(
            parser.pos,
            extract_exp_data(parser.pos, parser.get_data("exp".to_string()), "exp", name),
        )),
        "SizeOfExp" => ASTNode::Exp(Exp_::sizeof_exp(
            parser.pos,
            extract_exp_data(parser.pos, parser.get_data("exp".to_string()), "exp", name),
        )),
        "ArrayExp" => ASTNode::Exp(Exp_::array_exp(
            parser.pos,
            extract_explist_data(
                parser.pos,
                parser.get_data("ExpList".to_string()),
                "ExpList",
                name,
            ),
        )),
        "FuncExp" => ASTNode::Exp(Exp_::func_exp(
            parser.pos,
            extract_fieldlist_data(
                parser.pos,
                parser.get_data("params".to_string()),
                "params",
                name,
            ),
            extract_fieldlist_data(
                parser.pos,
                parser.get_data("result".to_string()),
                "result",
                name,
            ),
            extract_stm_data(parser.pos, parser.get_data("Stm".to_string()), "Stm", name),
        )),
        "PostfixExp" => {
            let new_exp =
                extract_exp_data(parser.pos, parser.get_data("exp".to_string()), "exp", name);
            let exp = match parser.get_data("suffix".to_string()) {
                Some(node) => match node {
                    ASTNode::ExpSuffixList(suffix) => {
                        ASTNode::Exp(Exp_::suffix_exp(parser.pos, new_exp, suffix))
                    }
                    _ => ASTNode::Exp(new_exp),
                },
                None => ASTNode::Exp(new_exp),
            };
            match parser.get_data_from_parent_scope("exp".to_string()) {
                Some(node) => match node {
                    ASTNode::ExpList(mut explist) => {
                        explist.push(exp.get_exp_data(parser.pos, "exp", name));
                        ASTNode::ExpList(explist)
                    }
                    ASTNode::Exp(e) => {
                        ASTNode::ExpList(vec![e, exp.get_exp_data(parser.pos, "exp", name)])
                    }
                    _ => ASTNode::ExpList(vec![exp.get_exp_data(parser.pos, "exp", name)]),
                },
                None => ASTNode::ExpList(vec![exp.get_exp_data(parser.pos, "exp", name)]),
            }
        }
        "CallSuffix" | "DotSuffix" | "ArrowSuffix" | "SubscriptSuffix" => {
            let data = if name == "CallSuffix" {
                ExpSuffix_::call_suffix(
                    parser.pos,
                    extract_explist_data(
                        parser.pos,
                        parser.get_data("explist".to_string()),
                        "explist",
                        name,
                    ),
                )
            } else {
                let data =
                    extract_exp_data(parser.pos, parser.get_data("exp".to_string()), "exp", name);
                if name == "DotSuffix" {
                    ExpSuffix_::dot_suffix(parser.pos, data)
                } else if name == "ArrowSuffix" {
                    ExpSuffix_::arrow_suffix(parser.pos, data)
                } else if name == "SubscriptSuffix" {
                    ExpSuffix_::subscript_suffix(parser.pos, data)
                } else {
                    panic!("suffix is not dot nor arrow nor subscript.");
                }
            };
            match parser.get_data_from_parent_scope("suffix".to_string()) {
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
            match parser.get_data_from_parent_scope("op".to_string()) {
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
            let new_exp =
                extract_exp_data(parser.pos, parser.get_data("exp".to_string()), "exp", name);
            match parser.get_data_from_parent_scope("exp".to_string()) {
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
            let handled_exp = match parser.get_data("op".to_string()) {
                Some(node) => {
                    let oplist = node.get_operlist_data(parser.pos, "op", name);
                    if name == "UnaryOpExp" {
                        Exp_::unaryop_exp(
                            parser.pos,
                            oplist,
                            extract_exp_data(
                                parser.pos,
                                parser.get_data("exp".to_string()),
                                "exp",
                                name,
                            ),
                        )
                    } else {
                        Exp_::binop_exp(
                            parser.pos,
                            oplist,
                            extract_explist_data(
                                parser.pos,
                                parser.get_data("exp".to_string()),
                                "exp",
                                name,
                            ),
                        )
                    }
                }
                None => {
                    extract_exp_data(parser.pos, parser.get_data("exp".to_string()), "exp", name)
                }
            };
            match parser.get_data_from_parent_scope("exp".to_string()) {
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
        "ExpList" => match parser.get_data("exp".to_string()) {
            Some(explist) => explist,
            None => ASTNode::ExpList(vec![]),
        },
        "Field" => {
            let new_node = Field_::new(
                parser.pos,
                extract_var_data(parser.pos, parser.get_data("Var".to_string()), "Var", name),
                extract_ty_data(
                    parser.pos,
                    parser.get_data("Type".to_string()),
                    "Type",
                    name,
                ),
            );
            match parser.get_data_from_parent_scope("Field".to_string()) {
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
        "FieldList" => match parser.get_data("Field".to_string()) {
            Some(fieldlist) => fieldlist,
            None => ASTNode::FieldList(vec![]),
        },
        "PublicMembers" | "PrivateMembers" => {
            let new_list = ClassMemberList::new_list(
                extract_declist_data(
                    parser.pos,
                    parser.get_data("DecList".to_string()),
                    "DecList",
                    name,
                ),
                if name == "PublicMembers" {
                    MemberSpecifier::Public
                } else {
                    MemberSpecifier::Private
                },
            );
            match parser.get_data_from_parent_scope("members".to_string()) {
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
        "ClassMemberList" => parser
            .get_data("members".to_string())
            .expect("ClassMemberList"),
        "ClassDec" => ASTNode::Dec(Dec_::class_dec(
            parser.pos,
            extract_string_data(parser.pos, parser.get_data("ID".to_string()), "ID", name),
            extract_classmembers_data(
                parser.pos,
                parser.get_data("ClassMemberList".to_string()),
                "ClassMemberList",
                name,
            ),
            extract_stringlist_data(
                parser.pos,
                parser.get_data("IDList".to_string()),
                "IDList",
                name,
            ),
        )),
        "OperDec" | "FuncDec" | "JsImportDec" => {
            let id = extract_string_data(parser.pos, parser.get_data("ID".to_string()), "ID", name);
            let params = extract_fieldlist_data(
                parser.pos,
                parser.get_data("params".to_string()),
                "params",
                name,
            );
            let result = extract_fieldlist_data(
                parser.pos,
                parser.get_data("result".to_string()),
                "result",
                name,
            );

            if name == "OperDec" || name == "FuncDec" {
                let body = Stm_::compound_stm(
                    parser.pos,
                    extract_stmlist_data(
                        parser.pos,
                        parser.get_data("StmList".to_string()),
                        "StmList",
                        name,
                    ),
                );
                if name == "OperDec" {
                    ASTNode::Dec(Dec_::oper_dec(parser.pos, id, params, result, body))
                } else {
                    ASTNode::Dec(Dec_::func_dec(parser.pos, id, params, result, body))
                }
            } else {
                let module = extract_string_data(
                    parser.pos,
                    parser.get_data("module".to_string()),
                    "module",
                    name,
                );
                let name = extract_string_data(
                    parser.pos,
                    parser.get_data("name".to_string()),
                    "name",
                    name,
                );
                ASTNode::Dec(Dec_::js_import_dec(
                    parser.pos, id, params, result, module, name,
                ))
            }
        }
        "JsExportDec" => ASTNode::Dec(Dec_::js_export_dec(
            parser.pos,
            extract_string_data(parser.pos, parser.get_data("ID".to_string()), "ID", name),
            extract_string_data(
                parser.pos,
                parser.get_data("String".to_string()),
                "String",
                name,
            ),
        )),
        "TemplateDec" => ASTNode::Dec(Dec_::template_dec(
            parser.pos,
            extract_dec_data(parser.pos, parser.get_data("Dec".to_string()), "Dec", name),
            extract_stringlist_data(
                parser.pos,
                parser.get_data("IDList".to_string()),
                "IDList",
                name,
            ),
        )),
        "VarDecInit" | "VarDecNoInit" => {
            let var = extract_var_data(parser.pos, parser.get_data("Var".to_string()), "Var", name);
            let ty = extract_ty_data(
                parser.pos,
                parser.get_data("Type".to_string()),
                "Type",
                name,
            );
            if name == "VarDecInit" {
                ASTNode::Dec(Dec_::var_dec(
                    parser.pos,
                    var,
                    ty,
                    extract_exp_data(parser.pos, parser.get_data("Exp".to_string()), "Exp", name),
                ))
            } else {
                ASTNode::Dec(Dec_::var_dec(
                    parser.pos,
                    var,
                    ty,
                    Box::new(Exp_ {
                        pos: parser.pos,
                        data: ExpData::None,
                    }),
                ))
            }
        }
        "VarDec" => parser.get_data("vardec".to_string()).expect("VarDec"),
        "Dec" => {
            let new_dec =
                extract_dec_data(parser.pos, parser.get_data("dec".to_string()), "dec", name);
            match parser.get_data_from_parent_scope("dec".to_string()) {
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
        "DecList" => parser.get_data("Dec".to_string()).expect("DecList"),
        "Start" => parser.get_data("DecList".to_string()).expect("Start"),
        _ => {
            let _ = writeln!(stderr(), "What is this token: {name}");
            ASTNode::None
        }
    }
}
