pub type WasmTypeList = Vec<WasmType>;

#[derive(Debug)]
pub enum WasmType {
    I32,
    I64,
    F32,
    F64,
    None,
}

#[derive(Debug)]
pub enum BinOper {
    Add,
    Sub,
    Mul,
    DivSigned,
    DivUnsigned,
    RemSigned,
    RemUnsigned,
    Eq,
    Ne,
    LtSigned,
    LtUnsigned,
    GtSigned,
    GtUnsigned,
    LeSigned,
    LeUnsigned,
    GeSigned,
    GeUnsigned,
    And,
    Or,
}

#[derive(Debug)]
pub enum UniOper {
    Abs,
    Neg,
    Ceil,
    Floor,
    Trunc,
    Nearest,
    Sqrt,
}
