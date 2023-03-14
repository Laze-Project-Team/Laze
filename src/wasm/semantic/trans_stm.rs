use std::{
    io::{stderr, Write},
    mem::replace,
};

use crate::{
    ast::{
        dec::{DecData, Dec_},
        exp::{ASTExp, ASTExp_},
        ifelse::{IfElseData, IfElseList},
        op::Oper,
        stm::{
            AssignType, Stm as ASTStm, StmData as ASTStmData, StmList as ASTStmList,
            Stm_ as ASTStm_,
        },
        ty::Type_,
        var::{Var, VarData, Var_},
    },
    wasm::{
        frame::frame::FrameAccess,
        il::{
            exp::{ExpData, Exp_},
            stm::{Stm, StmList, Stm_},
        },
    },
};

use super::{
    entry_map::EnvEntry,
    laze_type::{LazeTypeData, LazeType_},
    semantic_param::SemanticParam,
    trans_dec::trans_dec,
    trans_exp::trans_exp,
    trans_var::{check_member, trans_right_var, trans_suffix_var_to_addr},
};

pub fn trans_stm(stm: &ASTStm, semantic_data: &mut SemanticParam) -> Stm {
    let new_stm;
    match &stm.data {
        ASTStmData::Compound(stmlist) => {
            semantic_data.venv.enter_scope();
            semantic_data.tenv.enter_scope();
            new_stm = Stm_::block_stm(trans_stmlist(&stmlist, semantic_data));
            semantic_data.venv.exit_scope();
            semantic_data.tenv.exit_scope();
        }
        ASTStmData::Assign(var, init, assign_type) => {
            new_stm = trans_assign_stm(var, init, assign_type, semantic_data);
        }
        ASTStmData::IfElse(ifelselist) => {
            new_stm = trans_if_stm(ifelselist, semantic_data, stm.pos);
        }
        ASTStmData::Exp(exp) => match trans_exp(exp, semantic_data).exp("".to_string()).data {
            ExpData::CallExp(index, label, args) => {
                new_stm = Stm_::call_stm(index, args, label);
            }
            ExpData::CallIndirect(index, args, type_index) => {
                new_stm = Stm_::call_indirect_stm(index, args, type_index);
            }
            _ => {
                new_stm = Stm_::none_stm();
            }
        },
        ASTStmData::While(test_exp, while_body) => {
            semantic_data.loop_index += 1;
            new_stm = Stm_::loop_stm(
                trans_exp(test_exp, semantic_data).exp("".to_string()),
                trans_stm(while_body, semantic_data),
                semantic_data.loop_index,
                false,
            );
        }
        ASTStmData::Continue => {
            if semantic_data.loop_index > 1 {
                new_stm = Stm_::break_stm(semantic_data.loop_index);
            } else {
                let _ = writeln!(
                    stderr(),
                    "Cannot continue, because this statement is not in a loop: {:?}",
                    stm.pos
                );
                new_stm = Stm_::none_stm();
            }
        }
        ASTStmData::Break => {
            if semantic_data.loop_index > 1 {
                new_stm = Stm_::break_stm(semantic_data.loop_index - 1);
            } else {
                let _ = writeln!(
                    stderr(),
                    "Cannot break out, because this statement is not in a loop: {:?}",
                    stm.pos
                );
                new_stm = Stm_::none_stm();
            }
        }
        ASTStmData::For(init, test_exp, incr, for_body) => {
            let mut stm_list = vec![];
            stm_list.push(trans_stm(init, semantic_data));
            let mut loop_body = vec![];
            loop_body.push(trans_stm(incr, semantic_data));
            loop_body.push(trans_stm(for_body, semantic_data));

            stm_list.push(Stm_::loop_stm(
                trans_exp(test_exp, semantic_data).exp("".to_string()),
                Stm_::block_stm(loop_body),
                semantic_data.loop_index,
                true,
            ));
            new_stm = Stm_::block_stm(stm_list);
        }
        ASTStmData::Dec(dec) => match &dec.data {
            DecData::Var(_, _, _) => {
                new_stm = trans_dec(dec, None, semantic_data).stm(
                    format_args!("Failed to analyze dec semantically: {:?}", dec.pos).to_string(),
                );
            }
            DecData::Func(..) => {
                let _ = writeln!(
                    stderr(),
                    "Cannot define functions inside function body: {:?}",
                    dec.pos
                );
                new_stm = Stm_::none_stm();
            }
            _ => {
                let _ = writeln!(stderr(), "Please declare a variable: {:?}", dec.pos);
                new_stm = Stm_::none_stm();
            }
        },
        ASTStmData::Return(value) => {
            new_stm = Stm_::return_stm(trans_exp(value, semantic_data).exp("".to_string()))
        }
        ASTStmData::Repeat(limit, repeat_body) => {
            let init = ASTStm_::dec_stm(
                stm.pos,
                Dec_::var_dec(
                    stm.pos,
                    Var_::simple_var(stm.pos, "カウンタ".to_string()),
                    Type_::int_type(stm.pos),
                    ASTExp_::int_exp(stm.pos, "0".to_string()),
                ),
            );
            let mut stm_list = vec![];
            stm_list.push(trans_stm(&init, semantic_data));
            let mut loop_body = vec![];
            let incr = ASTStm_::assign_stm(
                stm.pos,
                Var_::simple_var(stm.pos, "カウンタ".to_string()),
                ASTExp_::int_exp(stm.pos, "1".to_string()),
                AssignType::Add,
            );
            loop_body.push(trans_stm(&incr, semantic_data));
            loop_body.push(trans_stm(repeat_body, semantic_data));
            let test_exp = ASTExp_::binop_exp(
                stm.pos,
                vec![Oper::Eq],
                vec![
                    ASTExp_::var_exp(stm.pos, Var_::simple_var(stm.pos, "カウンタ".to_string())),
                    limit.clone(),
                ],
            );
            stm_list.push(Stm_::loop_stm(
                trans_exp(&test_exp, semantic_data).exp("".to_string()),
                Stm_::block_stm(loop_body),
                semantic_data.loop_index,
                true,
            ));
            new_stm = Stm_::block_stm(stm_list);
        }
        _ => {
            new_stm = Stm_::none_stm();
        }
    }
    if semantic_data.temp_stmlist.len() > 0 {
        let mut block = replace(&mut semantic_data.temp_stmlist, vec![]);
        block.push(new_stm);
        Stm_::block_stm(block)
    } else {
        new_stm
    }
}

