use super::{field, op, stm, ty};

pub type Exp = Box<Exp_>;
pub type ExpList = Vec<Exp>;

pub struct Exp_ {
    pos: (i32, i32),
    data: ExpData,
}

pub enum ExpData {
    Int(i64),
    Addr(i32),
    Real(f64),
    Char(char),
    String(String),
    Bool(bool),

    Var(String),
    Call(Exp, ExpList),
    BinOp(op::Oper, Exp, Exp),
    UnaryOp(op::Oper, Exp),
    Func(field::FieldList, ty::Type, stm::Stm),
    Field(Exp, String),
    Array(ExpList),
    SizeOf(Exp),
    Paren(Exp),

    None,
}

impl Exp_ {
    fn IntExp(pos: (i32, i32), data: i64) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::Int(data),
        })
    }
    fn AddrExp(pos: (i32, i32), data: i32) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::Addr(data),
        })
    }
    fn RealExp(pos: (i32, i32), data: f64) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::Real(data),
        })
    }
    fn CharExp(pos: (i32, i32), data: char) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::Char(data),
        })
    }
    fn StringExp(pos: (i32, i32), data: String) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::String(data),
        })
    }
    fn BoolExp(pos: (i32, i32), data: bool) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::Bool(data),
        })
    }

    fn VarExp(pos: (i32, i32), data: String) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::Var(data),
        })
    }
    fn CallExp(pos: (i32, i32), func: Exp, args: ExpList) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::Call(func, args),
        })
    }
    fn BinOpExp(pos: (i32, i32), op: op::Oper, left: Exp, right: Exp) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::BinOp(op, left, right),
        })
    }
    fn UnaryOpExp(pos: (i32, i32), op: op::Oper, exp: Exp) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::UnaryOp(op, exp),
        })
    }
    fn FuncExp(pos: (i32, i32), params: field::FieldList, result: ty::Type, stm: stm::Stm) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::Func(params, result, stm),
        })
    }
    fn FieldExp(pos: (i32, i32), field: Exp, member: String) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::Field(field, member),
        })
    }
    fn ArrayExp(pos: (i32, i32), expList: ExpList) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::Array(expList),
        })
    }
    fn SizeOfExp(pos: (i32, i32), var: Exp) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::SizeOf(var),
        })
    }
    fn ParenExp(pos: (i32, i32), exp: Exp) -> Exp {
        Box::new(Exp_ {
            pos,
            data: ExpData::Paren(exp),
        })
    }
}
