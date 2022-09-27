use super::util::{BinOper, UniOper, WasmType};

pub type ExpList = Vec<Exp>;
pub type Exp = Box<Exp_>;

#[derive(Debug)]
pub struct Exp_ {
    pub ty: WasmType,
    pub data: ExpData,
}

#[derive(Debug)]
pub enum ExpData {
    BinOp(BinOper, Exp, Exp),
    UnaryOp(UniOper, Exp),
    Const(ConstData),
    GetLocal(i32),
    GetGlobal(i32),
    CallExp(i32, Option<String>, ExpList),
    CallIndirect(Exp, ExpList, i32),
    IfExp(Exp, Exp, Exp),
    Load(Exp),
    Convert(Exp),
    None,
}

impl Exp_ {
    pub fn none_exp() -> Exp {
        Box::new(Exp_ {
            ty: WasmType::None,
            data: ExpData::None,
        })
    }
    pub fn add_addr_exp(left: Exp, right: Exp) -> Exp {
        Self::binop_exp(WasmType::I32, BinOper::Add, left, right)
    }
    pub fn mul_addr_exp(left: Exp, right: Exp) -> Exp {
        Self::binop_exp(WasmType::I32, BinOper::Add, left, right)
    }
    pub fn binop_exp(ty: WasmType, oper: BinOper, left: Exp, right: Exp) -> Exp {
        Box::new(Exp_ {
            ty,
            data: ExpData::BinOp(oper, left, right),
        })
    }
    pub fn unaryop_exp(ty: WasmType, oper: UniOper, exp: Exp) -> Exp {
        Box::new(Exp_ {
            ty,
            data: ExpData::UnaryOp(oper, exp),
        })
    }
    pub fn consti32_exp(i: i32) -> Exp {
        Box::new(Exp_ {
            ty: WasmType::I32,
            data: ExpData::Const(ConstData::I32(i)),
        })
    }
    pub fn consti64_exp(i: i64) -> Exp {
        Box::new(Exp_ {
            ty: WasmType::I64,
            data: ExpData::Const(ConstData::I64(i)),
        })
    }
    pub fn constf32_exp(f: f32) -> Exp {
        Box::new(Exp_ {
            ty: WasmType::F32,
            data: ExpData::Const(ConstData::F32(f)),
        })
    }
    pub fn constf64_exp(f: f64) -> Exp {
        Box::new(Exp_ {
            ty: WasmType::F64,
            data: ExpData::Const(ConstData::F64(f)),
        })
    }
    pub fn getlocal_exp(ty: WasmType, index: i32) -> Exp {
        Box::new(Exp_ {
            ty,
            data: ExpData::GetLocal(index),
        })
    }
    pub fn getglobal_exp(ty: WasmType, index: i32) -> Exp {
        Box::new(Exp_ {
            ty,
            data: ExpData::GetGlobal(index),
        })
    }
    pub fn call_exp(ty: WasmType, index: i32, args: ExpList, label: Option<String>) -> Exp {
        Box::new(Exp_ {
            ty,
            data: ExpData::CallExp(index, label, args),
        })
    }
    pub fn call_indirect_exp(ty: WasmType, index: Exp, type_index: i32, args: ExpList) -> Exp {
        Box::new(Exp_ {
            ty,
            data: ExpData::CallIndirect(index, args, type_index),
        })
    }
    pub fn if_exp(ty: WasmType, test: Exp, if_body: Exp, else_body: Exp) -> Exp {
        Box::new(Exp_ {
            ty,
            data: ExpData::IfExp(test, if_body, else_body),
        })
    }
    pub fn load_exp(ty: WasmType, addr: Exp) -> Exp {
        Box::new(Exp_ {
            ty,
            data: ExpData::Load(addr),
        })
    }
    pub fn convert_exp(ty: WasmType, exp: Exp) -> Exp {
        Box::new(Exp_ {
            ty,
            data: ExpData::Convert(exp),
        })
    }
}

#[derive(Debug)]
pub enum ConstData {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}