pub fn trans_if_stm(
    ifelselist: &IfElseList,
    semantic_data: &mut SemanticParam,
    stm_pos: (usize, usize),
) -> Stm {
    let mut ifelse_iter = ifelselist.iter();
    let mut result_stm;
    if ifelselist.len() > 0 {
        if let IfElseData::If(test_exp, if_body) = &ifelselist[0].data {
            result_stm = Stm_::if_stm(
                trans_exp(&test_exp, semantic_data).exp("".to_string()),
                trans_stm(&if_body, semantic_data),
                Stm_::none_stm(),
            );
            ifelse_iter.next();
            while let Some(ifelse) = ifelse_iter.next() {
                match &ifelse.data {
                    IfElseData::ElseIf(test_exp, elseif_body) => {
                        result_stm.set_if_else_body(
                            Stm_::if_stm(
                                trans_exp(&test_exp, semantic_data).exp("".to_string()),
                                trans_stm(&elseif_body, semantic_data),
                                Stm_::none_stm(),
                            ),
                            stm_pos,
                        );
                    }
                    IfElseData::Else(else_body) => {
                        result_stm.set_if_else_body(trans_stm(&else_body, semantic_data), stm_pos);
                    }
                    IfElseData::If(..) => {
                        let _ =
                            writeln!(stderr(), "Do not connect if statements: {:?}", ifelse.pos);
                    }
                }
            }
        } else {
            result_stm = Stm_::none_stm();
        }
    } else {
        result_stm = Stm_::none_stm();
    }
    result_stm
}

