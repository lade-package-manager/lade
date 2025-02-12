#![allow(unused)]
use std::path::PathBuf;

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
    lade_config_dir().unwrap().join("packagelists")
}

pub fn lade_bin_path() -> PathBuf {
    lade_config_dir().unwrap().join("bin")
}

pub fn lade_build_path() -> PathBuf {
    lade_config_dir().unwrap().join("build")
}

pub fn lade_build_git_path() -> PathBuf {
    lade_build_path().join("git")
}

pub fn lade_downloaded_package_path() -> PathBuf {
    lade_cache_path().join("packages")
}

pub fn lade_upgrade_info_path() -> PathBuf {
    lade_cache_path().join("upgrade-info")
}

pub fn lade_cache_path() -> PathBuf {
    lade_config_dir().unwrap().join("cache")
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
