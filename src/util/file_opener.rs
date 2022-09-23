use std::{
    fs::File,
    io::{stderr, Read, Write},
    path::Path,
    process::exit,
};

pub fn open_file(path: &Path) -> String {
    match File::open(path) {
        Ok(mut file) => {
            let mut file_content = String::new();
            let _ = file
                .read_to_string(&mut file_content)
                .expect("Reading file");
            file_content
        }
        Err(error) => {
            let _ = writeln!(
                stderr(),
                "Could not open file \"{}\" because of: {}",
                path.to_str().expect("Converting path to str"),
                error.to_string()
            );
            exit(1);
        }
    }
}
