use std::{cell::RefCell, path::{Path, PathBuf}};
use rhai::Shared;

use super::files::RFile;


#[derive(Clone)]
pub struct RPath{
    pub path: PathBuf
}

pub fn path(path: &str) -> Shared<RefCell<RPath>>{
    let path: &Path = path.as_ref();
    Shared::new(RefCell::new(RPath {
        path: path.to_path_buf()
    }))
}
impl RPath{
    pub fn to_string(file: Shared<RefCell<RPath>>) -> String{
        format!("{}", file.borrow().path.display())
    }

    pub fn exists(file: Shared<RefCell<RPath>>) -> bool{
        file.borrow().path.exists()
    }

    
    pub fn file_name(file: Shared<RefCell<RPath>>) -> String{
        let file_ref = file.borrow();
        let name = file_ref.path.file_name().unwrap_or_else(|| {
            eprintln!("Error: {}: file name is None", file_ref.path.display());
            std::process::exit(1);
        });

        name.to_str().unwrap().to_string()

    }

    pub fn read_file(file: Shared<RefCell<RPath>>) -> String{
        let rfile = RFile::open_file(file.borrow().path.to_str().unwrap());
        let content = RFile::read_to_string(&Shared::new(RefCell::new(rfile)));
        content
    }
                                                                                                                                                                                                                                                                                                                                                                                                          
}

