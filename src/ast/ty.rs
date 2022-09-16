use super::field;

pub type Type = Box<Type_>;

pub struct Type_ {
    pos: i32,
    data: TypeData,
}

pub enum TypeData {
    Void,
    Name(String),
    Array(Type, i32),
    Pointer(Type),
    Template(String, Type),
    Func(field::FieldList, Type),
}
