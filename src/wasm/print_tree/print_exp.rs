use crate::wasm::il::{
    exp::{Exp, ExpData, ExpList},
    util::WasmType,
};

pub fn print_exp(exp: &Exp) -> String {
    match &exp.data {
        ExpData::BinOp(oper, lhs, rhs) => format_args!(
            "({}.{} {} {})",
            exp.ty.to_string(),
            oper.to_string(&exp.ty),
            print_exp(lhs),
            print_exp(rhs)
        )
        .to_string(),
        ExpData::CallExp(index, _label, args) => {
            format_args!("(call {} {})", index, print_explist(args)).to_string()
        }
        ExpData::CallIndirect(index, args, type_index) => format_args!(
            "(call_indirect (type {}) {} {})",
            type_index,
            print_explist(args),
            print_exp(index),
        )
        .to_string(),
        ExpData::Const(const_data) => {
            format_args!("({}.const {})", exp.ty.to_string(), const_data.to_string()).to_string()
        }
        ExpData::Convert(convert_exp) => match exp.ty {
            WasmType::F32 => match convert_exp.ty {
                WasmType::F32 => print_exp(convert_exp),
                WasmType::F64 => {
                    format_args!("(f32.demote_f64 {})", print_exp(convert_exp)).to_string()
                }
                WasmType::I32 => {
                    format_args!("(f32.convert_i32_s {})", print_exp(convert_exp)).to_string()
                }
                WasmType::I64 => {
                    format_args!("(f32.convert_i64_s {})", print_exp(convert_exp)).to_string()
                }
                WasmType::None => "".to_string(),
            },
            WasmType::F64 => match convert_exp.ty {
                WasmType::F32 => {
                    format_args!("(f64.promote_f32 {})", print_exp(convert_exp)).to_string()
                }
                WasmType::F64 => print_exp(convert_exp),
                WasmType::I32 => {
                    format_args!("(f64.convert_i32_s {})", print_exp(convert_exp)).to_string()
                }
                WasmType::I64 => {
                    format_args!("(f64.convert_i64_s {})", print_exp(convert_exp)).to_string()
                }
                WasmType::None => "".to_string(),
            },
            WasmType::I32 => match convert_exp.ty {
                WasmType::F32 => {
                    format_args!("(i32.trunc_f32_s {})", print_exp(convert_exp)).to_string()
                }
                WasmType::F64 => {
                    format_args!("(i32.trunc_f64_s {})", print_exp(convert_exp)).to_string()
                }
                WasmType::I32 => print_exp(convert_exp),
                WasmType::I64 => {
                    format_args!("(i32.wrap_i64 {})", print_exp(convert_exp)).to_string()
                }
                WasmType::None => "".to_string(),
            },
            WasmType::I64 => match convert_exp.ty {
                WasmType::F32 => {
                    format_args!("(i64.trunc_f32_s {})", print_exp(convert_exp)).to_string()
                }
                WasmType::F64 => {
                    format_args!("(i64.trunc_f64_s {})", print_exp(convert_exp)).to_string()
                }
                WasmType::I32 => {
                    format_args!("(i64.extend_i32_s {})", print_exp(convert_exp)).to_string()
                }
                WasmType::I64 => print_exp(convert_exp),
                WasmType::None => "".to_string(),
            },
            WasmType::None => "".to_string(),
        },
        ExpData::GetGlobal(index) => format_args!("(get_global {})", index).to_string(),
        ExpData::GetLocal(index) => format_args!("(local.get {})", index).to_string(),
        ExpData::IfExp(test_exp, then_exp, else_exp) => format_args!(
            "(if({}) {} (then {}) (else {}))",
            exp.ty.to_string(),
            print_exp(test_exp),
            print_exp(then_exp),
            print_exp(else_exp)
        )
        .to_string(),
        ExpData::Load(addr) => {
            format_args!("({}.load {})", exp.ty.to_string(), print_exp(addr)).to_string()
        }
        ExpData::None => "".to_string(),
        ExpData::UnaryOp(oper, op_exp) => format_args!(
            "({}.{} {})",
            exp.ty.to_string(),
            oper.to_string(),
            print_exp(op_exp)
        )
        .to_string(),
    }
}

pub fn print_explist(explist: &ExpList) -> String {
    let mut result: String = "".to_string();
    for exp in explist {
        result += &print_exp(exp);
    }
    result
}
