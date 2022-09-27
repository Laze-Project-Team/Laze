use crate::{
    ast::{
        exp::ASTExpData,
        field::FieldData,
        ty::{Type, TypeData, TypeList},
    },
    wasm::il::util::{WasmType, WasmTypeList},
};

pub fn trans_ty(ty: Type) -> LazeType {
    match ty.data {
        TypeData::Void => LazeType_::void_type(),
        TypeData::Int => LazeType_::int_type(),
        TypeData::Bool => LazeType_::bool_type(),
        TypeData::Char => LazeType_::char_type(),
        TypeData::Real => LazeType_::real_type(),
        TypeData::Short => LazeType_::short_type(),
        TypeData::Name(name) => LazeType_::class_type(name, 0),
        TypeData::Array(ty, size) => {
            let array_size = match size.data {
                ASTExpData::Int(int) => int.parse::<i32>().unwrap(),
                ASTExpData::Short(int) => int.parse::<i32>().unwrap(),
                _ => {
                    panic!("The size of this array is not a constant: {:?}", size.pos);
                }
            };
            LazeType_::array_type(trans_ty(ty), array_size)
        }
        TypeData::Pointer(ty) => LazeType_::pointer_type(trans_ty(ty)),
        TypeData::Func(fieldlist, result) => {
            let mut param_types = vec![];
            for field in fieldlist {
                if let FieldData::Field(_, ty) = field.data {
                    param_types.push(trans_ty(ty));
                }
            }
            LazeType_::func_type(param_types, trans_ty(result), 0)
        }
        TypeData::Template(name, type_params) => {
            LazeType_::template_type(name, trans_tylist(type_params), 0)
        }
        TypeData::None => {
            panic!("Type is not valid: {:?}", ty.pos);
        }
    }
}

pub fn trans_tylist(list: TypeList) -> LazeTypeList {
    let mut result = vec![];
    for ty in list {
        result.push(trans_ty(ty));
    }
    result
}

pub type LazeTypeList = Vec<LazeType>;
pub type LazeType = Box<LazeType_>;

#[derive(Clone, PartialEq)]
pub struct LazeType_ {
    pub size: i32,
    pub escape: bool,
    pub data: LazeTypeData,
}

#[derive(Clone, PartialEq)]
pub enum LazeTypeData {
    Void,
    Int,
    Short,
    Real,
    Bool,
    Char,
    Class(String),
    Template(String, LazeTypeList),
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
            LazeTypeData::Template(_, _) => WasmType::I32,
            LazeTypeData::None => WasmType::None,
        }
    }
    pub fn list_to_wasm_type(list: LazeTypeList) -> WasmTypeList {
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
    pub fn template_type(name: String, type_params: LazeTypeList, size: i32) -> LazeType {
        Box::new(LazeType_ {
            size,
            escape: true,
            data: LazeTypeData::Template(name, type_params),
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
