use super::{field, op, stm, suffix::ASTExpSuffixList, var::Var};

pub type ASTExp = Box<ASTExp_>;
pub type ASTExpList = Vec<ASTExp>;

#[derive(Clone, Debug)]
pub struct ASTExp_ {
    pub pos: (usize, usize),
    pub data: ASTExpData,
}

#[derive(Clone, Debug)]
pub enum ASTExpData {
    Int(String),
    Short(String),
    Real(String),
    Char(char),
    String(String),
    Bool(bool),

    Var(Var),
    Call(ASTExp, ASTExpList),
    BinOp(op::OperList, ASTExpList),
    UnaryOp(op::OperList, ASTExp),
    Func(field::FieldList, field::FieldList, stm::Stm),
    Field(ASTExp, String),
    Array(ASTExpList),
    SizeOf(ASTExp),
    Paren(ASTExp),
    Suffix(ASTExp, ASTExpSuffixList),

    None,
}

impl ASTExp_ {
    pub fn none_exp(pos: (usize, usize)) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            data: ASTExpData::None,
        })
    }
    pub fn int_exp(pos: (usize, usize), data: String) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            data: ASTExpData::String(data),
        })
    }
    pub fn real_exp(pos: (usize, usize), data: String) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            data: ASTExpData::Real(data),
        })
    }
    pub fn char_exp(pos: (usize, usize), data: char) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            data: ASTExpData::Char(data),
        })
    }
    pub fn string_exp(pos: (usize, usize), data: String) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            data: ASTExpData::String(data),
        })
    }
    pub fn bool_exp(pos: (usize, usize), data: bool) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            data: ASTExpData::Bool(data),
        })
    }

    pub fn var_exp(pos: (usize, usize), data: Var) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            data: ASTExpData::Var(data),
        })
    }
    pub fn call_exp(pos: (usize, usize), func: ASTExp, args: ASTExpList) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            data: ASTExpData::Call(func, args),
        })
    }
    pub fn binop_exp(pos: (usize, usize), oplist: op::OperList, explist: ASTExpList) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            data: ASTExpData::BinOp(oplist, explist),
        })
    }
    pub fn unaryop_exp(pos: (usize, usize), oplist: op::OperList, exp: ASTExp) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            data: ASTExpData::UnaryOp(oplist, exp),
        })
    }
    pub fn func_exp(
        pos: (usize, usize),
        params: field::FieldList,
        result: field::FieldList,
        stm: stm::Stm,
    ) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            data: ASTExpData::Func(params, result, stm),
        })
    }
    pub fn field_exp(pos: (usize, usize), field: ASTExp, member: String) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            data: ASTExpData::Field(field, member),
        })
    }
    pub fn array_exp(pos: (usize, usize), explist: ASTExpList) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            data: ASTExpData::Array(explist),
        })
    }
    pub fn sizeof_exp(pos: (usize, usize), var: ASTExp) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            data: ASTExpData::SizeOf(var),
        })
    }
    pub fn paren_exp(pos: (usize, usize), exp: ASTExp) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            data: ASTExpData::Paren(exp),
        })
    }
    pub fn suffix_exp(pos: (usize, usize), exp: ASTExp, suffix: ASTExpSuffixList) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            data: ASTExpData::Suffix(exp, suffix),
        })
    }
}
