use std::path::Path;

use command_handler::handler::handle_args;

pub mod ast;
pub mod command_handler;
pub mod laze_parser;
pub mod util;
// pub mod IL;
// pub mod semantic;

fn main() {
    let mut info = handle_args(std::env::args());
    let ast = info.parser.parse(Path::new(&info.program_file_path));
    println!("{:?}", ast);
}
