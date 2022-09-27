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
    pub pos: (usize, usize),
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
    Assign(Var, exp::ASTExp, AssignType),
    Dec(dec::Dec),
    Exp(exp::ASTExp),
    IfElse(ifelse::IfElseList),
    While(exp::ASTExp, Stm),
    For(Stm, exp::ASTExp, Stm, Stm),
    Call(exp::ASTExp, exp::ASTExpList),
    Return(exp::ASTExp),
    Loop(Stm),
    Repeat(exp::ASTExp, Stm),
    Continue,
    Break,

    None,
}

impl Stm_ {
    pub fn compound_stm(pos: (usize, usize), stmlist: StmList) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Compound(stmlist),
        })
    }
    pub fn assign_stm(
        pos: (usize, usize),
        var: Var,
        init: exp::ASTExp,
        assign_type: AssignType,
    ) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Assign(var, init, assign_type),
        })
    }
    pub fn dec_stm(pos: (usize, usize), dec: dec::Dec) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Dec(dec),
        })
    }
    pub fn exp_stm(pos: (usize, usize), exp: exp::ASTExp) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Exp(exp),
        })
    }
    pub fn ifelse_stm(pos: (usize, usize), ifelselist: IfElseList) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::IfElse(ifelselist),
        })
    }
    pub fn while_stm(pos: (usize, usize), test: exp::ASTExp, body: Stm) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::While(test, body),
        })
    }
    pub fn for_stm(pos: (usize, usize), init: Stm, test: exp::ASTExp, incr: Stm, body: Stm) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::For(init, test, incr, body),
        })
    }
    pub fn call_stm(pos: (usize, usize), func: exp::ASTExp, args: exp::ASTExpList) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Call(func, args),
        })
    }
    pub fn return_stm(pos: (usize, usize), val: exp::ASTExp) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Return(val),
        })
    }
    pub fn continue_stm(pos: (usize, usize)) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Continue,
        })
    }
    pub fn break_stm(pos: (usize, usize)) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Break,
        })
    }
    pub fn loop_stm(pos: (usize, usize), body: Stm) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Loop(body),
        })
    }
    pub fn repeat_stm(pos: (usize, usize), count: exp::ASTExp, body: Stm) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Repeat(count, body),
        })
    }
}
