#![allow(unused)]
use std::path::PathBuf;

fn lade_config_dir() -> Option<PathBuf> {
    dirs_next::config_dir().map(|p| p.join("lade"))
}

pub fn lade_package_list_main_path() -> PathBuf {
    lade_config_dir()
        .unwrap()
        .join("packagelists")
        .join("lade")
        .join("main.zip")
}

pub fn lade_package_list_extra_path() -> PathBuf {
    lade_config_dir()
        .unwrap()
        .join("packagelists")
        .join("lade")
        .join("extra.zip")
}

pub fn lade_package_list_path_dir() -> PathBuf {
    lade_config_dir().unwrap().join("packagelists").join("lade")
}

pub fn lade_bin_path() -> PathBuf {
    lade_config_dir().unwrap().join("bin")
}

pub fn lade_build_path() -> PathBuf {
    lade_config_dir().unwrap().join("build")
}

pub fn lade_cache_path() -> PathBuf {
    lade_config_dir().unwrap().join("cache")
}

pub fn rade_package_list_path() -> PathBuf {
    lade_config_dir().unwrap().join("packagelists").join("rade")
}

pub fn lade_log_path() -> PathBuf {
    lade_config_dir().unwrap().join("logs").join("lade.log")
}

pub fn lade_packages_installed_path() -> PathBuf {
    lade_config_dir()
        .unwrap()
        .join("installed")
        .join("installed.json")
}

pub fn lade_packages_installed_dir_path() -> PathBuf {
    lade_config_dir().unwrap().join("installed")
}
