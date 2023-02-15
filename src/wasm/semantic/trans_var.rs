use std::io::{stderr, Write};

use crate::{
    ast::{
        dec::MemberSpecifier,
        suffix::{ASTExpSuffixList, SuffixData},
        var::{Var, VarData},
    },
    wasm::il::{
        exp::{Exp, Exp_},
        util::{BinOper, WasmExpTy},
    },
};

use super::{
    entry_map::{EntryMap, EnvEntry},
    laze_type::{LazeType, LazeTypeData, LazeType_},
    semantic_param::SemanticParam,
    trans_exp::{trans_access_to_exp, trans_exp, trans_explist},
};

pub fn trans_right_var(var: &Var, semantic_data: &mut SemanticParam) -> WasmExpTy {
    match &var.data {
        VarData::SuffixVar(var, suffixlist) => {
            trans_suffix_var_to_addr(var, suffixlist, semantic_data)
        }
        VarData::Pointer(pointer_var) => trans_right_var(pointer_var, semantic_data),
        VarData::Simple(name) => {
            let var_entry = semantic_data.venv.get_data(name);
            if let Some(entry) = var_entry {
                if let EnvEntry::Var(ty, access) = entry {
                    WasmExpTy::new_exp(ty.clone(), trans_access_to_exp(access, var, name))
                } else {
                    let _ = writeln!(stderr(), "{:?} is not a variable: {:?}", name, var.pos);
                    WasmExpTy::new_exp(LazeType_::none_type(), Exp_::none_exp())
                }
            } else {
                let _ = writeln!(
                    stderr(),
                    "Could not find variable {:?}: {:?}",
                    name,
                    var.pos
                );
                WasmExpTy::new_exp(LazeType_::none_type(), Exp_::none_exp())
            }
        }
        VarData::None => {
            let _ = writeln!(stderr(), "Not a variable: {:?}", var.pos);
            WasmExpTy::new_exp(LazeType_::none_type(), Exp_::none_exp())
        }
    }
}

// Returns address of the var
// because all suffix vars are escaped
pub fn trans_suffix_var_to_addr(
    var: &Var,
    suffixlist: &ASTExpSuffixList,
    semantic_data: &mut SemanticParam,
) -> WasmExpTy {
    let name = &get_var_name(&var);

    if let Some(var_entry) = semantic_data.venv.get_data(name) {
        let (mut ty, mut result_exp) = match var_entry {
            EnvEntry::Var(ty, access) => (ty.clone(), trans_access_to_exp(access, &var, name)),
            // the function was declared with func: <ID> () => () {<body>}
            // this function returns an address
            EnvEntry::Func(index, params, return_ty) => (
                // check params type
                LazeType_::func_type(params.clone(), return_ty.clone(), -(index + 1)),
                Exp_::none_exp(), // Exp_::call_exp(
                                  //     return_ty.to_wasm_type(),
                                  //     *index,
                                  //     get_args_from_suffixlist(suffixlist, semantic_data, var.pos),
                                  //     None,
                                  // ),
            ),
            _ => {
                let _ = writeln!(
                    stderr(),
                    "{} is not a variable nor a function: {:?}",
                    name,
                    var.pos
                );
                (LazeType_::none_type(), Exp_::none_exp())
            }
        };
        for suffix in suffixlist {
            match &suffix.data {
                SuffixData::Subscript(index) => {
                    match &ty.data {
                        LazeTypeData::Array(element_ty, _) => {
                            ty = element_ty.clone();
                        }
                        LazeTypeData::Class(_class_name) => {
                            // operator overloading for subscript
                        }
                        LazeTypeData::Template(_class_name, _type_param) => {
                            // operator overloading for subscript
                        }
                        _ => {}
                    }
                    result_exp = Exp_::add_addr_exp(
                        result_exp,
                        Exp_::mul_addr_exp(
                            Exp_::consti32_exp(ty.size),
                            trans_exp(index, semantic_data).exp("".to_string()),
                        ),
                    )
                }
                SuffixData::Dot(field) => {
                    (ty, result_exp) =
                        trans_dot_var(field, &ty, result_exp, semantic_data, var.pos, name)
                }
                SuffixData::Call(explist) => {
                    // class methods will be function variables
                    if let LazeTypeData::Func(_typelist, return_type, type_index) = ty.data {
                        // check param type
                        ty = return_type.clone();
                        if type_index < 0 {
                            result_exp = Exp_::call_exp(
                                ty.to_wasm_type(),
                                -(type_index + 1),
                                trans_explist(explist, semantic_data).1,
                                None,
                            )
                        } else {
                            result_exp = Exp_::call_indirect_exp(
                                ty.to_wasm_type(),
                                result_exp,
                                type_index,
                                trans_explist(explist, semantic_data).1,
                            );
                        }
                    } else {
                        let _ = writeln!(
                            stderr(),
                            "Cannot call {:?} because it is not a function: {:?}",
                            name,
                            var.pos
                        );
                    }
                }
                SuffixData::Arrow(field) => match ty.data {
                    LazeTypeData::Pointer(pointer_ty) => {
                        ty = pointer_ty;
                        result_exp = Exp_::load_exp(ty.to_wasm_type(), result_exp);
                        (ty, result_exp) =
                            trans_dot_var(field, &ty, result_exp, semantic_data, var.pos, name);
                    }
                    _ => {
                        let _ = writeln!(
                            stderr(),
                            "To use the arrow operator, {:?} needs to be a pointer: {:?}",
                            name,
                            var.pos
                        );
                    }
                },
            }
        }
        WasmExpTy::new_exp(ty, result_exp)
    } else {
        let _ = writeln!(
            stderr(),
            "Could not find a variable or function named {:?}",
            name
        );
        WasmExpTy::none()
    }
}

