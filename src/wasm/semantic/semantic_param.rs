use crate::wasm::{
    frame::frame::{Frame, FrameType, Frame_},
    il::{module::ModuleList, stm::StmList},
};

use super::{
    entry_map::EntryMap,
    laze_type::{LazeType, LazeTypeData},
};

pub struct SemanticParam {
    pub venv: EntryMap,
    pub tenv: EntryMap,
    pub loop_index: i32,
    pub func_num: i32,
    pub frame: Vec<Frame>,
    pub temp_stmlist: StmList,
    pub result_modlist: ModuleList,
}

impl SemanticParam {
    pub fn new() -> SemanticParam {
        SemanticParam {
            venv: EntryMap::new(),
            tenv: EntryMap::new(),
            loop_index: 0,
            func_num: 0,
            frame: vec![],
            temp_stmlist: vec![],
            result_modlist: vec![],
        }
    }
    pub fn get_mem_size(&self) -> i32 {
        if self.frame.len() > 0 {
            self.frame.last().unwrap().memory_offset + self.frame.last().unwrap().frame_size
        } else {
            0
        }
    }
    pub fn new_frame(&mut self, func_name: &String, class: Option<&LazeType>) -> &mut Frame {
        let new_frame = match class {
            Some(ty) => match &ty.data {
                LazeTypeData::Class(name) | LazeTypeData::Template(name, _) => Frame_::new(
                    self.get_mem_size(),
                    FrameType::Method(func_name.clone(), name.clone()),
                ),
                _ => Frame_::new(self.get_mem_size(), FrameType::Func(func_name.clone())),
            },
            None => Frame_::new(self.get_mem_size(), FrameType::Func(func_name.clone())),
        };
        self.frame.push(new_frame);
        self.frame.last_mut().unwrap()
    }
}
