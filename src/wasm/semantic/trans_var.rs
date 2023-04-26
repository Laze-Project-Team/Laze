use std::io::{stderr, Write};

use crate::{
    ast::{
        dec::MemberSpecifier,
        suffix::{ASTExpSuffixList, ASTExpSuffix_, SuffixData},
        ty::TypeData,
        var::{Var, VarData, Var_},
    },
    error_handler::error::CompileError_,
    wasm::{
        frame::frame::FrameType,
        il::{
            exp::{Exp, Exp_},
            util::{BinOper, WasmExpTy},
        },
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
                    let checked_var = check_member(var, semantic_data);
                    if let Some(checked_var_exists) = checked_var {
                        return trans_right_var(&checked_var_exists, semantic_data);
                    } else {
                        semantic_data.errors.push(CompileError_::new_error(
                            var.pos,
                            format_args!("{:?} is not a variable.", name).to_string(),
                        ));
                        WasmExpTy::new_exp(LazeType_::none_type(), Exp_::none_exp())
                    }
                }
            } else {
                let checked_var = check_member(var, semantic_data);
                if let Some(checked_var_exists) = checked_var {
                    return trans_right_var(&checked_var_exists, semantic_data);
                } else {
                    semantic_data.errors.push(CompileError_::new_error(
                        var.pos,
                        format_args!("Could not find variable {:?}", name).to_string(),
                    ));
                    WasmExpTy::new_exp(LazeType_::none_type(), Exp_::none_exp())
                }
            }
        }
        VarData::None => {
            semantic_data.errors.push(CompileError_::new_error(
                var.pos,
                format_args!("Not a variable.").to_string(),
            ));
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
                semantic_data.errors.push(CompileError_::new_error(
                    var.pos,
                    format_args!("{} is not a variable nor a function.", name).to_string(),
                ));
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
                        LazeTypeData::Template(_class_name, _type_param, _) => {
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
                        semantic_data.errors.push(CompileError_::new_error(
                            var.pos,
                            format_args!("Cannot call {:?} because it is not a function", name)
                                .to_string(),
                        ));
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
                        semantic_data.errors.push(CompileError_::new_error(
                            var.pos,
                            format_args!(
                                "To use the arrow operator, {:?} needs to be a pointer.",
                                name
                            )
                            .to_string(),
                        ));
                    }
                },
            }
        }
        WasmExpTy::new_exp(ty, result_exp)
    } else {
        let checked_var = check_member(var, semantic_data);
        if let Some(checked_var_exists) = checked_var {
            return trans_suffix_var_to_addr(&checked_var_exists, suffixlist, semantic_data);
        } else {
            semantic_data.errors.push(CompileError_::new_error(
                var.pos,
                format_args!("Could not find a variable or function named {:?}", name).to_string(),
            ));
            WasmExpTy::none()
        }
    }
}

