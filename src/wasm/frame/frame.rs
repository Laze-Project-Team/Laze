use crate::wasm::semantic::trans_ty::LazeType;

pub type Frame = Box<Frame_>;

pub struct Frame_ {
    data: FrameType,
    locals: Vec<FrameAccess>,
    params: Vec<FrameAccess>,
    // memory offset of the frame
    memory_offset: i32,
    // total memory size of the frame
    frame_size: i32,
}

pub enum FrameType {
    Func(String),
    Method(String, String),
    Global,
}

impl Frame_ {
    pub fn new(memory_offset: i32, frame_type: FrameType) -> Frame {
        Box::new(Frame_ {
            data: frame_type,
            locals: vec![],
            params: vec![],
            memory_offset,
            frame_size: 0,
        })
    }
    pub fn alloc_param(&mut self, ty: LazeType) -> FrameAccess {
        let new_access: FrameAccess;
        if ty.escape {
            new_access = FrameAccess::EscapedParam(
                self.params.len() as i32,
                self.memory_offset,
                self.frame_size,
            );
            self.params.push(new_access);
            self.frame_size += ty.size;
        } else {
            new_access = FrameAccess::InLocal(self.params.len() as i32);
            self.params.push(new_access);
        }
        new_access
    }
    pub fn alloc(&mut self, ty: LazeType) -> FrameAccess {
        let new_access: FrameAccess;
        if ty.escape {
            new_access = FrameAccess::InFrame(self.memory_offset, self.frame_size);
            self.frame_size += ty.size;
        } else {
            match self.data {
                FrameType::Func(_) => {
                    new_access = FrameAccess::InLocal(self.locals.len() as i32);
                    self.locals.push(new_access);
                }
                FrameType::Method(_, _) => {
                    new_access = FrameAccess::InLocal(self.locals.len() as i32);
                    self.locals.push(new_access);
                }
                FrameType::Global => {
                    new_access = FrameAccess::InGlobal(self.locals.len() as i32);
                    self.locals.push(new_access);
                }
            }
        }
        new_access
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FrameAccess {
    EscapedParam(i32, i32, i32),
    InFrame(i32, i32),
    InLocal(i32),
    InGlobal(i32),
    None,
}

impl FrameAccess {
    pub fn get_address(&self) -> i32 {
        match self {
            Self::EscapedParam(_, memory_offset, frame_offset) => memory_offset + frame_offset,
            Self::InFrame(memory_offset, frame_offset) => memory_offset + frame_offset,
            _ => {
                panic!("The variable is not in frame.");
            }
        }
    }
}
