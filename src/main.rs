use std::{env, path::Path, time::Instant};

use command_handler::handler::handle_args;

use crate::wasm::semantic::trans_ast::trans_ast;

// use crate::wasm::semantic::trans_ast::trans_ast;

pub mod ast;
pub mod command_handler;
pub mod laze_parser;
pub mod util;
pub mod wasm;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let start = Instant::now();
    let mut info = handle_args(std::env::args());
    let ast = info.parser.parse(Path::new(&info.program_file_path));
    println!("{}ms", start.elapsed().as_millis());
    println!("{:?}", ast);
    let module_list = trans_ast(ast);
    println!("{}ms", start.elapsed().as_millis());
    println!("{:?}", module_list);
}
