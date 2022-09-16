use super::{exp, field, op, stm, ty};

pub type Dec = Box<Dec_>;
pub type DecList = Vec<Dec>;

pub struct Dec_ {
    pos: (i32, i32),
    data: DecData,
}

pub enum DecData {
    Func(String, field::FieldList, ty::Type, stm::Stm),
    Oper(op::Oper, field::FieldList, ty::Type, stm::Stm),
    JsImport(String, field::FieldList, ty::Type, String, String),
    JsExport(String, String),

    Var(String, ty::Type, exp::Exp, bool),
    Class(String, ClassMemberList, Vec<String>),
    Template(Dec, Vec<String>),
}

pub enum MemberSpecifier {
    Public,
    Private,
}

pub struct ClassMember {
    specifier: MemberSpecifier,
    dec: Dec,
}

pub type ClassMemberList = Vec<ClassMember>;
