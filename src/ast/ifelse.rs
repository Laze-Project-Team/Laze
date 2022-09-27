use super::{exp::ASTExp, stm::Stm};

pub type IfElseList = Vec<IfElse>;
pub type IfElse = Box<IfElse_>;
#[derive(Clone, Debug)]
pub struct IfElse_ {
    pub pos: (usize, usize),
    pub data: IfElseData,
}

#[derive(Clone, Debug)]
pub enum IfElseData {
    If(ASTExp, Stm),
    ElseIf(ASTExp, Stm),
    Else(Stm),
}

impl IfElse_ {
    pub fn if_(pos: (usize, usize), test: ASTExp, body: Stm) -> IfElse {
        Box::new(IfElse_ {
            pos,
            data: IfElseData::If(test, body),
        })
    }
    pub fn else_if(pos: (usize, usize), test: ASTExp, body: Stm) -> IfElse {
        Box::new(IfElse_ {
            pos,
            data: IfElseData::ElseIf(test, body),
        })
    }
    pub fn else_(pos: (usize, usize), body: Stm) -> IfElse {
        Box::new(IfElse_ {
            pos,
            data: IfElseData::Else(body),
        })
    }
}
