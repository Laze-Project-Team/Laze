use std::{
    io::{stderr, Write},
    mem::swap,
};

use crate::ast::{
    exp::{ASTExpData, ASTExpList},
    field::{FieldData, FieldList},
    suffix::SuffixData,
    ty::{Type, TypeData, TypeList, Type_},
    var::{Var, VarData},
};

use super::{
    entry_map::EnvEntry,
    laze_type::{LazeType, LazeTypeList, LazeType_},
    semantic_param::SemanticParam,
    trans_dec::trans_dec,
};

pub fn trans_ty(ty: &Type, semantic_data: &mut SemanticParam) -> LazeType {
    match &ty.data {
        TypeData::Void => LazeType_::void_type(),
        TypeData::Int => LazeType_::int_type(),
        TypeData::Bool => LazeType_::bool_type(),
        TypeData::Char => LazeType_::char_type(),
        TypeData::Real => LazeType_::real_type(),
        TypeData::Short => LazeType_::short_type(),
        TypeData::Name(name) => LazeType_::class_type(name.clone(), 0),
        TypeData::Array(ty, size) => {
            let array_size = match &size.data {
                ASTExpData::Int(int) => int.parse::<i32>().unwrap(),
                ASTExpData::Short(int) => int.parse::<i32>().unwrap(),
                _ => {
                    let _ = writeln!(
                        stderr(),
                        "The size of this array is not a constant: {:?}",
                        size.pos
                    );
                    0
                }
            };
            LazeType_::array_type(trans_ty(&ty, semantic_data), array_size)
        }
        TypeData::Pointer(ty) => LazeType_::pointer_type(trans_ty(&ty, semantic_data)),
        TypeData::Func(fieldlist, result) => {
            let mut param_types = vec![];
            for field in fieldlist {
                if let FieldData::Field(_, ty) = &field.data {
                    param_types.push(trans_ty(&ty, semantic_data));
                }
            }
            LazeType_::func_type(param_types, trans_ty(&result, semantic_data), 0)
        }
        TypeData::Template(name, type_params) => {
            let type_params_lazetype = trans_tylist(type_params.clone(), semantic_data);
            let template_entry = semantic_data.tenv.get_data_clone(name);

            if let Some(entry) = template_entry {
                if let EnvEntry::Template(
                    original_dec,
                    template_map,
                    mut template_venv,
                    type_params_str,
                ) = entry
                {
                    let specific_template = template_map.get_data(&type_params_lazetype);
                    if let Some(template) = specific_template {
                        if let EnvEntry::Class(_, _, class_size) = template {
                            return LazeType_::template_type(
                                name.clone(),
                                type_params_lazetype,
                                class_size.clone(),
                            );
                        }
                    } else {
                        if type_params.len() == type_params_str.len() {
                            for (index, param) in type_params.iter().enumerate() {
                                let poly_entry = EnvEntry::Poly(trans_ty(param, semantic_data));
                                semantic_data
                                    .tenv
                                    .add_data(type_params_str[index].clone(), poly_entry);
                            }
                        } else if type_params.len() < type_params_str.len() {
                            let _ = writeln!(
                                stderr(),
                                "Type parameter is missing: {:?}",
                                type_params_str[type_params.len()]
                            );
                            return LazeType_::none_type();
                        } else if type_params.len() > type_params_str.len() {
                            let _ = writeln!(
                                stderr(),
                                "There are too many type parameters: {:?}",
                                type_params[type_params_str.len()]
                            );
                            return LazeType_::none_type();
                        }
                        swap(&mut semantic_data.venv, &mut template_venv);
                        let _ = trans_dec(&original_dec, None, semantic_data);
                        swap(&mut semantic_data.venv, &mut template_venv);
                        if type_params.len() == type_params_str.len() {
                            for type_str in type_params_str.iter() {
                                semantic_data.tenv.remove_data(type_str);
                            }
                        }
                        let specific_template = template_map.get_data(&type_params_lazetype);
                        if let Some(template) = specific_template {
                            if let EnvEntry::Class(_, _, class_size) = template {
                                return LazeType_::template_type(
                                    name.clone(),
                                    type_params_lazetype,
                                    class_size.clone(),
                                );
                            }
                        }
                        return LazeType_::none_type();
                    }
                }
            }
            LazeType_::none_type()
        }
        TypeData::None => {
            let _ = writeln!(stderr(), "Type is not valid: {:?}", ty.pos);
            LazeType_::none_type()
        }
    }
}

pub fn trans_tylist(list: TypeList, semantic_data: &mut SemanticParam) -> LazeTypeList {
    let mut result = vec![];
    for ty in list {
        result.push(trans_ty(&ty, semantic_data));
    }
    result
}

pub fn trans_params(list: &FieldList, semantic_data: &mut SemanticParam) -> LazeTypeList {
    let mut result = vec![];
    for field in list {
        match &field.data {
            FieldData::Field(var, var_ty) => {
                let (_, new_var_ty, _) = trans_var_ty(var, var_ty);
                let new_var_lazetype = trans_ty(&new_var_ty, semantic_data);
                semantic_data
                    .frame
                    .last_mut()
                    .unwrap()
                    .alloc_param(&new_var_lazetype);
                result.push(new_var_lazetype);
            }
            FieldData::None => {}
        }
    }
    result
}

pub fn trans_result<'a>(
    pos: (usize, usize),
    result_list: &'a FieldList,
    semantic_data: &mut SemanticParam,
) -> (Option<&'a Var>, LazeType) {
    let return_field = if result_list.len() > 1 {
        let _ = writeln!(
            stderr(),
            "Laze does not support multiple return values: {:?}",
            pos
        );
        &result_list[0].data
    } else if result_list.len() == 1 {
        &result_list[0].data
    } else {
        &FieldData::None
    };
    match return_field {
        FieldData::Field(var, ty) => {
            let (new_var, new_var_ty, object_explist) = trans_var_ty(var, ty);
            (Some(new_var), trans_ty(&new_var_ty, semantic_data))
        }
        FieldData::None => (None, LazeType_::void_type()),
    }
}

pub fn trans_var_ty<'a>(var: &'a Var, var_ty: &Type) -> (&'a Var, Type, Option<&'a ASTExpList>) {
    let mut object_explist = None;
    match &var.data {
        VarData::Pointer(pointer_var) => (
            pointer_var,
            Type_::pointer_type(var_ty.pos, var_ty.clone()),
            object_explist,
        ),
        VarData::SuffixVar(suffix_var, suffixlist) => {
            let mut temp_var_ty = var_ty.clone();
            if suffixlist.len() > 0 {
                match &suffixlist[0].data {
                    SuffixData::Subscript(index) => {
                        temp_var_ty =
                            Type_::array_type(temp_var_ty.pos, temp_var_ty, index.clone());
                    }
                    SuffixData::Call(args) => {
                        object_explist = Some(args);
                    }
                    _ => {}
                }
            }
            for suffix in suffixlist {
                match &suffix.data {
                    SuffixData::Subscript(index) => {
                        let _ = writeln!(
                            stderr(),
                            "Warning: Avoid using N-Dimensional arrays: {:?}",
                            var.pos
                        );
                        temp_var_ty =
                            Type_::array_type(temp_var_ty.pos, temp_var_ty, index.clone());
                    }
                    SuffixData::Call(_) => {
                        let _ = writeln!(stderr(), "Unknown declaration: {:?}", var.pos);
                    }
                    _ => {}
                }
            }
            (suffix_var, temp_var_ty, object_explist)
        }
        _ => (var, var_ty.clone(), object_explist),
    }
}
