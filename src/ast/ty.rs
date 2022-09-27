use super::{exp::ASTExp, field::FieldList};

pub type TypeList = Vec<Box<Type_>>;
pub type Type = Box<Type_>;

#[derive(Clone, Debug)]
pub struct Type_ {
    pub pos: (usize, usize),
    pub data: TypeData,
}

#[derive(Clone, Debug)]
pub enum TypeData {
    Void,
    Int,
    Short,
    Char,
    Bool,
    Real,
    Name(String),
    Array(Type, ASTExp),
    Pointer(Type),
    Template(String, Vec<Type>),
    Func(FieldList, Type),
    None,
}

impl Type_ {
    pub fn void_type(pos: (usize, usize)) -> Type {
        Box::new(Type_ {
            pos,
            data: TypeData::Void,
        })
    }
    pub fn int_type(pos: (usize, usize)) -> Type {
        Box::new(Type_ {
            pos,
            data: TypeData::Int,
        })
    }
    pub fn short_type(pos: (usize, usize)) -> Type {
        Box::new(Type_ {
            pos,
            data: TypeData::Short,
        })
    }
    pub fn real_type(pos: (usize, usize)) -> Type {
        Box::new(Type_ {
            pos,
            data: TypeData::Real,
        })
    }
    pub fn char_type(pos: (usize, usize)) -> Type {
        Box::new(Type_ {
            pos,
            data: TypeData::Char,
        })
    }
    pub fn bool_type(pos: (usize, usize)) -> Type {
        Box::new(Type_ {
            pos,
            data: TypeData::Bool,
        })
    }
    pub fn name_type(pos: (usize, usize), name: String) -> Type {
        Box::new(Type_ {
            pos,
            data: TypeData::Name(name),
        })
    }
    pub fn array_type(pos: (usize, usize), ty: Type, size: ASTExp) -> Type {
        Box::new(Type_ {
            pos,
            data: TypeData::Array(ty, size),
        })
    }
    pub fn pointer_type(pos: (usize, usize), ty: Type) -> Type {
        Box::new(Type_ {
            pos,
            data: TypeData::Pointer(ty),
        })
    }
    pub fn template_type(pos: (usize, usize), name: String, ty_params: Vec<Type>) -> Type {
        Box::new(Type_ {
            pos,
            data: TypeData::Template(name, ty_params),
        })
    }
    pub fn func_type(pos: (usize, usize), params: FieldList, result: Type) -> Type {
        Box::new(Type_ {
            pos,
            data: TypeData::Func(params, result),
        })
    }
}
