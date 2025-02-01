use rhai::Shared;
use std::cell::RefCell;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct RFile {
    path: PathBuf,
    file: fs::File,
}

impl Clone for RFile {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            file: self.file.try_clone().unwrap(),
        }
    }
}

pub fn open_file_share(file: &str) -> Shared<RefCell<RFile>> {
    Shared::new(RefCell::new(RFile::open_file(file)))
}

impl RFile {
    pub fn open_file(file: &str) -> Self {
        let path: &Path = file.as_ref();
        let file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path)
            .unwrap_or_else(|e| {
                eprintln!("Failed to open {file}: {e}");
                std::process::exit(1);
            });

        Self {
            path: path.to_path_buf(),
            file,
        }
    }

    

    pub fn write(&mut self, content: &str) {
        writeln!(&self.file, "{content}").unwrap_or_else(|e| {
            eprintln!("Failed to write {} file: {e}", self.path.display());
            std::process::exit(1);
        });
    }

    pub fn clear(&mut self) {
        self.file = fs::File::create(&self.path).unwrap_or_else(|e| {
            eprintln!("Failed to clear file: {e}");
            std::process::exit(1);
        });
    }

    pub fn read_to_string(file: &Shared<RefCell<Self>>) -> String {
        let mut file = file.borrow().clone().file;

        let mut content = String::new();

        file.read_to_string(&mut content).unwrap();

        content
    }
}
