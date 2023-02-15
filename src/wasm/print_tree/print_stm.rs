use crate::wasm::il::stm::{Stm, StmList, Stm_};

use super::print_exp::{print_exp, print_explist};

pub fn print_stm(stm: &Stm) -> String {
    match &**stm {
        Stm_::Block(stmlist) => format_args!("(block {})", print_stmlist(stmlist)).to_string(),
        Stm_::Break(index) => format_args!("(br {})", index).to_string(),
        Stm_::Call(index, args, _label) => {
            format_args!("(call {} {})", index, print_explist(args)).to_string()
        }
        Stm_::CallIndirect(func_index_exp, args, type_index) => format_args!(
            "(call_indirect (type {}) {} {})",
            type_index,
            print_explist(args),
            print_exp(func_index_exp)
        )
        .to_string(),
        Stm_::Copy(dest, src, size) => format_args!(
            "(memory.copy {} {} {})",
            print_exp(dest),
            print_exp(src),
            print_exp(size)
        )
        .to_string(),
        Stm_::If(test_exp, then_stm, else_stm) => {
            let mut result = "".to_string();
            result += &format_args!("(if {} (then {})", print_exp(test_exp), print_stm(then_stm),)
                .to_string();
            if let Stm_::None = **else_stm {
                result += ")";
            } else {
                result += &format_args!("(else {}))", print_stm(else_stm),).to_string()
            }
            result
        }
        Stm_::Loop(test_exp, loop_stm, loop_index, is_for) => {
            let mut result = "(loop ".to_string();
            if *is_for {
                result +=
                    &format_args!("(br_if {} {})", loop_index, print_exp(test_exp),).to_string();
                result += &print_stm(loop_stm);
            } else {
                result += &print_stm(loop_stm);
                result +=
                    &format_args!("(br_if {} {})", loop_index, print_exp(test_exp),).to_string();
            }
            result += &format_args!("(br {}))", loop_index - 1).to_string();
            result
        }
        Stm_::None => "".to_string(),
        Stm_::Return(return_exp) => format_args!("(return {})", print_exp(return_exp)).to_string(),
        Stm_::SetGlobal(index, exp) => {
            format_args!("(set_global {} {})", index, print_exp(exp)).to_string()
        }
        Stm_::SetLocal(index, exp) => {
            format_args!("(set_local {} {})", index, print_exp(exp)).to_string()
        }
        Stm_::Store(addr, value) => format_args!(
            "({}.store {} {})",
            value.ty.to_string(),
            print_exp(addr),
            print_exp(value)
        )
        .to_string(),
    }
}

pub fn print_stmlist(stmlist: &StmList) -> String {
    let mut result: String = "".to_string();
    for stm in stmlist {
        result += &print_stm(stm);
    }
    result
}
