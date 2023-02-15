use crate::{
    ast::{
        exp::ASTExp_,
        field::{FieldData, FieldList},
        stm::{StmList, Stm_},
        var::Var,
    },
    wasm::il::{
        module::{Module, Module_},
        stm::Stm_ as WASMStm_,
    },
};

use super::{
    entry_map::EnvEntry,
    laze_type::{LazeType, LazeTypeList, LazeType_},
    semantic_param::SemanticParam,
    trans_stm::{trans_stm, trans_stmlist},
    trans_var::get_var_name,
};

pub fn trans_funcdec(
    func_body: &StmList,
    params: &FieldList,
    params_lazetype: &LazeTypeList,
    return_var: Option<&Var>,
    return_type: &LazeType,
    semantic_data: &mut SemanticParam,
) -> Module {
    //
    //start scope
    semantic_data.venv.enter_scope();
    //
    // add the return var to the venv
    for (index, param) in params.iter().enumerate() {
        match &param.data {
            FieldData::Field(var, _) => {
                // add the parameters to the venv
                semantic_data.venv.add_data(
                    get_var_name(&var),
                    EnvEntry::Var(
                        params_lazetype[index].clone(),
                        semantic_data
                            .frame
                            .last_mut()
                            .unwrap()
                            .alloc_param(&params_lazetype[index]),
                    ),
                );
                // TODO: add copy statement
            }
            FieldData::None => {}
        }
    }
    if let Some(var) = return_var {
        semantic_data.venv.add_data(
            get_var_name(&var),
            EnvEntry::Var(
                return_type.clone(),
                semantic_data.frame.last_mut().unwrap().alloc(&return_type),
            ),
        );
    }
    let mut result_body = vec![];
    result_body.append(&mut trans_stmlist(func_body, semantic_data));
    if let Some(var) = return_var {
        let return_stm = Stm_::return_stm(var.pos, ASTExp_::var_exp(var.pos, var.clone()));
        result_body.push(trans_stm(&return_stm, semantic_data));
    };
    //
    semantic_data.venv.exit_scope();
    //exit scope
    //
    Module_::func_mod(
        semantic_data.func_num,
        LazeType_::list_to_wasm_type(&params_lazetype),
        LazeType_::list_to_wasm_type(&semantic_data.frame.last().unwrap().locals_type),
        return_type.to_wasm_type(),
        WASMStm_::block_stm(result_body),
    )
}
