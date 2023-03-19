use crate::{
    ast::{
        dec::{Dec, DecData},
        exp::ASTExpData,
        field::Field_,
        stm::{AssignType, StmList, Stm_},
        ty::{Type, TypeData, Type_},
        var::Var_,
    },
    wasm::il::{
        module::{ModuleList, Module_},
        stm::Stm_ as WASMStm_,
        util::WasmExpTy,
    },
};

use super::{
    entry_map::{EntryMap, EnvEntry, TemplateMap},
    laze_type::LazeType_,
    semantic_param::SemanticParam,
    trans_funcdec::trans_funcdec,
    trans_stm::trans_stm,
    trans_ty::{trans_params, trans_result, trans_ty, trans_var_ty},
    trans_var::get_var_name,
};

pub fn trans_dec(
    dec: &Dec,
    parent_class: Option<&Type>,
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
            let export_name = if func_name == "実行" {
                Some("main".to_string())
            } else {
                None
            };
            let func_mod = trans_funcdec(
                func_body,
                params,
                &params_lazetype,
                return_var,
                &return_lazetype,
                export_name,
                semantic_data,
            );
            semantic_data.result_modlist.push(func_mod);
            semantic_data.func_num += 1;
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
            let import_mod = Module_::jsimport_mod(
                name.clone(),
                module_name.clone(),
                Module_::func_mod(
                    semantic_data.func_num,
                    LazeType_::list_to_wasm_type(&params_lazetype),
                    vec![],
                    return_lazetype.to_wasm_type(),
                    WASMStm_::none_stm(),
                    None,
                ),
            );
            semantic_data.result_modlist.push(import_mod);
            semantic_data.func_num += 1;
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
            let parent_class_type: Type;
            if let Some(EnvEntry::Template(_, template_map, _, _)) = class_entry {
                if let Some(class_type) = parent_class {
                    if let TypeData::Template(_, type_params) = &class_type.data {
                        parent_class_type =
                            Type_::template_type(dec.pos, class_name.clone(), type_params.clone());
                        template_map.add_data(
                            type_params.clone(),
                            EnvEntry::Class(
                                class_name.clone(),
                                members_entrymap.clone(),
                                class_size,
                            ),
                        );
                    } else {
                        return WasmExpTy::none();
                    }
                } else {
                    return WasmExpTy::none();
                }
            } else {
                parent_class_type = Type_::name_type(dec.pos, class_name.clone());
                semantic_data.tenv.add_data(
                    class_name.clone(),
                    EnvEntry::Class(class_name.clone(), members_entrymap.clone(), class_size),
                );
            }

            for member in member_list {
                match &member.dec.data {
                    DecData::Func(func_name, params, result, func_body)
                    | DecData::Oper(func_name, params, result, func_body) => {
                        let _new_frame =
                            semantic_data.new_frame(func_name, Some(&parent_class_type));
                        let self_param = Field_::new(
                            dec.pos,
                            Var_::simple_var(dec.pos, "self".to_string()),
                            Type_::pointer_type(dec.pos, parent_class_type.clone()),
                        );
                        let mut params_with_self = vec![self_param];
                        params_with_self.append(&mut params.clone());
                        let params_lazetype = trans_params(&params_with_self, semantic_data);
                        let (return_var, return_type) =
                            trans_result(dec.pos, result, semantic_data);
                        let func_mod = trans_funcdec(
                            func_body,
                            &params_with_self,
                            &params_lazetype,
                            return_var,
                            &return_type,
                            None,
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
