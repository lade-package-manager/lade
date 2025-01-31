use std::io::Write;
use std::{fs, path::Path};

pub fn write_file(file: &str, content: &str) {
    let path: &Path = file.as_ref();

    let mut file = fs::File::open(path).unwrap_or_else(|e| {
        eprintln!("Failed to open {file} file: {e}");
        std::process::exit(1);
    });

    writeln!(file, "{content}").unwrap_or_else(|e| {
        eprintln!("Failed to write {} file: {e}", path.display());
        std::process::exit(1);
    });
}

pub fn read_file(file: &str) -> String {
    let path: &Path = file.as_ref();

    fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to open {file} file: {e}");
        std::process::exit(1);
    })
}
