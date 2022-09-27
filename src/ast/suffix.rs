use super::exp::{ASTExp, ASTExpList};

pub type ASTExpSuffixList = Vec<ASTExpSuffix>;
pub type ASTExpSuffix = Box<ASTExpSuffix_>;

#[derive(Clone, Debug)]
pub struct ASTExpSuffix_ {
    pub pos: (usize, usize),
    pub data: SuffixData,
}

#[derive(Clone, Debug)]
pub enum SuffixData {
    Call(ASTExpList),
    Dot(String),
    Arrow(String),
    Subscript(ASTExp),
}

impl ASTExpSuffix_ {
    pub fn call_suffix(pos: (usize, usize), explist: ASTExpList) -> ASTExpSuffix {
        Box::new(ASTExpSuffix_ {
            pos,
            data: SuffixData::Call(explist),
        })
    }
    pub fn dot_suffix(pos: (usize, usize), field: String) -> ASTExpSuffix {
        Box::new(ASTExpSuffix_ {
            pos,
            data: SuffixData::Dot(field),
        })
    }
    pub fn arrow_suffix(pos: (usize, usize), field: String) -> ASTExpSuffix {
        Box::new(ASTExpSuffix_ {
            pos,
            data: SuffixData::Arrow(field),
        })
    }
    pub fn subscript_suffix(pos: (usize, usize), index: ASTExp) -> ASTExpSuffix {
        Box::new(ASTExpSuffix_ {
            pos,
            data: SuffixData::Subscript(index),
        })
    }
}
