use crate::{
    crash, err,
    package_list_structs::{PackageJson, Packages, RadePackage},
    paths::{lade_package_list_path, rade_package_list_path},
};
use colored::*;
use std::{ffi::OsStr, fs};

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
    let package_list_rade = rade_package_list_path();

    let dir_path = package_list_rade;
    let dir = match fs::read_dir(&dir_path) {
        Ok(dir) => dir,
        Err(e) => {
            err!(
                "
                Failed to retrieve package list.\n
                Please run `rade update` to retrieve package list.\n
                Error code:",
                e
            );
            crash!();
        }
    };

    let mut found: bool = false;
    for entry in dir.flatten() {
        if entry.file_name() == <&str as AsRef<OsStr>>::as_ref(&package) {
            found = true;

            let target = entry.path();

            if !target.is_dir() {
                found = false;
            }

            break;
        }
    }

    let mut package_rade: Option<RadePackage> = None;

    if found {
        let package_toml = dir_path.join(package).join("package.toml");
        let content = fs::read_to_string(&package_toml).unwrap_or_else(|e| {
            err!(format!("Failed to read {}", package_toml.display()), e);
            crash!();
        });

        package_rade = Some(toml::from_str(&content).unwrap_or_else(|e| {
            err!(format!("Failed to parse {}", package_toml.display()), e);
            crash!();
        }));
    }
    if !found {
        err!("Package not found");
        return None;
    } else {
        if let Some(package_r) = package_rade {
            return Some(package_r);
        } else {
            return None;
        }
    }
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
        crash!();
    }

    let package_lade = content.unwrap();
    let package_json_parsed: Packages = match serde_json::from_str(&package_lade) {
        Ok(parsed) => parsed,
        Err(e) => {
            err!("Failed to parse package list", e);
            crash!();
        }
    };

    for package_j in package_json_parsed.packages {
        if package_j.name == package {
            return Some(package_j);
        }
    }

    None
}
