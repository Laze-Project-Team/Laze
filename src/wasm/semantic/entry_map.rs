use crate::{
    ast::dec::{Dec, MemberSpecifier},
    wasm::frame::frame::FrameAccess,
};

use super::laze_type::{LazeType, LazeTypeList};

#[derive(Clone, Debug)]
pub struct EntryMap {
    data: Vec<(String, EnvEntry)>,
}

impl EntryMap {
    pub fn new() -> Self {
        EntryMap { data: vec![] }
    }
    pub fn enter_scope(&mut self) {
        self.data.push(("<mark>".to_string(), EnvEntry::None));
    }
    pub fn exit_scope(&mut self) {
        while let Some(last) = self.data.last() {
            if last.0 == "<mark>".to_string() {
                self.data.pop();
                break;
            } else {
                self.data.pop();
            }
        }
    }
    pub fn get_data(&self, id: &String) -> Option<&EnvEntry> {
        for pair in self.data.iter().rev() {
            if pair.0 == *id {
                return Some(&pair.1);
            }
        }
        return None;
    }
    pub fn remove_data(&mut self, id: &String) {
        let mut found_index: i32 = -1;
        for (index, pair) in self.data.iter().rev().enumerate() {
            if pair.0 == *id {
                found_index = index as i32;
            }
        }
        if found_index >= 0 {
            self.data.remove(found_index as usize);
        }
    }
    pub fn get_mut_data(&mut self, id: &String) -> Option<&mut EnvEntry> {
        for pair in self.data.iter_mut().rev() {
            if pair.0 == *id {
                return Some(&mut pair.1);
            }
        }
        return None;
    }
    pub fn get_data_clone(&self, id: &String) -> Option<EnvEntry> {
        for pair in self.data.iter().rev() {
            if pair.0 == *id {
                return Some(pair.1.clone());
            }
        }
        return None;
    }
    pub fn add_data(&mut self, id: String, data: EnvEntry) {
        self.data.push((id, data));
    }
    pub fn add_data_return_mut(&mut self, id: String, data: EnvEntry) -> &mut EnvEntry {
        self.data.push((id, data));
        &mut self.data.last_mut().unwrap().1
    }
}

#[derive(Clone, Debug)]
pub enum EnvEntry {
    // var_type: LazeType, var_access: FrameAccess
    Var(LazeType, FrameAccess),
    // func_num: i32, params: LazeTypeList, return_type: LazeType, return_var_access: FrameAccess, func_frame: Frame
    Func(i32, LazeTypeList, LazeType),
    // base_dec: Dec, template_map: TemplateMap, venv_when_declared: EntryMap, type_params: Vec<String>
    Template(Dec, TemplateMap, EntryMap, Vec<String>),
    // class_name: String, members: Entrymap, size: i32
    Class(String, EntryMap, i32),
    // type_var_value: LazeType
    Poly(LazeType),
    // specifier: MemberSpecifier, member_type: LazeType, offset: i32
    Member(MemberSpecifier, LazeType, i32),
    // specifier: MemberSpecifier, func_num: i32, params: LazeTypeList, return_type: LazeType, return_var_access: FrameAccess, func_frame: Frame
    Method(MemberSpecifier, i32, LazeTypeList, LazeType),
    None,
}

#[derive(Clone, Debug)]
pub struct TemplateMap {
    map: Vec<(LazeTypeList, EnvEntry)>,
}

impl TemplateMap {
    pub fn new() -> TemplateMap {
        TemplateMap { map: vec![] }
    }
    pub fn get_data<'a>(&'a self, type_param: &LazeTypeList) -> Option<&'a EnvEntry> {
        for data in &self.map {
            if type_param == &data.0 {
                return Some(&data.1);
            }
        }
        None
    }
    pub fn add_data(&mut self, type_param: LazeTypeList, entry: EnvEntry) {
        self.map.push((type_param, entry));
    }
    pub fn add_data_return_mut(
        &mut self,
        type_param: LazeTypeList,
        entry: EnvEntry,
    ) -> &mut EnvEntry {
        self.map.push((type_param, entry));
        &mut self.map.last_mut().unwrap().1
    }
}
