use crate::{info, installed_structs::Installed, search_package::search_package_lade};

pub fn check() {
    let installed = Installed::new();

    for package in installed.packages.clone() {
        let pkg = search_package_lade(&package.name);

        if let Some(packagejson) = pkg {
            if package.name == packagejson.name && package.version != packagejson.version {
                info!(
                    "Updates are available for package {} ({} -> {})",
                    packagejson.name, package.version, packagejson.version
                );
                continue;
            }
        }
    }
}
