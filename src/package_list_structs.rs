use serde::{Deserialize, Serialize};

// ladeのパッケージ情報を格納するための構造体json
#[derive(Serialize, Deserialize, Debug)]
pub struct PackageJson {
    pub name: String,
    pub version: String,
    pub description: String,
    pub license: String,
    pub authors: Vec<String>,
    pub dependencies: Vec<String>,
    pub repository: String,
    pub download: Option<String>,
}

// ladeのパースされる本命struct
#[derive(Serialize, Deserialize)]
pub struct Packages {
    pub version: String,
    pub packages: Vec<PackageJson>,
}

// radeのパッケージ情報を格納するための構造体toml
#[derive(Serialize, Deserialize, Debug)]
pub struct RadePackage {
    pub dependencies: String,
    pub language: String,
    pub repository: String,
    pub capacity: i64,
    pub version: String,
    pub download: bool,
}
