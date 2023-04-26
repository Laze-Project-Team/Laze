use std::io::{stderr, Write};

pub type CompileError = Box<CompileError_>;

#[derive(Debug)]
pub enum CompileError_ {
    Error((usize, usize), String),
    Warning((usize, usize), String),
}

impl CompileError_ {
    pub fn new_error(pos: (usize, usize), message: String) -> CompileError {
        Box::new(CompileError_::Error(pos, message))
    }

    pub fn new_warning(pos: (usize, usize), message: String) -> CompileError {
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
