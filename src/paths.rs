#![allow(unused)]
use std::{fs, path::PathBuf};

use crate::error;

fn lade_config_dir() -> Option<PathBuf> {
    dirs_next::config_dir().map(|p| p.join("lade"))
}

pub fn lade_package_list_main_path() -> PathBuf {
    lade_config_dir()
        .unwrap()
        .join("packagelists")
        .join("main.zip")
}

pub fn lade_package_list_extra_path() -> PathBuf {
    lade_config_dir()
        .unwrap()
        .join("packagelists")
        .join("extra.zip")
}

pub fn lade_package_list_path_dir() -> PathBuf {
    let p = lade_config_dir().unwrap().join("packagelists");
    if !p.exists() {
        fs::create_dir_all(&p).unwrap_or_else(|e| {
            error!(format!("Failed to create directory: {e}"));
        });
    }
    p
}

pub fn lade_bin_path() -> PathBuf {
    let p = lade_config_dir().unwrap().join("bin");

    if !p.exists() {
        fs::create_dir_all(&p).unwrap_or_else(|e| {
            error!(format!("Failed to create directory: {e}"));
        });
    }
    p
}

pub fn lade_build_path() -> PathBuf {
    let p = lade_config_dir().unwrap().join("build");

    if !p.exists() {
        fs::create_dir_all(&p).unwrap_or_else(|e| {
            error!(format!("Failed to create directory: {e}"));
        });
    }

    p
}

pub fn lade_build_git_path() -> PathBuf {
    let p = lade_build_path().join("git");
    if !p.exists() {
        fs::create_dir_all(&p).unwrap_or_else(|e| {
            error!(format!("Failed to create directory: {e}"));
        });
    }
    p
}

pub fn lade_downloaded_package_path() -> PathBuf {
    let p = lade_cache_path().join("packages");
    if !p.exists() {
        fs::create_dir_all(&p).unwrap_or_else(|e| {
            error!(format!("Failed to create directory: {e}"));
        });
    }
    p
}

pub fn lade_upgrade_info_path() -> PathBuf {
    let p = lade_cache_path().join("upgrade-info");
    if !p.exists() {
        fs::create_dir_all(&p).unwrap_or_else(|e| {
            error!(format!("Failed to create directory: {e}"));
        });
    }
    p
}

pub fn lade_cache_path() -> PathBuf {
    let p = lade_config_dir().unwrap().join("cache");
    if !p.exists() {
        fs::create_dir_all(&p).unwrap_or_else(|e| {
            error!(format!("Failed to create directory: {e}"));
        });
    }
    p
}

pub fn lade_log_path() -> PathBuf {
    lade_config_dir().unwrap().join("logs")
}

pub fn lade_packages_installed_path() -> PathBuf {
    lade_config_dir()
        .unwrap()
        .join("installed")
        .join("installed.json")
}

pub fn lade_packages_installed_dir_path() -> PathBuf {
    let p = lade_config_dir().unwrap().join("installed");
    if !p.exists() {
        fs::create_dir_all(&p).unwrap_or_else(|e| {
            error!(format!("Failed to create directory: {e}"));
        });
    }

    p
}

pub fn lade_licenses_path() -> PathBuf {
    let p = lade_config_dir().unwrap().join("Licenses");

    if !p.exists() {
        fs::create_dir_all(&p).unwrap_or_else(|e| {
            error!(format!("Failed to create {}: {}", &p.display(), e));
        });
    }

    p
}

pub fn lade_build_download_path() -> PathBuf {
    let p = lade_build_path().join("download").join("build");
    if !p.exists() {
        fs::create_dir_all(&p).unwrap_or_else(|e| {
            error!(format!("Failed to create {}: {}", &p.display(), e));
        });
    }

    p
}
