use super::exp::ExpList;

pub type Var = Box<Var_>;

#[derive(Clone, Debug)]
pub struct Var_ {
    pub pos: (usize, usize),
    pub data: VarData,
}

#[derive(Clone, Debug)]
pub enum VarData {
    Simple(String),
    Call(Var, ExpList),
    Array(Var, ExpList),
    Pointer(Var),
    None,
}

impl Var_ {
    pub fn simple_var(pos: (usize, usize), name: String) -> Var {
        Box::new(Var_ {
            pos,
            data: VarData::Simple(name),
        })
    }
    pub fn call_var(pos: (usize, usize), var: Var, elist: ExpList) -> Var {
        Box::new(Var_ {
            pos,
            data: VarData::Call(var, elist),
        })
    }
    pub fn array_var(pos: (usize, usize), var: Var, exp: ExpList) -> Var {
        Box::new(Var_ {
            pos,
            data: VarData::Array(var, exp),
        })
    }
    pub fn pointer_var(pos: (usize, usize), var: Var) -> Var {
        Box::new(Var_ {
            pos,
            data: VarData::Pointer(var),
        })
    }
}
