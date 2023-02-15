use std::{
    i32,
    io::{stderr, Write},
};

use crate::{
    ast::{
        exp::{ASTExp, ASTExpData, ASTExpList, ASTExp_},
        op::Oper,
        var::{Var, VarData},
    },
    wasm::{
        frame::frame::FrameAccess,
        il::{
            exp::{Exp, ExpList, Exp_},
            stm::{StmList, Stm_},
            util::{BinOper, WasmExpTy, WasmType},
        },
    },
};

use super::{
    laze_type::{comp_type_binop, LazeType, LazeTypeData, LazeTypeList, LazeType_},
    semantic_param::SemanticParam,
    trans_var::trans_suffix_var_to_addr,
};

pub fn trans_exp(exp: &ASTExp, semantic_data: &mut SemanticParam) -> WasmExpTy {
    match &exp.data {
        ASTExpData::Array(explist) => {
            let (access, ty, mut init_stmlist) = trans_arrayexp_to_stm(explist, semantic_data);
            semantic_data.temp_stmlist.append(&mut init_stmlist);
            WasmExpTy::new_exp(ty, Exp_::consti32_exp(access.get_address()))
        }
        ASTExpData::BinOp(operlist, explist) => {
            let mut operlist_iter = operlist.iter();
            let mut explist_iter = explist.iter();
            let mut result_exp;
            let mut result_ty;
            if let Some(exp) = explist_iter.next() {
                (result_ty, result_exp) =
                    trans_exp(exp, semantic_data).ty_exp(format_args!("{:?}", exp.pos).to_string());
            } else {
                result_ty = LazeType_::none_type();
                result_exp = Exp_::none_exp();
            }
            while let Some(right_exp) = explist_iter.next() {
                if let Some(op) = operlist_iter.next() {
                    (result_ty, result_exp) =
                        trans_binop_exp(op, result_exp, result_ty, right_exp, semantic_data)
                            .ty_exp(format_args!("{:?}", exp.pos).to_string());
                }
            }
            WasmExpTy::new_exp(result_ty, result_exp)
        }
        ASTExpData::Bool(boolean) => WasmExpTy::new_exp(
            LazeType_::bool_type(),
            if *boolean {
                Exp_::consti32_exp(1)
            } else {
                Exp_::consti32_exp(0)
            },
        ),
        ASTExpData::Char(c) => {
            WasmExpTy::new_exp(LazeType_::char_type(), Exp_::consti32_exp(*c as i32))
        }
        ASTExpData::Func(_params, _result, _func_body) => {
            // Function Expression needs to be supported
            WasmExpTy::new_exp(LazeType_::none_type(), Exp_::none_exp())
        }
        ASTExpData::Int(i) => {
            let int_data = i.parse::<i64>();
            if let Ok(data) = int_data {
                WasmExpTy::new_exp(LazeType_::int_type(), Exp_::consti64_exp(data))
            } else {
                WasmExpTy::new_exp(LazeType_::none_type(), Exp_::none_exp())
            }
        }
        ASTExpData::Paren(exp) => trans_exp(exp, semantic_data),
        ASTExpData::Real(r) => {
            let real_data = r.parse::<f64>();
            if let Ok(data) = real_data {
                WasmExpTy::new_exp(LazeType_::real_type(), Exp_::constf64_exp(data))
            } else {
                WasmExpTy::new_exp(LazeType_::none_type(), Exp_::none_exp())
            }
        }
        ASTExpData::Short(s) => {
            let short_data = s.parse::<i32>();
            if let Ok(data) = short_data {
                WasmExpTy::new_exp(LazeType_::short_type(), Exp_::consti32_exp(data))
            } else {
                WasmExpTy::new_exp(LazeType_::none_type(), Exp_::none_exp())
            }
        }
        ASTExpData::SizeOf(exp) => WasmExpTy::new_exp(
            LazeType_::short_type(),
            Exp_::consti32_exp(trans_exp(exp, semantic_data).ty.size),
        ),
        ASTExpData::String(exp) => {
            // using unwrap
            let string_address = semantic_data.get_mem_size();
            let string_type = LazeType_::array_type(LazeType_::char_type(), exp.len() as i32);
            for c in exp.chars() {
                let access = semantic_data
                    .frame
                    .last_mut()
                    .unwrap()
                    .alloc_inframe(&LazeType_::char_type());
                semantic_data.temp_stmlist.push(Stm_::store_stm(
                    Exp_::consti32_exp(access.get_address()),
                    Exp_::consti32_exp(c as i32),
                ));
            }
            WasmExpTy::new_exp(string_type, Exp_::consti32_exp(string_address))
        }
        ASTExpData::UnaryOp(oper_list, calc_exp) => {
            if oper_list.len() > 1 {
                let _ = writeln!(stderr(), "Warning: Laze doesn't support multiple unary operators, put parantheses around the expression: {:?}", exp.pos);
                trans_unaryop_exp(&oper_list[0], calc_exp, semantic_data)
            } else if oper_list.len() == 1 {
                trans_unaryop_exp(&oper_list[0], calc_exp, semantic_data)
            } else {
                trans_exp(calc_exp, semantic_data)
            }
        }
        ASTExpData::Var(var) => match &var.data {
            VarData::Simple(..) => trans_suffix_var_to_addr(var, &vec![], semantic_data),
            VarData::SuffixVar(var, suffixlist) => {
                trans_suffix_var_to_addr(&var, &suffixlist, semantic_data)
            }
            VarData::Pointer(pointer_var) => trans_exp(
                &ASTExp_::unaryop_exp(
                    var.pos,
                    vec![Oper::Deref],
                    // this statement won't be called that much so it should be okay.
                    ASTExp_::var_exp(pointer_var.pos, pointer_var.clone()),
                ),
                semantic_data,
            ),
            VarData::None => WasmExpTy::new_exp(LazeType_::none_type(), Exp_::none_exp()),
        },
        _ => WasmExpTy::new_exp(LazeType_::none_type(), Exp_::none_exp()),
    }
}

