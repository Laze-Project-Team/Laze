use std::io::{stderr, Write};

pub type CompileError = Box<CompileError_>;

#[derive(Debug)]
pub enum CompileError_ {
    Error((i32, i32), String),
    Warning((i32, i32), String),
}

impl CompileError_ {
    pub fn new_error(pos: (i32, i32), message: String) -> CompileError {
        Box::new(CompileError_::Error(pos, message))
    }

    pub fn new_warning(pos: (i32, i32), message: String) -> CompileError {
        Box::new(CompileError_::Warning(pos, message))
    }

    pub fn print(&self) {
        if let CompileError_::Error(pos, message) = self {
            let _ = writeln!(stderr(), "Error: {}, {}: {}", pos.0, pos.1, message);
        } else if let CompileError_::Warning(pos, message) = self {
            let _ = writeln!(stderr(), "Warning: {}, {}: {}", pos.0, pos.1, message);
        }
    }
}
