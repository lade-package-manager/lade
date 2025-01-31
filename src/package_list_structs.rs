use clap::builder::Str;
use crate::version::Version;
use serde::{Deserialize, Serialize};

use crate::version::Version;

// ladeのパッケージ情報を格納するための構造体json
#[derive(Serialize, Deserialize, Debug)]
pub struct LadePackage {
    pub name: String,
    pub version: Vec<String>,
    pub description: String,
    pub license: String,
    pub authors: Vec<String>,
    pub dependencies: Vec<String>,
    pub repository: String,
    pub download: Option<DownloadUrls>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DownloadUrls{
    pub windows: String,
    pub macos: String,
    pub linux: String,
}

pub trait GetLatest {
    fn get_latest(&self) -> String;
}

impl GetLatest for Vec<String> {
    fn get_latest(&self) -> String {
        self.iter()
            .filter_map(|v| Version::parse(v)) // パースできたものだけを対象にする
            .max() // 最も高いバージョンを取得
            .map(|v| v.to_string()) // バージョンを文字列に戻す
            .unwrap_or_else(|| "0.0.0".to_string()) // 空の場合のデフォルト値
    }
}

// ladeのパースされる本命struct
#[derive(Serialize, Deserialize)]
pub struct Packages {
    pub version: String,
    pub packages: Vec<LadePackage>,
}
