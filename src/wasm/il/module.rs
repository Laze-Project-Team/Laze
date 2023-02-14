use super::{
    exp::Exp,
    stm::Stm,
    util::{WasmType, WasmTypeList},
};

pub type ModuleList = Vec<Module>;
pub type Module = Box<Module_>;

#[derive(Debug)]
pub enum Module_ {
    Func(i32, WasmTypeList, WasmTypeList, WasmType, Stm),
    JsImport(String, String, Module),
    JsExport(String, i32),
    Global(WasmType, Exp),
    Data(String, Exp),
    Elem(Exp, ModuleList),
    Table(i32),
    Type(WasmTypeList, WasmType),
    None,
}

impl Module_ {
    pub fn none_mod() -> Module {
        Box::new(Module_::None)
    }
    pub fn func_mod(
        index: i32,
        params: WasmTypeList,
        local: WasmTypeList,
        result: WasmType,
        body: Stm,
    ) -> Module {
        Box::new(Module_::Func(index, params, local, result, body))
    }
    pub fn jsimport_mod(name: String, module_name: String, module: Module) -> Module {
        Box::new(Module_::JsImport(name, module_name, module))
    }
    pub fn jsexport_mod(name: String, index: i32) -> Module {
        Box::new(Module_::JsExport(name, index))
    }
    pub fn global_mod(ty: WasmType, exp: Exp) -> Module {
        Box::new(Module_::Global(ty, exp))
    }
    pub fn data_mod(data: String, exp: Exp) -> Module {
        Box::new(Module_::Data(data, exp))
    }
    pub fn elem_mod(offset: Exp, funcs: ModuleList) -> Module {
        Box::new(Module_::Elem(offset, funcs))
    }
    pub fn table_mod(size: i32) -> Module {
        Box::new(Module_::Table(size))
    }
    pub fn type_mod(params: WasmTypeList, result: WasmType) -> Module {
        Box::new(Module_::Type(params, result))
    }
}