pub fn trans_explist(
    explist: &ASTExpList,
    semantic_data: &mut SemanticParam,
) -> (LazeTypeList, ExpList) {
    let mut explist_result = vec![];
    let mut tylist_result = vec![];

    for exp in explist {
        let (ty, exp) = trans_exp(exp, semantic_data).ty_exp("".to_string());
        tylist_result.push(ty);
        explist_result.push(exp);
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

pub fn trans_arrayexp_to_stm(
    explist: &ASTExpList,
    semantic_data: &mut SemanticParam,
) -> (FrameAccess, LazeType, StmList) {
    if explist.len() > 0 {
        let mut init_stmlist = vec![];
        let flat_explist = flatten_explist(explist);
        let first_elem = trans_exp(&flat_explist[0], semantic_data);
        let first_access = semantic_data
            .frame
            .last_mut()
            .unwrap()
            .alloc(&first_elem.ty);
        for exp in flat_explist.iter().skip(1) {
            let elem = trans_exp(exp, semantic_data);
            if first_elem.ty != elem.ty {
                let _ = writeln!(
                    stderr(),
                    "This expression's type does not match with the first element's type."
                );
            } else {
                let access = semantic_data
                    .frame
                    .last_mut()
                    .unwrap()
                    .alloc_inframe(&first_elem.ty);
                init_stmlist.push(Stm_::store_stm(
                    Exp_::consti32_exp(access.get_address()),
                    elem.exp("".to_string()),
                ));
            }
        }
        (first_access, first_elem.ty, init_stmlist)
    } else {
        (FrameAccess::None, LazeType_::none_type(), vec![])
    }
}

pub fn flatten_explist(explist: &ASTExpList) -> ASTExpList {
    let mut result_explist = vec![];
    for exp in explist {
        if let ASTExpData::Array(elist) = &exp.data {
            let mut list = flatten_explist(&elist);
            result_explist.append(&mut list);
        } else {
            result_explist.push(exp.clone());
        }
    }
    result_explist
}

pub fn trans_binop_exp(
    oper: &Oper,
    left: Exp,
    left_ty: LazeType,
    right: &ASTExp,
    semantic_data: &mut SemanticParam,
) -> WasmExpTy {
    let (right_ty, right_exp) = trans_exp(right, semantic_data).ty_exp("".to_string());
    if let Some((ty, lhs, rhs)) = comp_type_binop(left_ty, left, right_ty, right_exp) {
        let wasm_type = ty.to_wasm_type();
        match oper {
            Oper::Plus
            | Oper::Minus
            | Oper::Times
            | Oper::Divide
            | Oper::Mod
            | Oper::Ge
            | Oper::Gt
            | Oper::Le
            | Oper::Lt
            | Oper::Eq
            | Oper::Neq
            | Oper::And
            | Oper::Or => WasmExpTy::new_exp(
                ty,
                Exp_::binop_exp(wasm_type, BinOper::from_ast(&oper), lhs, rhs),
            ),
            _ => {
                let _ = writeln!(
                    stderr(),
                    "This operator is not functional right now: {:?}",
                    right.pos
                );
                WasmExpTy::new_exp(LazeType_::none_type(), Exp_::none_exp())
            }
        }
    } else {
        WasmExpTy::new_exp(LazeType_::none_type(), Exp_::none_exp())
    }
}

pub fn trans_unaryop_exp(
    oper: &Oper,
    exp: &ASTExp,
    semantic_data: &mut SemanticParam,
) -> WasmExpTy {
    match oper {
        Oper::Deref => match &exp.data {
            ASTExpData::Var(var) => {
                let (result_ty, result_exp) =
                    trans_suffix_var_to_addr(&var, &vec![], semantic_data).ty_exp("".to_string());
                if let LazeTypeData::Pointer(pointer_ty) = result_ty.data {
                    let wasm_ty = pointer_ty.to_wasm_type();
                    WasmExpTy::new_exp(pointer_ty, Exp_::load_exp(wasm_ty, result_exp))
                } else {
                    let _ = writeln!(
                        stderr(),
                        "Cannot dereference a non-pointer type: {:?}",
                        exp.pos
                    );
                    WasmExpTy::new_exp(LazeType_::none_type(), Exp_::none_exp())
                }
            }
            _ => {
                let _ = writeln!(
                    stderr(),
                    "Cannot dereference a non-variable expression: {:?}",
                    exp.pos
                );
                WasmExpTy::new_exp(LazeType_::none_type(), Exp_::none_exp())
            }
        },
        Oper::Address => match &exp.data {
            // handle non-framed variables
            ASTExpData::Var(var) => trans_suffix_var_to_addr(&var, &vec![], semantic_data),
            _ => {
                let _ = writeln!(
                    stderr(),
                    "Cannot get the address of a non-variable expression: {:?}",
                    exp.pos
                );
                WasmExpTy::new_exp(LazeType_::none_type(), Exp_::none_exp())
            }
        },
        Oper::Not => {
            let (result_ty, result_exp) = trans_exp(exp, semantic_data).ty_exp("".to_string());
            if let LazeTypeData::Bool = result_ty.data {
                WasmExpTy::new_exp(
                    LazeType_::bool_type(),
                    Exp_::binop_exp(
                        LazeType_::bool_type().to_wasm_type(),
                        BinOper::Sub,
                        Exp_::consti32_exp(0),
                        result_exp,
                    ),
                )
            } else {
                let _ = writeln!(
                    stderr(),
                    "Cannot get a Not expression of non-boolean type: {:?}",
                    exp.pos
                );
                WasmExpTy::new_exp(LazeType_::none_type(), Exp_::none_exp())
            }
        }
        _ => {
            let _ = writeln!(stderr(), "Not a unary operator: {:?}", exp.pos);
            WasmExpTy::new_exp(LazeType_::none_type(), Exp_::none_exp())
        }
    }
}
