use std::io::{stderr, Write};

use super::exp::{Exp, ExpList};

pub type StmList = Vec<Stm>;
pub type Stm = Box<Stm_>;

#[derive(Debug)]
pub enum Stm_ {
    If(Exp, Stm, Stm),
    Block(StmList),
    Loop(Exp, Stm, i32, bool),
    SetLocal(i32, Exp),
    SetGlobal(i32, Exp),
    Store(Exp, Exp),
    Break(i32),
    Call(i32, ExpList, Option<String>),
    CallIndirect(Exp, ExpList, i32),
    Return(Exp),
    Copy(Exp, Exp, Exp),
    None,
}

impl Stm_ {
    pub fn none_stm() -> Stm {
        Box::new(Stm_::None)
    }
    pub fn if_stm(test: Exp, if_body: Stm, else_body: Stm) -> Stm {
        Box::new(Stm_::If(test, if_body, else_body))
    }
    pub fn set_if_else_body(&mut self, new_else_body: Stm, stm_pos: (usize, usize)) {
        match self {
            Stm_::If(_, _, else_body) => {
                *else_body = new_else_body;
            }
            _ => {
                let _ = writeln!(
                    stderr(),
                    "This statement is not an if statement: {:?}",
                    stm_pos
                );
            }
        }
    }
    pub fn block_stm(body: StmList) -> Stm {
        Box::new(Stm_::Block(body))
    }
    pub fn loop_stm(test: Exp, body: Stm, index: i32, is_for: bool) -> Stm {
        Box::new(Stm_::Loop(test, body, index, is_for))
    }
    pub fn setlocal_stm(index: i32, exp: Exp) -> Stm {
        Box::new(Stm_::SetLocal(index, exp))
    }
    pub fn setglobal_stm(index: i32, exp: Exp) -> Stm {
        Box::new(Stm_::SetGlobal(index, exp))
    }
    pub fn store_stm(addr: Exp, exp: Exp) -> Stm {
        Box::new(Stm_::Store(addr, exp))
    }
    pub fn break_stm(index: i32) -> Stm {
        Box::new(Stm_::Break(index))
    }
    pub fn call_stm(index: i32, args: ExpList, label: Option<String>) -> Stm {
        Box::new(Stm_::Call(index, args, label))
    }
    pub fn call_indirect_stm(index: Exp, args: ExpList, type_index: i32) -> Stm {
        Box::new(Stm_::CallIndirect(index, args, type_index))
    }
    pub fn return_stm(value: Exp) -> Stm {
        Box::new(Stm_::Return(value))
    }
    pub fn copy_stm(dest: Exp, src: Exp, size: Exp) -> Stm {
        Box::new(Stm_::Copy(dest, src, size))
    }
}
