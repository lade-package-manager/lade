use crate::{
    crash, err,
    package_list_structs::{PackageJson, Packages, RadePackage},
    paths::{lade_package_list_path, rade_package_list_path},
};
use colored::*;
use std::fs;

<<<<<<< HEAD
#[derive(Debug)]
=======
>>>>>>> 4079977e07ff7d9059d2f14529ff85f2701247a3
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
<<<<<<< HEAD
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
=======
    let rade_package_list_dir = rade_package_list_path();

    let dir = match fs::read_dir(&rade_package_list_dir) {
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

    let found = dir
        .flatten()
        .find(|entry| entry.file_name() == <&str as AsRef<OsStr>>::as_ref(&package))
        .map_or(false, |e| e.path().is_dir());

    if !found {
        err!("Package not found");
        return None;
    }

    let package_toml = rade_package_list_dir.join(package).join("package.toml");
    let content = fs::read_to_string(&package_toml).unwrap_or_else(|e| {
        err!(format!("Failed to read {}", package_toml.display()), e);
        crash!();
    });

    let package_rade = Some(toml::from_str(&content).unwrap_or_else(|e| {
        err!(format!("Failed to parse {}", package_toml.display()), e);
        crash!();
    }));

    package_rade
>>>>>>> 4079977e07ff7d9059d2f14529ff85f2701247a3
}

pub fn search_package_lade(package: &str) -> Option<PackageJson> {
    let package_list_path = lade_package_list_path();

    let package_lade = match fs::read_to_string(package_list_path) {
        Ok(content) => content,
        Err(e) => {
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
    };

    let package_json_parsed: Packages = match serde_json::from_str(&package_lade) {
        Ok(parsed) => parsed,
        Err(e) => {
            err!("Failed to parse package list", e);
            crash!();
        }
    };

    package_json_parsed
        .packages
        .into_iter()
        .find(|package_j| package_j.name == package)
}
