use crate::error;
use crate::paths;
use crate::version::Version;
use std::fs;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Package {
    pub name: String,
    pub version: Version,
    pub older_versions: Option<Vec<Version>>,
    pub description: String,
    pub license: String,
    pub authors: Vec<String>,
    pub dependencies: Vec<String>,
    pub repository: String,
    pub download_url: Option<DownloadUrls>,
    bin_name: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct DownloadUrls {
    pub windows: String,
    pub macos: String,
    pub linux: String,
}

pub fn installed() -> Vec<Package> {
    let path = paths::lade_packages_installed_path();

    let file = fs::read_to_string(&path).unwrap_or_else(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            fs::create_dir_all(paths::lade_packages_installed_dir_path()).unwrap_or_else(|e| {
                error!("Failed to create installed packages dir", &e);
            });

            fs::File::create(&path).unwrap_or_else(|e| {
                error!("Failed to create installed packages file", &e);
            });
            "".to_string()
        } else {
            error!("Failed to open installed packages file", e);
        }
    });

    if file.trim().is_empty() {
        vec![]
    } else {
        serde_json::from_str::<Vec<Package>>(&file).unwrap_or_else(|e| {
            error!("Failed to parse json", format!("Failed to parse PackageJson: {e}"));
        })
    }
}

pub fn already_installed(package: &str) -> bool {
    let installed = installed();

    installed
        .iter()
        .map(|a| a.name.as_str())
        .any(|name| name == package)
}

pub fn find(package: &str) -> Option<Package> {
    installed().into_iter().find(|p| p.name == package)
}

pub fn add_installed(package: Package) {
    update_installed(|installed| installed.push(package));
}

pub fn remove_installed_by_name(package: &str) {
    update_installed(|installed| installed.retain(|pkg| pkg.name != package));
}

fn update_installed<F: FnOnce(&mut Vec<Package>)>(f: F) {
    let path = paths::lade_packages_installed_path();
    let mut installed = installed();
    f(&mut installed);
    fs::write(
        path,
        serde_json::to_string::<Vec<Package>>(&installed).unwrap(),
    )
    .unwrap();
}

impl Package {
    pub fn bin_name(&self) -> String {
        if let Some(ref bin) = self.bin_name {
            bin.clone()
        } else {
            self.name.clone()
        }
    }
}
