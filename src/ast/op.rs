pub type OperList = Vec<Oper>;

#[derive(Clone, Debug)]
pub enum Oper {
    Plus,
    Minus,
    Times,
    Divide,
    Mod,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Subscript,
    TypeEq,

    Deref,
    Address,
    UMinus,
    Not,

    None,
}

pub fn string_to_oper(name: &str) -> Oper {
    match name {
        "AddOp" => Oper::Plus,
        "SubOp" => Oper::Minus,
        "EqOp" => Oper::Eq,
        "NeOp" => Oper::Neq,
        "LtOp" => Oper::Lt,
        "LeOp" => Oper::Le,
        "GtOp" => Oper::Gt,
        "GeOp" => Oper::Ge,
        "AndOp" => Oper::And,
        "OrOp" => Oper::Or,
        "MulOp" => Oper::Times,
        "DivOp" => Oper::Divide,
        "DerefOp" => Oper::Deref,
        "AddressOp" => Oper::Address,
        "NotOp" => Oper::Not,
        _ => Oper::None,
    }
}
