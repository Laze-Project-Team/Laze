use std::{
    io::{stderr, Write},
    process::exit,
};

use crate::laze_parser::parser::LazeParser;

pub enum CompilerMode {
    Compile,
    Convert,
}
pub struct OptionCompilerInfo {
    pub mode: Option<CompilerMode>,
    pub parser: Option<LazeParser>,
    pub program_file_path: Option<String>,
    pub dist_file_path: Option<String>,
}

impl OptionCompilerInfo {
    pub fn new() -> Self {
        OptionCompilerInfo {
            mode: None,
            parser: None,
            program_file_path: None,
            dist_file_path: None,
        }
    }
}

pub struct CompilerInfo {
    pub mode: CompilerMode,
    pub parser: LazeParser,
    pub program_file_path: String,
    pub dist_file_path: String,
}

impl CompilerInfo {
    pub fn from_option(info: OptionCompilerInfo) -> Self {
        CompilerInfo {
            mode: match info.mode {
                Some(mode) => mode,
                None => {
                    let _ = writeln!(
                        stderr(),
                        "Please select a mode with the option: --compile / --convert"
                    );
                    exit(1);
                }
            },
            parser: match info.parser {
                Some(parser) => parser,
                None => {
                    let _ = writeln!(
                        stderr(),
                        "Please give a parser file as a parameter with: --parser [PATH]"
                    );
                    exit(1);
                }
            },
            program_file_path: match info.program_file_path {
                Some(path) => path,
                None => {
                    let _ = writeln!(
                        stderr(),
                        "Please specify a file path to convert / compile: lazec [FILEPATH]"
                    );
                    exit(1);
                }
            },
            dist_file_path: match info.dist_file_path {
                Some(path) => path,
                None => "".to_string(),
            },
        }
    }
}
