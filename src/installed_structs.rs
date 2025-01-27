use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, Write},
};

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

        // if the file does not exist, create it
        let file = fs::read_to_string(&path)
            .map(|read| read.trim().to_string())
            .unwrap_or_else(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    fs::File::create(&path).unwrap_or_else(|e| {
                        error!("Failed to create installed packages file", &e);
                    });
                    "".to_string()
                } else {
                    error!("Failed to open installed packages file", e);
                }
            });

        if file.is_empty() {
            let n = Installed {
                last_update: chrono::Local::now().to_string(),
                packages: Vec::new(),
            };
            let mut file = fs::File::create(lade_packages_installed_path()).unwrap_or_else(|e| {
                error!(
                    "Failed to create Log file",
                    format!("Failed to create log file: {}", e)
                );
            });

            let content = serde_json::to_string(&n).unwrap_or_else(|e| {
                error!(
                    "Failed to create json",
                    format!("Failed to create json: {}", e)
                );
            });

            writeln!(file, "{}", content).unwrap_or_else(|e| {
                error!(
                    "Failed to write to log file",
                    format!("Failed to write to log file: {}", e)
                );
            });

            return n;
        }

        let installed: Installed = serde_json::from_str(&file).unwrap_or_else(|e| {
            error!("Failed to parse installed packages file", e);
        });

        installed
    }

    pub fn is_installed(package: &str) -> bool {
        let path = lade_packages_installed_path();
        let content = fs::read_to_string(&path).unwrap_or_else(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                (Installed::new());

                return fs::read_to_string(&path).unwrap_or_else(|e| {
                    error!(e, e);
                });
            }
            error!(
                format!("Failed to read {}: {}", path.display(), e),
                format!("Failed to read {}: {}", path.display(), e)
            );
        });

        let json: Installed = serde_json::from_str(&content).unwrap_or_else(|e| {
            error!(
                "Failed to parse json",
                format!("Failed to parse json: {}\nFile: {}", e, path.display())
            );
        });

        json.packages.into_iter().any(|pkg| pkg.name == package)
    }

    #[allow(warnings)]
    pub fn search_package(package: &str) -> Option<Package> {
        let content = fs::read_to_string(lade_packages_installed_path()).unwrap_or_else(|e| {
            error!("Failed to read installed.json", e);
        });

        let json: Installed = serde_json::from_str(&content).unwrap_or_else(|e| {
            error!("Failed to parse installed.json", e);
        });

        json.packages.into_iter().find(|n| n.name == package)
    }

    pub fn add_package(&mut self, package: Package) {
        self.packages.push(package);
        let json = serde_json::to_string(&self).unwrap_or_else(|e| {
            error!(e);
        });
        fs::write(lade_packages_installed_path(), json).unwrap_or_else(|_| {
            error!("Failed to write file", "Failed to write file");
        });
    }

    pub fn remove_package(&mut self, package: Package) {
        self.packages
            .clone()
            .into_iter()
            .enumerate()
            .for_each(|(i, s)| {
                if s == package {
                    self.packages.remove(i);
                }
            });

        let json = serde_json::to_string(&self).unwrap_or_else(|e| {
            error!(e, e);
        });
        fs::write(lade_packages_installed_path(), json).unwrap_or_else(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                fs::File::create(lade_packages_installed_path()).unwrap_or_else(|e| {
                    error!(
                        "Failed to create file",
                        format!("Failed to create log file: {}", e),
                        4
                    );
                });
            }
            error!(
                "Failed to remove package: Failed to write installed.json file",
                format!("Failed to write installed.json file ({})", e)
            );
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