pub fn trans_assign_stm(
    var: &Var,
    init: &ASTExp,
    assign_type: &AssignType,
    semantic_data: &mut SemanticParam,
) -> Stm {
    let new_stm;
    let oper = match assign_type {
        AssignType::Add => Oper::Plus,
        AssignType::Sub => Oper::Minus,
        AssignType::Mul => Oper::Times,
        AssignType::Div => Oper::Divide,
        _ => Oper::None,
    };
    let added_init = if let Oper::None = oper {
        init.clone()
    } else {
        ASTExp_::binop_exp(
            init.pos,
            vec![oper],
            vec![ASTExp_::var_exp(var.pos, var.clone()), init.clone()],
        )
    };
    match &var.data {
        VarData::SuffixVar(var, suffixlist) => {
            new_stm = Stm_::store_stm(
                trans_suffix_var_to_addr(var, suffixlist, semantic_data)
                    .exp(format_args!("Var does not have address: {:?}", var.pos).to_string()),
                trans_exp(&added_init, semantic_data).exp("".to_string()),
            );
        }
        VarData::Pointer(var) => {
            new_stm = Stm_::store_stm(
                trans_right_var(var, semantic_data)
                    .exp(format_args!("Var does not have address: {:?}", var.pos).to_string()),
                trans_exp(&added_init, semantic_data).exp("".to_string()),
            );
        }
        VarData::Simple(name) => {
            // TODO: Dont' use clone
            let var_type;
            let access;
            {
                let var_entry = semantic_data.venv.get_data(name);
                (var_type, access) = if let Some(EnvEntry::Var(ty, access)) = var_entry {
                    (ty.clone(), access.clone())
                } else {
                    (LazeType_::none_type(), FrameAccess::None)
                };
            }
            if var_type.data != LazeTypeData::None && access != FrameAccess::None {
                match access {
                    FrameAccess::InLocal(index) => {
                        new_stm = Stm_::setlocal_stm(
                            index,
                            trans_exp(&added_init, semantic_data).exp("".to_string()),
                        );
                    }
                    FrameAccess::InGlobal(index) => {
                        new_stm = Stm_::setglobal_stm(
                            index,
                            trans_exp(&added_init, semantic_data).exp("".to_string()),
                        );
                    }
                    FrameAccess::InFrame(memory_offset, frame_offset)
                    | FrameAccess::EscapedParam(_, memory_offset, frame_offset) => {
                        if var_type.escape == false {
                            new_stm = Stm_::store_stm(
                                Exp_::consti32_exp(memory_offset + frame_offset),
                                trans_exp(&added_init, semantic_data).exp("".to_string()),
                            );
                        } else {
                            // match added_init.data {
                            //     ASTExpData::Array(explist) => {
                            //         new_stm = Stm_::
                            //     }
                            // }
                            new_stm = Stm_::copy_stm(
                                Exp_::consti32_exp(memory_offset + frame_offset),
                                trans_exp(&added_init, semantic_data).exp("".to_string()),
                                Exp_::consti32_exp(var_type.size),
                            );
                        }
                    }
                    FrameAccess::None => {
                        let _ = writeln!(stderr(), "{:?} is not in scope: {:?}", name, var.pos);
                        new_stm = Stm_::none_stm();
                    }
                }
            } else {
                let checked_var = check_member(var, semantic_data);
                if let Some(checked_var_exists) = checked_var {
                    return trans_assign_stm(
                        &checked_var_exists,
                        &added_init,
                        &AssignType::Normal,
                        semantic_data,
                    );
                } else {
                    let _ = writeln!(stderr(), "{:?} is not a variable: {:?}", name, var.pos);
                    new_stm = Stm_::none_stm();
                }
            }
        }
        _ => {
            new_stm = Stm_::none_stm();
        }
    }
    new_stm
}

pub fn trans_stmlist(stmlist: &ASTStmList, semantic_data: &mut SemanticParam) -> StmList {
    let mut result = vec![];
    for stm in stmlist {
        result.push(trans_stm(stm, semantic_data));
    }
    result
}