pub fn get_var_name(var: &Var) -> String {
    match &var.data {
        VarData::Simple(name) => name.clone(),
        VarData::Pointer(var) => get_var_name(&var),
        VarData::SuffixVar(var, _) => get_var_name(&var),
        VarData::None => {
            let _ = writeln!(stderr(), "Variable does not exist: {:?}", var.pos);
            "".to_string()
        }
    }
}

fn get_member_of_class(
    ty: &LazeType,
    result_exp: Exp,
    members: &EntryMap,
    field_name: &String,
    class_name: &String,
    var_pos: (usize, usize),
) -> (LazeType, Exp) {
    let member_entry = members.get_data(field_name);
    match member_entry.expect(
        format_args!(
            "Could not find member {:?} in class {:?}: {:?}",
            field_name, class_name, var_pos
        )
        .as_str()
        .unwrap(),
    ) {
        EnvEntry::Member(specifier, member_ty, offset) => {
            if let MemberSpecifier::Public = specifier {
                (
                    member_ty.clone(),
                    Exp_::binop_exp(
                        ty.to_wasm_type(),
                        BinOper::Add,
                        result_exp,
                        Exp_::consti32_exp(*offset),
                    ),
                )
            } else {
                let _ = writeln!(
                    stderr(),
                    "Cannot access a member that is not public: {:?}",
                    var_pos
                );
                (LazeType_::none_type(), Exp_::none_exp())
            }
        }
        _ => {
            let _ = writeln!(
                stderr(),
                "{} is not a member of {}.",
                field_name,
                class_name
            );
            (LazeType_::none_type(), Exp_::none_exp())
        }
    }
}

fn trans_dot_var(
    field: &String,
    ty: &LazeType,
    result_exp: Exp,
    semantic_data: &mut SemanticParam,
    var_pos: (usize, usize),
    name: &String,
) -> (LazeType, Exp) {
    match &ty.data {
        LazeTypeData::Class(name) => {
            let class_entry = semantic_data.tenv.get_data(&name);
            if let Some(EnvEntry::Class(_, members, _)) = class_entry {
                get_member_of_class(&ty, result_exp, &members, &field, &name, var_pos)
            } else {
                let _ = writeln!(stderr(), "{:?} is not a class: {:?}", name, var_pos);
                (LazeType_::none_type(), Exp_::none_exp())
            }
        }
        LazeTypeData::Template(name, type_param) => {
            let template_entry = semantic_data.tenv.get_data(&name);
            if let Some(EnvEntry::Template(_, specific, _, _)) = template_entry {
                let class_entry = specific.get_data(type_param);
                if let Some(EnvEntry::Class(_, members, _)) = class_entry {
                    get_member_of_class(&ty, result_exp, members, &field, &name, var_pos)
                } else {
                    let _ = writeln!(stderr(), "{:?} is not a class: {:?}", name, var_pos);
                    (LazeType_::none_type(), Exp_::none_exp())
                }
            } else {
                let _ = writeln!(stderr(), "{:?} is not a template: {:?}", name, var_pos);
                (LazeType_::none_type(), Exp_::none_exp())
            }
        }
        _ => {
            let _ = writeln!(
                stderr(),
                "Cannot take field {:?} of {:?} because it is a non-class type: {:?}",
                field,
                name,
                var_pos
            );
            (LazeType_::none_type(), Exp_::none_exp())
        }
    }
}
