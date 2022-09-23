use super::exp::{Exp, ExpList};

pub type Var = Box<Var_>;

#[derive(Clone, Debug)]
pub struct Var_ {
    pub pos: u32,
    pub data: VarData,
}

#[derive(Clone, Debug)]
pub enum VarData {
    Simple(String),
    Call(Var, ExpList),
    Array(Var, Exp),
    Pointer(Var),
    None,
}

impl Var_ {
    pub fn simple_var(pos: u32, name: String) -> Var {
        Box::new(Var_ {
            pos,
            data: VarData::Simple(name),
        })
    }
    pub fn call_var(pos: u32, var: Var, elist: ExpList) -> Var {
        Box::new(Var_ {
            pos,
            data: VarData::Call(var, elist),
        })
    }
    pub fn array_var(pos: u32, var: Var, exp: Exp) -> Var {
        Box::new(Var_ {
            pos,
            data: VarData::Array(var, exp),
        })
    }
    pub fn pointer_var(pos: u32, var: Var) -> Var {
        Box::new(Var_ {
            pos,
            data: VarData::Pointer(var),
        })
    }
}
