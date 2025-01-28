use crate::{
    err,
    package_list_structs::{PackageJson, Packages, RadePackage},
    paths::{lade_package_list_path, rade_package_list_path},
};
use colored::*;
use std::fs;

#[derive(Debug)]
pub struct LRPackage {
    pub lade: Option<PackageJson>,
    pub rade: Option<RadePackage>,
}

pub fn search_package(package: &str) -> LRPackage {
    let lade_result = search_package_lade(package);
    if let Some(result) = lade_result {
        return LRPackage {
            lade: Some(result),
            rade: None,
        };
    } else {
        if let Some(s) = search_package_rade(package) {
            return LRPackage {
                lade: None,
                rade: Some(s),
            };
        } else {
            return LRPackage {
                lade: None,
                rade: None,
            };
        }
    }
}

pub fn search_package_rade(package: &str) -> Option<RadePackage> {
    let dir_path = rade_package_list_path();
    let dir = fs::read_dir(&dir_path).ok()?;

    for entry in dir.flatten() {
        if entry.file_name() == package {
            let target = entry.path();

            if target.is_dir() {
                let package_toml = target.join("package.toml");
                let content = fs::read_to_string(&package_toml).ok()?;
                return toml::from_str(&content).ok();
            }
        }
    }

    None
}

pub fn search_package_lade(package: &str) -> Option<PackageJson> {
    let package_list_path = lade_package_list_path();

    let content = fs::read_to_string(package_list_path);

    if let Err(e) = content {
        err!(format!(
            "{}\n{}{}{}\nError code:{}",
            "Failed to retrieve package list.".bold(),
            "please run ".bold(),
            "lade update ".cyan(),
            "to retrieve package list.".bold(),
            e
        ));
        std::process::exit(1);
    }

    let package_lade = content.unwrap();
    let package_json_parsed: Packages = match serde_json::from_str(&package_lade) {
        Ok(parsed) => parsed,
        Err(e) => {
            err!("Failed to parse package list", e);
            std::process::exit(1);
        }
    };

    for package_j in package_json_parsed.packages {
        if package_j.name == package {
            return Some(package_j);
        }
    }

    None
}
