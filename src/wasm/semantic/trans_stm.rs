use crate::{
    ast::{
        exp::{ASTExp, ASTExp_},
        op::Oper,
        stm::{AssignType, Stm as ASTStm, StmData as ASTStmData, StmList as ASTStmList},
        var::{Var, VarData},
    },
    wasm::il::stm::{Stm, StmList, Stm_},
};

use super::entry_map::EntryMap;

pub fn trans_stm(stm: &ASTStm, venv: &mut EntryMap, tenv: &mut EntryMap) -> Stm {
    let new_stm;
    match &stm.data {
        ASTStmData::Compound(stmlist) => {
            venv.enter_scope();
            tenv.enter_scope();
            new_stm = Stm_::block_stm(trans_stmlist(&stmlist, venv, tenv));
            venv.exit_scope();
            tenv.exit_scope();
        }
        ASTStmData::Assign(var, init, ty) => {}
        _ => {}
    }
    Stm_::none_stm()
    // new_stm
}

pub fn trans_assign_stm(var: Var, init: ASTExp, assign_type: AssignType) -> Stm {
    // let new_stm;
    let oper = match assign_type {
        AssignType::Add => Oper::Plus,
        AssignType::Sub => Oper::Minus,
        AssignType::Mul => Oper::Times,
        AssignType::Div => Oper::Divide,
        _ => Oper::None,
    };
    let _added_init = if let Oper::None = oper {
        init
    } else {
        ASTExp_::binop_exp(
            init.pos,
            vec![oper],
            vec![ASTExp_::var_exp(var.pos, var.clone()), init],
        )
    };
    match var.data {
        VarData::SuffixVar(var, suffixlist) => {}
        _ => {}
    }
    Stm_::none_stm()
}

pub fn trans_stmlist(stmlist: &ASTStmList, venv: &mut EntryMap, tenv: &mut EntryMap) -> StmList {
    let mut result = vec![];
    for stm in stmlist {
        result.push(trans_stm(stm, venv, tenv));
    }
    result
}
