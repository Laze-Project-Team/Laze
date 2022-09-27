use std::rc::Rc;

use crate::{
    ast::dec::{Dec, MemberSpecifier},
    wasm::frame::frame::{Frame, FrameAccess},
};

use super::trans_ty::{LazeType, LazeTypeList};

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
    pub fn get_data(&self, id: &String) -> &EnvEntry {
        for pair in self.data.iter().rev() {
            if pair.0 == *id {
                return &pair.1;
            }
        }
        return &EnvEntry::None;
    }
    pub fn add_data(&mut self, id: String, data: EnvEntry) {
        self.data.push((id, data));
    }
}

pub enum EnvEntry {
    Var(LazeType, FrameAccess),
    Func(i32, LazeTypeList, LazeType, FrameAccess, Frame),
    Template(Dec, TemplateMap, Rc<EntryMap>, Rc<EntryMap>),
    Class(String, EntryMap, EntryMap, i32),
    Poly(LazeType),
    Member(MemberSpecifier, LazeType, i32),
    None,
}

pub struct TemplateMap {
    map: Vec<(LazeTypeList, EnvEntry)>,
}

impl TemplateMap {
    pub fn get_data<'a>(&'a self, type_param: &LazeTypeList) -> &'a EnvEntry {
        for data in &self.map {
            if type_param == &data.0 {
                return &data.1;
            }
        }
        &EnvEntry::None
    }
}
