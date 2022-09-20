use crate::parser::parser::Data;

use super::{
    dec::{self, Dec},
    exp::Exp,
    field::Field,
    stm::Stm,
    ty::Type,
};

pub type AST = dec::DecList;

#[derive(Clone)]
pub enum ASTNode {
    Dec(Dec),
    Stm(Stm),
    Exp(Exp),
    Type(Type),
    Field(Field),
    String(String),
    None,
}

impl Data for ASTNode {
    fn string(str: String) -> Self {
        Self::String(str)
    }
    fn null() -> Self {
        Self::None
    }
    fn data(name: String, parser: &mut crate::parser::parser::Parser<Self>) -> Self {
        // need to write code
        Self::None
    }
    fn is_null(&self) -> bool {
        if let Self::None = self {
            true
        } else {
            false
        }
    }
}
