use colored::Colorize;
use std::fs;

use crate::{
    error,
    package_list_structs::{Packages, RadePackage},
    paths::rade_package_list_path,
};

pub fn list() {
    let lade_packagelist = rade_package_list_path();
    let content = fs::read_to_string(&lade_packagelist).unwrap_or_else(|e| {
        error!(
            format!("Failed to read {}", &lade_packagelist.display()),
            format!("Failed to read {}: {}", &lade_packagelist.display(), e)
        );
    });

    let lade_list: Packages = serde_json::from_str(&content).unwrap_or_else(|e| {
        error!(
            "Failed to parse json",
            format!("Failed to parse lade package list: {}", e)
        );
    });

    println!("------lade package lists------");
    for n in lade_list.packages {
        println!(
            "{} ({}{})",
            n.name,
            "v".bright_yellow(),
            n.version.bright_yellow()
        );
    }

    println!("------rade package lists------");

    let dir_path = rade_package_list_path();
    let dir = fs::read_dir(&dir_path).unwrap_or_else(|e| {
        error!(
            "Failed to read directory",
            format!("Failed to read rade_package_list_path: {}", e)
        );
    });

    for entry in dir.flatten() {
        let target = entry.path();
        if target.is_dir() && target.file_name().unwrap().to_str().unwrap() != ".git" {
            let package_toml = target.join("package.toml");
            let content = fs::read_to_string(&package_toml).unwrap_or_else(|e| {
                error!(
                    "Failed to read package.toml!",
                    format!("Failed to read {}: {}", package_toml.display(), e)
                );
            });

            let t: RadePackage = toml::from_str(&content).unwrap_or_else(|e| {
                error!(
                    "Failed to parse pacakge.toml",
                    format!("Failed to parse package.toml: {}", e)
                );
            });

            let file_name = target.file_name().unwrap().to_str().unwrap();
            println!("{} ({})", file_name, t.version.bright_yellow());
        }
    }
}
