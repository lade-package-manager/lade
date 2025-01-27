#![allow(unused)]
use std::path::PathBuf;

pub fn lade_package_list_path() -> PathBuf {
    let mut path = dirs_next::config_dir().unwrap();
    path.push("lade");
    path.push("packagelists");
    path.push("lade");
    path.push("package_list.json");
    path
}

pub fn lade_package_list_path_dir() -> PathBuf {
    let mut path = dirs_next::config_dir().unwrap();
    path.push("lade");
    path.push("packagelists");
    path.push("lade");
    path
}

pub fn lade_bin_path() -> PathBuf {
    let mut path = dirs_next::config_dir().unwrap();
    path.push("lade");
    path.push("bin");
    path
}

pub fn lade_build_path() -> PathBuf {
    let mut path = dirs_next::config_dir().unwrap();
    path.push("lade");
    path.push("build");
    path
}

pub fn lade_cache_path() -> PathBuf {
    let mut path = dirs_next::config_dir().unwrap();
    path.push("lade");
    path.push("cache");
    path
}

pub fn rade_package_list_path() -> PathBuf {
    let mut path = dirs_next::config_dir().unwrap();
    path.push("lade");
    path.push("packagelists");
    path.push("rade");
    path
}

pub fn lade_log_path() -> PathBuf {
    let mut path = dirs_next::config_dir().unwrap();
    path.push("lade");
    path.push("logs");
    path.push("lade.log");
    path
}

pub fn lade_packages_installed_path() -> PathBuf {
    let mut path = dirs_next::config_dir().unwrap();
    path.push("lade");
    path.push("packages");
    path.push("installed.json");
    path
}
