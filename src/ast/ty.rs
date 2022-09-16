use super::field;

pub type Type = Box<Type_>;

pub struct Type_ {
    pos: (i32, i32),
    data: TypeData,
}

pub enum TypeData {
    Void,
    Name(String),
    Array(Type, i32),
    Pointer(Type),
    Template(String, Vec<Type>),
    Func(field::FieldList, Type),
}

impl Type_ {
    fn VoidType(pos: (i32, i32)) -> Type {
        Box::new(Type_ {
            pos,
            data: TypeData::Void,
        })
    }
    fn NameType(pos: (i32, i32), name: &str) -> Type {
        Box::new(Type_ {
            pos,
            data: TypeData::Name(name.to_string()),
        })
    }
    fn ArraTypey(pos: (i32, i32), ty: Type, size: i32) -> Type {
        Box::new(Type_ {
            pos,
            data: TypeData::Array(ty, size),
        })
    }
    fn PoinTypeter(pos: (i32, i32), ty: Type) -> Type {
        Box::new(Type_ {
            pos,
            data: TypeData::Pointer(ty),
        })
    }
    fn TempTypelate(pos: (i32, i32), name: String, tyParams: Vec<Type>) -> Type {
        Box::new(Type_ {
            pos,
            data: TypeData::Template(name, tyParams),
        })
    }
    fn FuncType(pos: (i32, i32), params: field::FieldList, result: Type) -> Type {
        Box::new(Type_ {
            pos,
            data: TypeData::Func(params, result),
        })
    }
}
