use super::{field, op, stm, suffix::ExpSuffixList};

pub type Exp = Box<Exp_>;
pub type ExpList = Vec<Exp>;

#[derive(Clone, Debug)]
pub struct Exp_ {
    pub pos: u32,
    pub data: ExpData,
}

#[derive(Clone, Debug)]
pub enum ExpData {
    Int(String),
    Real(String),
    Char(char),
    String(String),
    Bool(bool),

    Var(String),
    Call(Exp, ExpList),
    BinOp(op::OperList, ExpList),
    UnaryOp(op::OperList, Exp),
    Func(field::FieldList, field::FieldList, stm::Stm),
    Field(Exp, String),
    Array(ExpList),
    SizeOf(Exp),
    Paren(Exp),
    Suffix(Exp, ExpSuffixList),

    None,
}

impl Exp_ {
    pub fn int_exp(pos: u32, data: String) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::String(data),
        })
    }
    pub fn real_exp(pos: u32, data: String) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::Real(data),
        })
    }
    pub fn char_exp(pos: u32, data: char) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::Char(data),
        })
    }
    pub fn string_exp(pos: u32, data: String) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::String(data),
        })
    }
    pub fn bool_exp(pos: u32, data: bool) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::Bool(data),
        })
    }

    pub fn var_exp(pos: u32, data: String) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::Var(data),
        })
    }
    pub fn call_exp(pos: u32, func: Exp, args: ExpList) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::Call(func, args),
        })
    }
    pub fn binop_exp(pos: u32, oplist: op::OperList, explist: ExpList) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::BinOp(oplist, explist),
        })
    }
    pub fn unaryop_exp(pos: u32, oplist: op::OperList, exp: Exp) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::UnaryOp(oplist, exp),
        })
    }
    pub fn func_exp(
        pos: u32,
        params: field::FieldList,
        result: field::FieldList,
        stm: stm::Stm,
    ) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::Func(params, result, stm),
        })
    }
    pub fn field_exp(pos: u32, field: Exp, member: String) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::Field(field, member),
        })
    }
    pub fn array_exp(pos: u32, explist: ExpList) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::Array(explist),
        })
    }
    pub fn sizeof_exp(pos: u32, var: Exp) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::SizeOf(var),
        })
    }
    pub fn paren_exp(pos: u32, exp: Exp) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::Paren(exp),
        })
    }
    pub fn suffix_exp(pos: u32, exp: Exp, suffix: ExpSuffixList) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::Suffix(exp, suffix),
        })
    }
}
