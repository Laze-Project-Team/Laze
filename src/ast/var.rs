use super::suffix::ASTExpSuffixList;

pub type Var = Box<Var_>;

#[derive(Clone, Debug)]
pub struct Var_ {
    pub pos: (usize, usize),
    pub data: VarData,
}

#[derive(Clone, Debug)]
pub enum VarData {
    Simple(String),
    SuffixVar(Var, ASTExpSuffixList),
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
    pub fn suffix_var(pos: (usize, usize), var: Var, suffixlist: ASTExpSuffixList) -> Var {
        Box::new(Var_ {
            pos,
            data: VarData::SuffixVar(var, suffixlist),
        })
    }
    pub fn pointer_var(pos: (usize, usize), var: Var) -> Var {
        Box::new(Var_ {
            pos,
            data: VarData::Pointer(var),
        })
    }
}
