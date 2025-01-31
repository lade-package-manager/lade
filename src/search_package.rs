use crate::{
    error,
    package_list_structs::LadePackage,
    paths::{lade_package_list_extra_path, lade_package_list_main_path},
};
use std::{
    fs,
    io::{BufReader, Read},
};
use zip::ZipArchive;

pub fn search_package_lade(package: &str) -> Option<LadePackage> {
    let package_list_paths = [
        lade_package_list_main_path(),
        lade_package_list_extra_path(),
    ];

    for package_list_path in package_list_paths {
        if !package_list_path.exists() {
            continue;
        }

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

            if file
                .enclosed_name()
                .filter(|p| p.to_string_lossy() == target_path)
                .is_some()
            {
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

    None
}
