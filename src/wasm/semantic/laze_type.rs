use std::io::{stderr, Write};

use crate::{
    ast::ty::TypeList,
    wasm::il::{
        exp::{Exp, Exp_},
        util::{WasmType, WasmTypeList},
    },
};

pub type LazeTypeList = Vec<LazeType>;
pub type LazeType = Box<LazeType_>;

#[derive(Clone, Debug, PartialEq)]
pub struct LazeType_ {
    pub size: i32,
    pub escape: bool,
    pub data: LazeTypeData,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LazeTypeData {
    Void,
    Int,
    Short,
    Real,
    Bool,
    Char,
    Class(String),
    Template(String, LazeTypeList, TypeList),
    Array(LazeType, i32),
    Pointer(LazeType),
    Func(LazeTypeList, LazeType, i32),
    None,
}

impl LazeType_ {
    pub fn to_wasm_type(&self) -> WasmType {
        match self.data {
            LazeTypeData::Void => WasmType::None,
            LazeTypeData::Int => WasmType::I64,
            LazeTypeData::Bool => WasmType::I32,
            LazeTypeData::Char => WasmType::I32,
            LazeTypeData::Short => WasmType::I32,
            LazeTypeData::Real => WasmType::F64,
            LazeTypeData::Array(_, _) => WasmType::I32,
            LazeTypeData::Class(_) => WasmType::I32,
            LazeTypeData::Func(_, _, _) => WasmType::I32,
            LazeTypeData::Pointer(_) => WasmType::I32,
            LazeTypeData::Template(_, _, _) => WasmType::I32,
            LazeTypeData::None => WasmType::None,
        }
    }
    pub fn list_to_wasm_type(list: &LazeTypeList) -> WasmTypeList {
        let mut result = vec![];
        for ty in list {
            result.push(ty.to_wasm_type());
        }
        result
    }

    pub fn none_type() -> LazeType {
        Box::new(LazeType_ {
            size: 0,
            escape: false,
            data: LazeTypeData::None,
        })
    }
    pub fn void_type() -> LazeType {
        Box::new(LazeType_ {
            size: 0,
            escape: false,
            data: LazeTypeData::Void,
        })
    }
    pub fn int_type() -> LazeType {
        Box::new(LazeType_ {
            size: 8,
            escape: false,
            data: LazeTypeData::Int,
        })
    }
    pub fn real_type() -> LazeType {
        Box::new(LazeType_ {
            size: 8,
            escape: false,
            data: LazeTypeData::Real,
        })
    }
    pub fn char_type() -> LazeType {
        Box::new(LazeType_ {
            size: 4,
            escape: false,
            data: LazeTypeData::Char,
        })
    }
    pub fn short_type() -> LazeType {
        Box::new(LazeType_ {
            size: 4,
            escape: false,
            data: LazeTypeData::Short,
        })
    }
    pub fn bool_type() -> LazeType {
        Box::new(LazeType_ {
            size: 4,
            escape: false,
            data: LazeTypeData::Bool,
        })
    }
    pub fn class_type(name: String, size: i32) -> LazeType {
        Box::new(LazeType_ {
            size,
            escape: true,
            data: LazeTypeData::Class(name),
        })
    }
    pub fn array_type(ty: LazeType, size: i32) -> LazeType {
        Box::new(LazeType_ {
            size: ty.size * size,
            escape: true,
            data: LazeTypeData::Array(ty, size),
        })
    }
    pub fn pointer_type(ty: LazeType) -> LazeType {
        Box::new(LazeType_ {
            size: 4,
            escape: false,
            data: LazeTypeData::Pointer(ty),
        })
    }
    pub fn template_type(
        name: String,
        lazetype_params: LazeTypeList,
        type_params: TypeList,
        size: i32,
    ) -> LazeType {
        Box::new(LazeType_ {
            size,
            escape: true,
            data: LazeTypeData::Template(name, lazetype_params, type_params),
        })
    }
    pub fn func_type(params: LazeTypeList, result: LazeType, type_index: i32) -> LazeType {
        Box::new(LazeType_ {
            size: 4,
            escape: false,
            data: LazeTypeData::Func(params, result, type_index),
        })
    }
}

pub fn comp_type_binop<'a>(
    left: LazeType,
    left_exp: Exp,
    right: LazeType,
    right_exp: Exp,
) -> Option<(LazeType, Exp, Exp)> {
    if left == right {
        Some((left, left_exp, right_exp))
    } else {
        let left_wasm_type = left.to_wasm_type();
        let right_wasm_type = right.to_wasm_type();
        match left.data {
            LazeTypeData::Int => match right.data {
                LazeTypeData::Short => {
                    Some((left, left_exp, Exp_::convert_exp(left_wasm_type, right_exp)))
                }
                LazeTypeData::Real => Some((
                    right,
                    Exp_::convert_exp(right_wasm_type, left_exp),
                    right_exp,
                )),
                _ => None,
            },
            LazeTypeData::Real => match right.data {
                LazeTypeData::Int | LazeTypeData::Short => {
                    Some((left, left_exp, Exp_::convert_exp(left_wasm_type, right_exp)))
                }
                _ => None,
            },
            LazeTypeData::Short => match right.data {
                LazeTypeData::Int | LazeTypeData::Real => Some((
                    right,
                    Exp_::convert_exp(right_wasm_type, left_exp),
                    right_exp,
                )),
                LazeTypeData::Char => {
                    let _ = writeln!(
                        stderr(),
                        "Warning: conversion from char to short is dangerous."
                    );
                    Some((left, left_exp, right_exp))
                }
                _ => None,
            },
            LazeTypeData::Char => match right.data {
                LazeTypeData::Short => {
                    let _ = writeln!(
                        stderr(),
                        "Warning: conversion from short to char is dangerous."
                    );
                    Some((left, left_exp, right_exp))
                }
                _ => None,
            },
            _ => None,
        }
    }
}
