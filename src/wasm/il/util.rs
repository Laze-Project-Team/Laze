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
    pub fn ty_exp(self, message: &str) -> (LazeType, Exp) {
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
    pub fn stm(self, message: &str) -> Stm {
        match self.data {
            WasmData::Stm(stm) => stm,
            _ => {
                let _ = writeln!(stderr(), "{message}");
                Stm_::none_stm()
            }
        }
    }
    pub fn stmlist(self, message: &str) -> StmList {
        match self.data {
            WasmData::StmList(stmlist) => stmlist,
            _ => {
                let _ = writeln!(stderr(), "{message}");
                vec![]
            }
        }
    }
    pub fn exp(self, message: &str) -> Exp {
        match self.data {
            WasmData::Exp(exp) => exp,
            _ => {
                let _ = writeln!(stderr(), "{message}");
                Exp_::none_exp()
            }
        }
    }
    pub fn explist(self, message: &str) -> ExpList {
        match self.data {
            WasmData::ExpList(explist) => explist,
            _ => {
                let _ = writeln!(stderr(), "{message}");
                vec![]
            }
        }
    }
    pub fn module(self, message: &str) -> Module {
        match self.data {
            WasmData::Module(module) => module,
            _ => {
                let _ = writeln!(stderr(), "{message}");
                Module_::none_mod()
            }
        }
    }
    pub fn modulelist(self, message: &str) -> ModuleList {
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

    None,
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
