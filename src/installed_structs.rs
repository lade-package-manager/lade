use serde::{Deserialize, Serialize};
use std::fs;

use crate::{
    crash, err, error,
    paths::{self, lade_packages_installed_path},
    write_log,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Installed {
    pub last_update: String,
    pub packages: Vec<Package>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub license: String,
    pub authors: Vec<String>,
    pub dependencies: Vec<String>,
    pub repository: String,
    pub download: Option<String>,
    pub install_date: String,
    pub exec_name: String,
}

impl Installed {
    pub fn new() -> Installed {
        let path = paths::lade_packages_installed_path();

        let file = fs::read_to_string(&path).unwrap_or_else(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                fs::File::create(&path).unwrap_or_else(|e| {
                    error!("Failed to create installed packages file", &e);
                });
                "".to_string()
            } else {
                error!("Failed to open installed packages file", e);
            }
        });

        if file.trim().is_empty() {
            let n = Installed {
                last_update: chrono::Local::now().to_string(),
                packages: Vec::new(),
            };
            n.save();
            return n;
        }

        serde_json::from_str(&file).unwrap_or_else(|e| {
            error!("Failed to parse installed packages file", e);
        })
    }

    pub fn is_installed(package: &str) -> bool {
        let installed = Installed::new();
        installed.packages.iter().any(|pkg| pkg.name == package)
    }

    pub fn search_package(package: &str) -> Option<Package> {
        let installed = Installed::new();
        installed
            .packages
            .into_iter()
            .find(|pkg| pkg.name == package)
    }

    pub fn add_package(&mut self, package: Package) {
        self.packages.push(package);
        self.save();
    }

    pub fn remove_package_by_name(&mut self, package_name: &str) {
        self.packages.retain(|pkg| pkg.name != package_name);
        self.save();
    }

    pub fn save(&self) {
        let json = serde_json::to_string(&self).unwrap_or_else(|e| {
            error!(e);
        });
        fs::write(lade_packages_installed_path(), json).unwrap_or_else(|_| {
            error!("Failed to write file", "Failed to write file");
        });
    }
}

impl Package {
    pub fn new(
        name: String,
        version: String,
        description: String,
        license: String,
        authors: Vec<String>,
        dependencies: Vec<String>,
        repository: String,
        download: Option<String>,
        exec_name: String,
    ) -> Package {
        Package {
            name,
            version,
            description,
            license,
            authors,
            dependencies,
            repository,
            download,
            install_date: chrono::Local::now().to_string(),
            exec_name,
        }
    }
}
