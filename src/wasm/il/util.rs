use std::io::{stderr, Write};

use crate::{
    ast::op::Oper,
    wasm::semantic::laze_type::{LazeType, LazeType_},
};

use super::{
    exp::{Exp, ExpList, Exp_},
    module::{Module, ModuleList, Module_},
    stm::{Stm, StmList, Stm_},
};

pub type WasmTypeList = Vec<WasmType>;

pub struct WasmExpTy {
    pub ty: LazeType,
    pub data: WasmData,
}

impl WasmExpTy {
    pub fn ty_exp(self, message: String) -> (LazeType, Exp) {
        match self.data {
            WasmData::Exp(exp) => (self.ty, exp),
            _ => {
                let _ = writeln!(stderr(), "{message}");
                (LazeType_::none_type(), Exp_::none_exp())
            }
        }
    }
    pub fn none() -> WasmExpTy {
        WasmExpTy {
            ty: LazeType_::none_type(),
            data: WasmData::None,
        }
    }
    pub fn stm(self, message: String) -> Stm {
        match self.data {
            WasmData::Stm(stm) => stm,
            _ => {
                let _ = writeln!(stderr(), "{message}");
                Stm_::none_stm()
            }
        }
    }
    pub fn stmlist(self, message: String) -> StmList {
        match self.data {
            WasmData::StmList(stmlist) => stmlist,
            _ => {
                let _ = writeln!(stderr(), "{message}");
                vec![]
            }
        }
    }
    pub fn exp(self, message: String) -> Exp {
        match self.data {
            WasmData::Exp(exp) => exp,
            _ => {
                let _ = writeln!(stderr(), "{message}");
                Exp_::none_exp()
            }
        }
    }
    pub fn explist(self, message: String) -> ExpList {
        match self.data {
            WasmData::ExpList(explist) => explist,
            _ => {
                let _ = writeln!(stderr(), "{message}");
                vec![]
            }
        }
    }
    pub fn module(self, message: String) -> Module {
        match self.data {
            WasmData::Module(module) => module,
            _ => {
                let _ = writeln!(stderr(), "{message}");
                Module_::none_mod()
            }
        }
    }
    pub fn modulelist(self, message: String) -> ModuleList {
        match self.data {
            WasmData::ModuleList(modulelist) => modulelist,
            _ => {
                let _ = writeln!(stderr(), "{message}");
                vec![]
            }
        }
    }
    pub fn new_stm(ty: LazeType, stm: Stm) -> Self {
        WasmExpTy {
            ty,
            data: WasmData::Stm(stm),
        }
    }
    pub fn new_stmlist(ty: LazeType, stmlist: StmList) -> Self {
        WasmExpTy {
            ty,
            data: WasmData::StmList(stmlist),
        }
    }
    pub fn new_exp(ty: LazeType, exp: Exp) -> Self {
        WasmExpTy {
            ty,
            data: WasmData::Exp(exp),
        }
    }
    pub fn new_explist(ty: LazeType, explist: ExpList) -> Self {
        WasmExpTy {
            ty,
            data: WasmData::ExpList(explist),
        }
    }
    pub fn new_module(module: Module) -> Self {
        WasmExpTy {
            ty: LazeType_::none_type(),
            data: WasmData::Module(module),
        }
    }
    pub fn new_modulelist(module_list: ModuleList) -> Self {
        WasmExpTy {
            ty: LazeType_::none_type(),
            data: WasmData::ModuleList(module_list),
        }
    }
}

pub enum WasmData {
    Exp(Exp),
    ExpList(ExpList),
    Stm(Stm),
    StmList(StmList),
    Module(Module),
    ModuleList(ModuleList),
    None,
}

#[derive(Debug, Clone)]
pub enum WasmType {
    I32,
    I64,
    F32,
    F64,
    None,
}

impl WasmType {
    pub fn to_string(&self) -> String {
        match self {
            Self::F32 => "f32".to_string(),
            Self::F64 => "f64".to_string(),
            Self::I32 => "i32".to_string(),
            Self::I64 => "i64".to_string(),
            Self::None => "".to_string(),
        }
    }
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

    None,
}

impl BinOper {
    pub fn to_string(&self, ty: &WasmType) -> String {
        match self {
            Self::Add => "add".to_string(),
            Self::Sub => "sub".to_string(),
            Self::Mul => "mul".to_string(),
            Self::DivSigned => match ty {
                WasmType::I32 | WasmType::I64 => "div_s".to_string(),
                WasmType::F32 | WasmType::F64 => "div".to_string(),
                WasmType::None => "".to_string(),
            },
            Self::DivUnsigned => "div_u".to_string(),
            Self::RemSigned => "rem_s".to_string(),
            Self::RemUnsigned => "rem_u".to_string(),
            Self::Eq => "eq".to_string(),
            Self::Ne => "ne".to_string(),
            Self::LtSigned => match ty {
                WasmType::I32 | WasmType::I64 => "lt_s".to_string(),
                WasmType::F32 | WasmType::F64 => "lt".to_string(),
                WasmType::None => "".to_string(),
            },
            Self::LtUnsigned => "lt_u".to_string(),
            Self::GtSigned => match ty {
                WasmType::I32 | WasmType::I64 => "gt_s".to_string(),
                WasmType::F32 | WasmType::F64 => "gt".to_string(),
                WasmType::None => "".to_string(),
            },
            Self::GtUnsigned => "gt_u".to_string(),
            Self::LeSigned => match ty {
                WasmType::I32 | WasmType::I64 => "le_s".to_string(),
                WasmType::F32 | WasmType::F64 => "le".to_string(),
                WasmType::None => "".to_string(),
            },
            Self::LeUnsigned => "le_u".to_string(),
            Self::GeSigned => match ty {
                WasmType::I32 | WasmType::I64 => "ge_s".to_string(),
                WasmType::F32 | WasmType::F64 => "ge".to_string(),
                WasmType::None => "".to_string(),
            },
            Self::GeUnsigned => "ge_u".to_string(),
            Self::And => "and".to_string(),
            Self::Or => "or".to_string(),
            Self::None => "".to_string(),
        }
    }
}

impl BinOper {
    pub fn from_ast(oper: &Oper) -> Self {
        match oper {
            Oper::Plus => Self::Add,
            Oper::Minus => Self::Sub,
            Oper::Times => Self::Mul,
            Oper::Divide => Self::DivSigned,
            Oper::Mod => Self::RemSigned,
            Oper::And => Self::And,
            Oper::Or => Self::Or,
            Oper::Ge => Self::GeSigned,
            Oper::Gt => Self::GtSigned,
            Oper::Le => Self::LeSigned,
            Oper::Lt => Self::LtSigned,
            Oper::Eq => Self::Eq,
            Oper::Neq => Self::Ne,
            _ => Self::None,
        }
    }
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

impl UniOper {
    pub fn to_string(&self) -> String {
        match self {
            Self::Abs => "abs".to_string(),
            Self::Neg => "neg".to_string(),
            Self::Ceil => "ceil".to_string(),
            Self::Floor => "floor".to_string(),
            Self::Trunc => "trunc".to_string(),
            Self::Nearest => "nearest".to_string(),
            Self::Sqrt => "sqrt".to_string(),
        }
    }
}
