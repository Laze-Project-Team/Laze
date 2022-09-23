use std::{collections::VecDeque, path::Path, process::exit};

use crate::{command_handler::compiler_info::CompilerMode, laze_parser::parser::LazeParser};

use super::compiler_info::{CompilerInfo, OptionCompilerInfo};

pub fn handle_args(arguments: std::env::Args) -> CompilerInfo {
    let mut args_vector: VecDeque<String> = VecDeque::new();
    let mut info = OptionCompilerInfo::new();
    for argument in arguments.skip(1) {
        args_vector.push_back(argument);
    }
    for arg in args_vector.clone() {
        // println!("{arg}");
        if arg.starts_with("--") {
            handle_double_dash(&mut args_vector, &mut info);
        } else if arg.starts_with("-") {
            handle_single_dash(&mut args_vector, &mut info);
        } else {
            info.program_file_path = Some(arg);
            args_vector.pop_front();
        }
    }
    CompilerInfo::from_option(info)
}

fn handle_double_dash(arguments: &mut VecDeque<String>, info: &mut OptionCompilerInfo) {
    assert!(arguments[0].starts_with("--"));
    arguments[0].drain(..2);
    let param = &arguments[0];
    match param.as_str() {
        "help" => {
            println!("Usage: lazec [Filename] [OPTIONS]");
            exit(0);
        }
        "compile" => info.mode = Some(CompilerMode::Compile),
        "convert" => info.mode = Some(CompilerMode::Convert),
        str => {
            // parser="PATH" or parser=PATH
            if str.starts_with("parser=") {
                let (_, path) = param.split_at(7);
                info.parser = Some(LazeParser::new(Path::new(path)));
            } else {
                println!("Unknown option: --{param}");
            }
        }
    }
    arguments.pop_front();
}

fn handle_single_dash(_: &mut VecDeque<String>, _: &mut OptionCompilerInfo) {
    ()
}
