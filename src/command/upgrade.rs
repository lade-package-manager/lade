use std::{fs, io, path::Path};

use reqwest::blocking::get;
use serde::{Deserialize, Serialize};

use crate::{
    consts::{LADE_VERSION, VERSION},
    error, info,
    paths::lade_cache_path,
    upgrade_self,
};

#[derive(Serialize, Deserialize)]
struct Upgrade {
    pub version: u64,
    pub lade_version: String,
    pub info: String,
    pub upgrade_url: String,
}

pub fn upgrade() {
    super::update();

    info!("Downloading upgrade info file");
    download_file(
        "https://github.com/lade-package-manager/lade/releases/download/upgrade_info/upgrade_info.json",
        lade_cache_path().join("upgrade_info.json"),
    )
    .unwrap_or_else(|e| {
        error!(
            "Failed to download upgrade info file!",
            format!("Failed to download upgrade info file: {}", e)
        );
    });

    let content =
        fs::read_to_string(lade_cache_path().join("upgrade_info.json")).unwrap_or_else(|e| {
            error!(
                "Failed to read file",
                format!(
                    "Failed to read {}: {}",
                    lade_cache_path().join("upgrade_info.json").display(),
                    e
                )
            );
        });

    let info: Upgrade = serde_json::from_str(&content).unwrap_or_else(|e| {
        error!(
            "Failed to parse upgrade info json",
            format!("Failed to parse upgrade info json: {}", e)
        );
    });

    if info.version != VERSION {
        upgrade_self::self_upgrade();
    }
    if info.lade_version != LADE_VERSION {
        upgrade_self::self_upgrade();
    }
}

fn download_file<P: AsRef<Path>>(
    url: &str,
    output_path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(url)?;
    if !response.status().is_success() {
        return Err(format!("Failed to download: {}", url).into());
    }

    let mut dest = fs::File::create(output_path)?;
    let content = response.bytes()?;
    io::copy(&mut content.as_ref(), &mut dest)?;

    Ok(())
}