// TODO: Make cleaner / fix bugs(haven't found them yet)
pub fn check_member<'a>(var: &'a Var, semantic_data: &mut SemanticParam) -> Option<Var> {
    let var_name = get_var_name(var);
    let frame = semantic_data.current_frame();
    if let Some(frame_exists) = frame {
        if let FrameType::Method(name, parent_class) = &frame_exists.data {
            let members_map = match &parent_class.data {
                TypeData::Name(class) => {
                    if let Some(EnvEntry::Class(_, members_map, _)) =
                        semantic_data.tenv.get_data(class)
                    {
                        members_map
                    } else {
                        return None;
                    }
                }
                TypeData::Template(template_name, type_param) => {
                    if let Some(EnvEntry::Template(_, template_map, _, _)) =
                        semantic_data.tenv.get_data(template_name)
                    {
                        if let Some(EnvEntry::Class(_, members_map, _)) =
                            template_map.get_data(type_param)
                        {
                            members_map
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
                _ => {
                    return None;
                }
            };
            if let Some(member_entry) = members_map.get_data(&var_name) {
                match member_entry {
                    EnvEntry::Member(specifier, _, _) | EnvEntry::Method(specifier, _, _, _) => {
                        if let MemberSpecifier::Private = specifier {
                            return None;
                        }
                    }
                    _ => {
                        return None;
                    }
                }
            } else {
                return None;
            }
            match &var.data {
                VarData::Simple(name) => Some(Var_::suffix_var(
                    var.pos,
                    Var_::simple_var(var.pos, "self".to_string()),
                    vec![ASTExpSuffix_::arrow_suffix(var.pos, name.clone())],
                )),
                VarData::Pointer(pointer_var) => Some(Var_::pointer_var(
                    var.pos,
                    if let Some(member) = check_member(pointer_var, semantic_data) {
                        member
                    } else {
                        return None;
                    },
                )),
                VarData::SuffixVar(base_var, suffix_list) => match &base_var.data {
                    VarData::Simple(name) => {
                        let mut new_suffix_list =
                            vec![ASTExpSuffix_::arrow_suffix(var.pos, name.clone())];
                        new_suffix_list.append(&mut suffix_list.clone());
                        Some(Var_::suffix_var(
                            var.pos,
                            Var_::simple_var(var.pos, "self".to_string()),
                            new_suffix_list,
                        ))
                    }
                    VarData::Pointer(pointer_var) => {
                        let mut new_suffix_list =
                            vec![ASTExpSuffix_::arrow_suffix(var.pos, name.clone())];
                        new_suffix_list.append(&mut suffix_list.clone());
                        Some(Var_::suffix_var(
                            var.pos,
                            Var_::pointer_var(
                                var.pos,
                                if let Some(member) = check_member(&pointer_var, semantic_data) {
                                    member
                                } else {
                                    return None;
                                },
                            ),
                            new_suffix_list,
                        ))
                    }
                    VarData::SuffixVar(var, extra_suffix_list) => {
                        let mut new_suffix_list =
                            vec![ASTExpSuffix_::arrow_suffix(var.pos, name.clone())];
                        new_suffix_list.append(&mut extra_suffix_list.clone());
                        new_suffix_list.append(&mut suffix_list.clone());
                        Some(Var_::suffix_var(
                            var.pos,
                            Var_::simple_var(var.pos, "self".to_string()),
                            new_suffix_list,
                        ))
                    }
                    VarData::None => {
                        return None;
                    }
                },
                VarData::None => {
                    return None;
                }
            }
        } else {
            return None;
        }
    } else {
        return None;
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
    if let Some(member_entry_exists) = member_entry {
        match member_entry_exists {
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
    } else {
        let _ = writeln!(
            stderr(),
            "Could not find member {:?} in class {:?}: {:?}",
            field_name,
            class_name,
            var_pos
        );
        return (LazeType_::none_type(), Exp_::none_exp());
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
                semantic_data.errors.push(CompileError_::new_error(
                    var_pos,
                    format_args!("{:?} is not a class", name).to_string(),
                ));
                (LazeType_::none_type(), Exp_::none_exp())
            }
        }
        LazeTypeData::Template(name, _, type_param) => {
            let template_entry = semantic_data.tenv.get_data(&name);
            if let Some(EnvEntry::Template(_, specific, _, _)) = template_entry {
                let class_entry = specific.get_data(type_param);
                if let Some(EnvEntry::Class(_, members, _)) = class_entry {
                    get_member_of_class(&ty, result_exp, members, &field, &name, var_pos)
                } else {
                    semantic_data.errors.push(CompileError_::new_error(
                        var_pos,
                        format_args!("{:?} is not a class", name).to_string(),
                    ));
                    (LazeType_::none_type(), Exp_::none_exp())
                }
            } else {
                semantic_data.errors.push(CompileError_::new_error(
                    var_pos,
                    format_args!("{:?} is not a template", name).to_string(),
                ));
                (LazeType_::none_type(), Exp_::none_exp())
            }
        }
        _ => {
            semantic_data.errors.push(CompileError_::new_error(
                var_pos,
                format_args!(
                    "Cannot take field {:?} of {:?} because it is a non-class type",
                    field, name
                )
                .to_string(),
            ));
            (LazeType_::none_type(), Exp_::none_exp())
        }
    }
}
