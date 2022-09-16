use super::{field, op, stm};

pub type Exp = Box<Exp_>;
pub type ExpList = Vec<Exp>;

pub struct Exp_ {
    pos: (i32, i32),
    data: ExpData,
}

pub enum ExpData {
    Int(i64),
    Real(f64),
    Char(char),
    String(String),
    Bool(bool),

    Var(String),
    Call(Exp, ExpList),
    BinOp(op::Oper, Exp, Exp),
    UnaryOp(op::Oper, Exp),
    Func(field::FieldList, field::FieldList, stm::Stm),
    Field(Exp, String),
    Array(ExpList),
    SizeOf(String),
    Paren(Exp),

    None,
}
