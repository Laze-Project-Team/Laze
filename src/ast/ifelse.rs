use super::{exp::Exp, stm::Stm};

pub type IfElseList = Vec<IfElse>;
pub type IfElse = Box<IfElse_>;
#[derive(Clone, Debug)]
pub struct IfElse_ {
    pub pos: usize,
    pub data: IfElseData,
}

#[derive(Clone, Debug)]
pub enum IfElseData {
    If(Exp, Stm),
    ElseIf(Exp, Stm),
    Else(Stm),
}

impl IfElse_ {
    pub fn if_(pos: usize, test: Exp, body: Stm) -> IfElse {
        Box::new(IfElse_ {
            pos,
            data: IfElseData::If(test, body),
        })
    }
    pub fn else_if(pos: usize, test: Exp, body: Stm) -> IfElse {
        Box::new(IfElse_ {
            pos,
            data: IfElseData::ElseIf(test, body),
        })
    }
    pub fn else_(pos: usize, body: Stm) -> IfElse {
        Box::new(IfElse_ {
            pos,
            data: IfElseData::Else(body),
        })
    }
}
