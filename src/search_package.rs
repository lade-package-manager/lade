use crate::{
    error,
    package_list_structs::{LadePackage, RadePackage},
    paths::{lade_package_list_extra_path, lade_package_list_main_path, rade_package_list_path},
};
use std::{
    fs,
    io::{BufReader, Read},
};
use zip::ZipArchive;

#[derive(Debug)]
pub struct LRPackage {
    pub lade: Option<LadePackage>,
    pub rade: Option<RadePackage>,
}

pub fn search_package(package: &str) -> LRPackage {
    let lade_result = search_package_lade(package);
    if let Some(result) = lade_result {
        LRPackage {
            lade: Some(result),
            rade: None,
        }
    } else if let Some(s) = search_package_rade(package) {
        LRPackage {
            lade: None,
            rade: Some(s),
        }
    } else {
        LRPackage {
            lade: None,
            rade: None,
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

pub fn search_package_lade(package: &str) -> Option<LadePackage> {
    let package_list_paths = [
        lade_package_list_main_path(),
        lade_package_list_extra_path(),
    ];

    for package_list_path in package_list_paths {
        if package_list_path.exists() {
            // ZIPファイルを開く
            let file = fs::File::open(&package_list_path).unwrap_or_else(|e| {
                error!(
                    format!(
                        "Failed to open {}. Please update lade package list",
                        package_list_path.file_name().unwrap().to_str().unwrap()
                    ),
                    format!(
                        "Failed to open {}: {}",
                        package_list_path.file_name().unwrap().to_str().unwrap(),
                        e
                    ),
                    404
                );
            });

            let reader = BufReader::new(file);
            let mut archive = ZipArchive::new(reader).unwrap_or_else(|e| {
                error!(
                    "Failed to unzip archive",
                    format!("Failed to unzip archive: {}", e)
                );
            });

            let target_path = format!("{}/info.toml", package);

            // ZIP内のファイルを探索
            for i in 0..archive.len() {
                let mut file = archive.by_index(i).unwrap_or_else(|e| {
                    error!(
                        "Failed to read archive",
                        format!("Failed to read archive: {}", e)
                    );
                });

                if file.name() == target_path {
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap_or_else(|e| {
                        error!(
                            "Failed to read info.toml",
                            format!("Failed to read info.toml: {}", e)
                        );
                    });

                    let toml = toml::from_str(&contents).unwrap_or_else(|e| {
                        error!(
                            "Failed to parse info.toml",
                            format!("Failed to parse info.toml: {}", e)
                        );
                    });

                    return Some(toml);
                }
            }
        }
    }

    None
}
