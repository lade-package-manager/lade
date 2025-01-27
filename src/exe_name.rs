use std::fs;

use crate::{
    error,
    paths::lade_build_path,
};

pub fn get_exec_name() -> String {
    let path = lade_build_path().join("exec_name");
    if path.exists(){
        return fs::read_to_string(path).unwrap_or_else(|_| {
            error!("Failed to read exec_name", "Failed to read exec_name");
        });
    }
    "".to_string()
}
