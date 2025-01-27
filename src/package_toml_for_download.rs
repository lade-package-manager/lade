use serde::{Deserialize, Serialize};

// ladeのdownloadインストール方法時に読み込むpackage.tomlファイルをパースするためのstruct
#[derive(Serialize, Deserialize)]
pub struct PackageTomlForDownload {
    pub name: String,
}
