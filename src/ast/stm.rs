use super::{dec, exp, ty};

pub type StmList = Vec<Stm>;

pub type Stm = Box<Stm_>;

pub struct Stm_ {
    pos: (i32, i32),
    data: StmData,
}

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
