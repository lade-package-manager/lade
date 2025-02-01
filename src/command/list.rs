use colored::Colorize;
use std::fs;
use std::io::Read;

use crate::{package::Package, paths};

pub fn list() {
    let lade_packagelist = [
        paths::lade_package_list_main_path(),
        /* paths::lade_package_list_extra_path(), */
    ];

    for path in lade_packagelist {
        let file = fs::File::open(&path).unwrap();

        let mut archive = zip::ZipArchive::new(std::io::BufReader::new(file)).unwrap();

        for item in 0..archive.len() {
            let mut item = archive.by_index(item).unwrap();

            if item.is_dir() || item.name() == "out" {
                continue;
            }

            if let Some(name) = item.enclosed_name() {
                if name.file_name().map(|name| name.to_string_lossy()) != Some("info.toml".into()) {
                    continue;
                }

                let mut buf = String::new();
                item.read_to_string(&mut buf).unwrap();
                let parsed: Package = toml::from_str(&buf).unwrap();

                println!(
                    "{} ({}{})",
                    parsed.name,
                    "v".bright_yellow(),
                    parsed.version.to_string().bright_yellow()
                );
            }
        }
    }
}
