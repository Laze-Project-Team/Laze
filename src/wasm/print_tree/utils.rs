use crate::wasm::il::util::WasmTypeList;

pub fn print_params(typelist: &WasmTypeList) -> String {
    let mut result = "".to_string();
    for ty in typelist {
        result += &format_args!("(param {})", ty.to_string()).to_string();
    }
    result
}

pub fn print_locals(typelist: &WasmTypeList) -> String {
    let mut result = "".to_string();
    for ty in typelist {
        result += &format_args!("(local {})", ty.to_string()).to_string();
    }
    result
}
