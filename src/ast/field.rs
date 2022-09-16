use super::ty;

type Field = Box<Field_>;

pub struct Field_ {
    pos: (i32, i32),
    var: String,
    ty: ty::Type,
    escape: bool,
}

pub type FieldList = Vec<Field>;
