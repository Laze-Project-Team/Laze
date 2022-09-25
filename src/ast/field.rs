use super::{ty, var::Var};

pub type Field = Box<Field_>;
pub type FieldList = Vec<Field>;

#[derive(Clone, Debug)]
pub struct Field_ {
    pub pos: (usize, usize),
    pub data: FieldData,
}

#[derive(Clone, Debug)]
pub enum FieldData {
    Field(Var, ty::Type),
    None,
}

impl Field_ {
    pub fn new(pos: (usize, usize), var: Var, ty: ty::Type) -> Field {
        Box::new(Field_ {
            pos,
            data: FieldData::Field { 0: var, 1: ty },
        })
    }
}
