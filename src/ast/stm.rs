use super::{dec, exp, ty};

pub type StmList = Vec<Stm>;

pub type Stm = Box<Stm_>;

#[derive(Clone)]
pub struct Stm_ {
    pos: (i32, i32),
    data: StmData,
}

#[derive(Clone)]
pub enum StmData {
    Compound(StmList),
    Assign(String, ty::Type, exp::Exp),
    Dec(dec::Dec),
    // Ifelse
    While(exp::Exp, Stm),
    For(Stm, exp::Exp, Stm, Stm),
    Call(exp::Exp, exp::ExpList),
    Return(exp::Exp),
    Loop(Stm),
    Repeat(exp::Exp, Stm),

    None,
}

impl Stm_ {
    fn CompoundStm(pos: (i32, i32), stmList: StmList) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Compound(stmList),
        })
    }
    fn AssignStm(pos: (i32, i32), var: String, ty: ty::Type, init: exp::Exp) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Assign(var, ty, init),
        })
    }
    fn DecStm(pos: (i32, i32), dec: dec::Dec) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Dec(dec),
        })
    }
    fn WhileStm(pos: (i32, i32), test: exp::Exp, body: Stm) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::While(test, body),
        })
    }
    fn ForStm(pos: (i32, i32), init: Stm, test: exp::Exp, incr: Stm, body: Stm) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::For(init, test, incr, body),
        })
    }
    fn CallStm(pos: (i32, i32), func: exp::Exp, args: exp::ExpList) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Call(func, args),
        })
    }
    fn ReturnStm(pos: (i32, i32), val: exp::Exp) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Return(val),
        })
    }
    fn LoopStm(pos: (i32, i32), body: Stm) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Loop(body),
        })
    }
    fn RepeatStm(pos: (i32, i32), count: exp::Exp, body: Stm) -> Stm {
        Box::new(Stm_ {
            pos,
            data: StmData::Repeat(count, body),
        })
    }
}
