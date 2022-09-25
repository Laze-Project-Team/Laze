use super::{
    dec,
    exp::{self},
    ifelse::{self, IfElseList},
    var::Var,
};

pub type StmList = Vec<Stm>;

pub type Stm = Box<Stm_>;

#[derive(Clone, Debug)]
pub struct Stm_ {
    pub pos: usize,
    pub data: StmData,
}

#[derive(Clone, Debug)]
pub enum AssignType {
    Normal,
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Debug)]
pub enum StmData {
    Compound(StmList),
    Assign(Var, exp::Exp, AssignType),
    Dec(dec::Dec),
    Exp(exp::Exp),
    IfElse(ifelse::IfElseList),
    While(exp::Exp, Stm),
    For(Stm, exp::Exp, Stm, Stm),
    Call(exp::Exp, exp::ExpList),
    Return(exp::Exp),
    Loop(Stm),
    Repeat(exp::Exp, Stm),
    Continue,
    Break,

    None,
}

impl Stm_ {
    pub fn compound_stm(pos: usize, stmlist: StmList) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Compound(stmlist),
        })
    }
    pub fn assign_stm(pos: usize, var: Var, init: exp::Exp, assign_type: AssignType) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Assign(var, init, assign_type),
        })
    }
    pub fn dec_stm(pos: usize, dec: dec::Dec) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Dec(dec),
        })
    }
    pub fn exp_stm(pos: usize, exp: exp::Exp) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Exp(exp),
        })
    }
    pub fn ifelse_stm(pos: usize, ifelselist: IfElseList) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::IfElse(ifelselist),
        })
    }
    pub fn while_stm(pos: usize, test: exp::Exp, body: Stm) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::While(test, body),
        })
    }
    pub fn for_stm(pos: usize, init: Stm, test: exp::Exp, incr: Stm, body: Stm) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::For(init, test, incr, body),
        })
    }
    pub fn call_stm(pos: usize, func: exp::Exp, args: exp::ExpList) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Call(func, args),
        })
    }
    pub fn return_stm(pos: usize, val: exp::Exp) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Return(val),
        })
    }
    pub fn continue_stm(pos: usize) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Continue,
        })
    }
    pub fn break_stm(pos: usize) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Break,
        })
    }
    pub fn loop_stm(pos: usize, body: Stm) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Loop(body),
        })
    }
    pub fn repeat_stm(pos: usize, count: exp::Exp, body: Stm) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Repeat(count, body),
        })
    }
}
