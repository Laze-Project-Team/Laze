use crate::{
    ast::{
        dec::{Dec, DecData},
        exp::{ASTExpData, ASTExp_},
        field::{FieldData, FieldList},
        stm::{AssignType, StmList, Stm_},
        var::Var,
    },
    wasm::il::{
        module::{Module, ModuleList, Module_},
        stm::Stm_ as WASMStm_,
        util::WasmExpTy,
    },
};

use super::{
    entry_map::{EntryMap, EnvEntry, TemplateMap},
    laze_type::{LazeType, LazeTypeData, LazeTypeList, LazeType_},
    semantic_param::SemanticParam,
    trans_stm::{trans_stm, trans_stmlist},
    trans_ty::{trans_params, trans_result, trans_ty, trans_var_ty},
    trans_var::get_var_name,
};

pub fn trans_dec(
    dec: &Dec,
    parent_class: Option<&LazeType>,
    semantic_data: &mut SemanticParam,
) -> WasmExpTy {
    let mut _result_list: ModuleList = vec![];
    match &dec.data {
        DecData::Var(var, var_ty, init) => {
            let (new_var, new_var_ty, _object_explist) = trans_var_ty(var, var_ty);
            let var_lazetype = trans_ty(&new_var_ty, semantic_data);
            let new_var_access = semantic_data.frame.last_mut().unwrap().alloc(&var_lazetype);
            semantic_data.venv.add_data(
                get_var_name(new_var),
                EnvEntry::Var(var_lazetype, new_var_access),
            );
            WasmExpTy::new_stm(
                LazeType_::none_type(),
                trans_stm(
                    &Stm_::assign_stm(dec.pos, new_var.clone(), init.clone(), AssignType::Normal),
                    semantic_data,
                ),
            )
        }
        DecData::Func(func_name, params, result, func_body)
        | DecData::Oper(func_name, params, result, func_body) => {
            if let DecData::Oper(_, _, _, _) = &dec.data {
                if let None = parent_class {
                    return WasmExpTy::none();
                }
            }
            semantic_data.new_frame(&func_name, parent_class);
            let params_lazetype = trans_params(&params, semantic_data);
            let (return_var, return_lazetype) = trans_result(dec.pos, result, semantic_data);
            // TODO: need to implement function overloading
            semantic_data.venv.add_data(
                func_name.clone(),
                EnvEntry::Func(
                    semantic_data.func_num,
                    params_lazetype.clone(),
                    return_lazetype.clone(),
                ),
            );
            semantic_data.func_num += 1;
            let func_mod = trans_funcdec(
                func_body,
                params,
                &params_lazetype,
                return_var,
                &return_lazetype,
                semantic_data,
            );
            semantic_data.result_modlist.push(func_mod);
            WasmExpTy::none()
        }
        DecData::JsImport(func_name, params, result, module_name, name) => {
            semantic_data.new_frame(&func_name, parent_class);
            let params_lazetype = trans_params(&params, semantic_data);
            let (_, return_lazetype) = trans_result(dec.pos, result, semantic_data);
            let _return_access = semantic_data
                .frame
                .last_mut()
                .unwrap()
                .alloc(&return_lazetype);
            semantic_data.venv.add_data(
                func_name.clone(),
                EnvEntry::Func(
                    semantic_data.func_num,
                    params_lazetype.clone(),
                    return_lazetype.clone(),
                ),
            );
            semantic_data.func_num += 1;
            let import_mod = Module_::jsimport_mod(
                name.clone(),
                module_name.clone(),
                Module_::func_mod(
                    semantic_data.func_num,
                    LazeType_::list_to_wasm_type(&params_lazetype),
                    vec![],
                    return_lazetype.to_wasm_type(),
                    WASMStm_::none_stm(),
                ),
            );
            semantic_data.result_modlist.push(import_mod);
            WasmExpTy::none()
        }
        DecData::JsExport(func_name, export_name) => {
            let entry = semantic_data.venv.get_data(&func_name).expect(
                format_args!(
                    "Could not find function {:?} to export: {:?}",
                    func_name, dec.pos
                )
                .as_str()
                .unwrap(),
            );
            let export_mod = if let EnvEntry::Func(index, _, _) = entry {
                Module_::jsexport_mod(export_name.clone(), *index)
            } else {
                Module_::none_mod()
            };
            semantic_data.result_modlist.push(export_mod);
            WasmExpTy::none()
        }
        DecData::Class(class_name, member_list, _inheritance) => {
            let mut members_entrymap = EntryMap::new();
            let mut class_size = 0;
            let mut default_assignlist: StmList = vec![];
            // enter all members into members_entrymap
            for member in member_list {
                match &member.dec.data {
                    DecData::Var(var, ty, init) => {
                        let member_ty = trans_ty(ty, semantic_data);
                        class_size += member_ty.size;
                        members_entrymap.add_data(
                            get_var_name(var),
                            EnvEntry::Member(member.specifier, member_ty, class_size),
                        );
                        // initializtion in constructor
                        match &init.data {
                            ASTExpData::None => {}
                            _ => {
                                default_assignlist.push(Stm_::assign_stm(
                                    var.pos,
                                    var.clone(),
                                    init.clone(),
                                    AssignType::Normal,
                                ));
                            }
                        }
                    }
                    DecData::Func(func_name, params, result, _)
                    | DecData::Oper(func_name, params, result, _) => {
                        let mut params_lazetype = trans_params(&params, semantic_data);
                        params_lazetype.insert(0, LazeType_::pointer_type(LazeType_::void_type()));
                        let (_, return_type) = trans_result(dec.pos, result, semantic_data);
                        members_entrymap.add_data(
                            func_name.clone(),
                            EnvEntry::Method(
                                member.specifier,
                                semantic_data.func_num,
                                params_lazetype,
                                return_type,
                            ),
                        );
                    }
                    _ => {}
                }
            }
            let class_entry = semantic_data.tenv.get_mut_data(class_name);
            if let Some(EnvEntry::Template(_, template_map, _, _)) = class_entry {
                if let Some(parent_class_type) = parent_class {
                    if let LazeTypeData::Template(_, type_params) = &parent_class_type.data {
                        template_map.add_data(
                            type_params.clone(),
                            EnvEntry::Class(
                                class_name.clone(),
                                members_entrymap.clone(),
                                class_size,
                            ),
                        );
                    }
                }
            } else {
                semantic_data.tenv.add_data(
                    class_name.clone(),
                    EnvEntry::Class(class_name.clone(), members_entrymap.clone(), class_size),
                );
            }
            for member in member_list {
                match &member.dec.data {
                    DecData::Func(func_name, params, result, func_body)
                    | DecData::Oper(func_name, params, result, func_body) => {
                        let mut params_lazetype = trans_params(&params, semantic_data);
                        params_lazetype.insert(0, LazeType_::pointer_type(LazeType_::void_type()));
                        let (return_var, return_type) =
                            trans_result(dec.pos, result, semantic_data);
                        let parent_class_type =
                            LazeType_::class_type(class_name.clone(), class_size);
                        let _new_frame =
                            semantic_data.new_frame(func_name, Some(&parent_class_type));
                        let func_mod = trans_funcdec(
                            func_body,
                            params,
                            &params_lazetype,
                            return_var,
                            &return_type,
                            semantic_data,
                        );
                        semantic_data.result_modlist.push(func_mod);
                        semantic_data.func_num += 1;
                    }
                    _ => {}
                }
            }
            WasmExpTy::none()
        }
        DecData::Template(original_dec, type_params) => {
            match &original_dec.data {
                DecData::Class(name, _, _) | DecData::Func(name, _, _, _) => {
                    semantic_data.tenv.add_data(
                        name.clone(),
                        EnvEntry::Template(
                            original_dec.clone(),
                            TemplateMap::new(),
                            semantic_data.venv.clone(),
                            type_params.clone(),
                        ),
                    );
                }
                _ => {}
            };
            WasmExpTy::none()
        }
        DecData::None => WasmExpTy::none(),
    }
}

fn trans_funcdec(
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
    if let Some(var) = return_var {
        semantic_data.venv.add_data(
            get_var_name(&var),
            EnvEntry::Var(
                return_type.clone(),
                semantic_data.frame.last_mut().unwrap().alloc(&return_type),
            ),
        );
    }
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
                            .alloc(&params_lazetype[index]),
                    ),
                );
                // TODO: add copy statement
            }
            FieldData::None => {}
        }
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
