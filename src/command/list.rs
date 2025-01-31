use colored::Colorize;
use std::fs;

use crate::{error, package_list_structs::Packages, paths::rade_package_list_path};

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

    for n in lade_list.packages {
        println!(
            "{} ({}{})",
            n.name,
            "v".bright_yellow(),
            n.version.bright_yellow()
        );
    }
}
