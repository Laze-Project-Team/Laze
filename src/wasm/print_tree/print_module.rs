use std::{
    fs::File,
    io::{stderr, Write},
    path::Path,
};

use crate::wasm::il::{
    module::{Module, ModuleList, Module_},
    stm::Stm_,
    util::WasmType,
};

use super::{
    print_exp::print_exp,
    print_stm::print_stm,
    utils::{print_locals, print_params},
};

pub fn print_module(module: &Module) -> String {
    match &**module {
        Module_::Data(data, id) => {
            format_args!("(data {} \"{}\")", print_exp(&id), data).to_string()
        }
        Module_::Elem(offset, modlist) => {
            let mut result = format_args!("(elem {}", print_exp(&offset)).to_string();
            for module in modlist {
                if let Module_::Func(index, _, _, _, _, _) = **module {
                    result += &index.to_string();
                }
            }
            result += ")";
            result
        }
        Module_::Func(_, params, locals, return_type, body, export_name) => {
            let mut result = "(func ".to_string();
            if let Some(name) = export_name {
                result += &format_args!("(export \"{}\")", name).to_string();
            }
            result += &print_params(params).to_string();
            result += &print_locals(locals).to_string();
            if let WasmType::None = return_type {
            } else {
                result += &format_args!("(result {})", return_type.to_string()).to_string();
            }
            result += &print_stm(body).to_string();
            result += ")";
            result
        }
        Module_::Global(var_ty, init_val) => format_args!(
            "(global (mut {}) {}",
            var_ty.to_string(),
            print_exp(&init_val)
        )
        .to_string(),
        Module_::JsExport(export_name, func_index) => {
            format_args!("(export \"{}\" (func {}))", export_name, func_index).to_string()
        }
        Module_::JsImport(func_name, module_name, func_mod) => format_args!(
            "(import \"{}\" \"{}\" {})",
            module_name,
            func_name,
            print_modprototype(func_mod)
        )
        .to_string(),
        Module_::Memory(page_size) => format_args!("(memory {})", page_size).to_string(),
        Module_::None => "".to_string(),
        Module_::Table(table_size) => format_args!("(table {} anyfunc)", table_size).to_string(),
        Module_::Type(params, return_type) => format_args!(
            "(type {})",
            print_modprototype(&Module_::func_mod(
                -1,
                params.clone(),
                vec![],
                return_type.clone(),
                Stm_::none_stm(),
                None
            ))
        )
        .to_string(),
    }
}

pub fn print_modprototype(module: &Module) -> String {
    match &**module {
        Module_::Func(_, params, _, return_type, _, _) => {
            let mut result = "(func ".to_string();
            result += &print_params(params).to_string();
            if let WasmType::None = return_type {
            } else {
                result += &format_args!("(result {})", return_type.to_string()).to_string();
            }
            result += ")";
            result
        }
        Module_::Memory(page_num) => format_args!("(memory {})", page_num).to_string(),
        _ => "".to_string(),
    }
}

pub fn print_tree(modulelist: &ModuleList, mem_size: i32) -> String {
    let mut result = "(module ".to_string();
    for module in modulelist {
        result += &print_module(module).to_string();
    }
    result += &format_args!(
        "(func (export \"memorySize\") (result i32) (return (i32.const {})))",
        mem_size
    )
    .to_string();
    result += "(func (export \"clearMemory\") (memory.fill (i32.const 0) (i32.const 0) (i32.const 1114112)))";
    result += ")";
    result
}

pub fn fwrite_tree(
    modulelist: &ModuleList,
    mem_size: i32,
    file_path: &Path,
    dist_path: Option<&Path>,
) {
    let file_name_only = file_path.file_name();
    if let Some(file_name) = file_name_only {
        let dist_file = if let Some(dist) = dist_path {
            format_args!("{}", dist.to_str().unwrap(),).to_string()
        } else {
            format_args!("dist/{}.wat", file_name.to_str().unwrap()).to_string()
        };
        let file_to_write = File::create(&dist_file);
        if let Ok(mut file) = file_to_write {
            let result = file.write_all(print_tree(modulelist, mem_size).as_bytes());
            if let Err(error) = result {
                println!("Error while writing to file {}: {}", dist_file, error);
            }
        } else if let Err(error) = file_to_write {
            println!("Could not open file {}: {}", dist_file, error);
        }
    } else {
        let _ = writeln!(
            stderr(),
            "Could not get file name of {}",
            file_path.to_str().unwrap()
        );
    }
}
