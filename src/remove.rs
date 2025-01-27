use std::{error::Error, fs};

use crate::{crash, err, info, installed_structs::Installed, paths::lade_bin_path};

pub fn remove(package: &str) -> Result<(), Box<dyn Error>> {
    let pkg = Installed::search_package(package);

    if pkg.is_none() {
        err!(format!("Package not found: {}", package));
        crash!();
    }

    if let Some(pkg) = pkg {
        let path = lade_bin_path().join(pkg.clone().exec_name);
        fs::remove_file(path)?;
        let mut installed = Installed::new();
        installed.remove_package(pkg);
        info!("The package deletion was successfully completed without incident!");
    }

    Ok(())
}
