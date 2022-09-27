use std::io::{stderr, Write};

use crate::{
    ast::{
        exp::{ASTExp, ASTExpList},
        var::Var,
    },
    wasm::{
        frame::frame::FrameAccess,
        il::{
            exp::{Exp, ExpList, Exp_},
            util::WasmType,
        },
    },
};

use super::trans_ty::{LazeType, LazeTypeList, LazeType_};

pub fn trans_exp(exp: &ASTExp) -> (LazeType, Exp) {
    (LazeType_::none_type(), Exp_::none_exp())
}

pub fn trans_explist(explist: &ASTExpList) -> (LazeTypeList, ExpList) {
    let mut explist_result = vec![];
    let mut tylist_result = vec![];

    for exp in explist {
        let (ty, e) = trans_exp(exp);
        explist_result.push(e);
        tylist_result.push(ty);
    }
    (tylist_result, explist_result)
}

pub fn trans_access_to_exp(access: &FrameAccess, var: &Var, name: &String) -> Exp {
    match access {
        FrameAccess::InFrame(memory_offset, frame_offset) => {
            Exp_::consti32_exp(memory_offset + frame_offset)
        }
        FrameAccess::InLocal(index) => Exp_::getlocal_exp(WasmType::I32, *index),
        FrameAccess::InGlobal(index) => Exp_::getglobal_exp(WasmType::I32, *index),
        FrameAccess::EscapedParam(_, memory_offset, frame_offset) => {
            Exp_::consti32_exp(memory_offset + frame_offset)
        }
        FrameAccess::None => {
            let _ = writeln!(
                stderr(),
                "{name} does not exist in frame, local, global, or parameters: {:?}",
                var.pos
            );
            Exp_::none_exp()
        }
    }
}
