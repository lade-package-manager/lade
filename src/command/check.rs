use crate::{
    info,
    installed_structs::Installed,
    search_package::{self},
};

pub fn check() {
    let installed = Installed::new();

    for package in installed.packages.clone() {
        let pkg = search_package::search_package(&package.name);

        if let Some(packagejson) = pkg.lade {
            if package.name == packagejson.name && package.version != packagejson.version {
                info!(
                    "Updates are available for package {} ({} -> {})",
                    packagejson.name, package.version, packagejson.version
                );
                continue;
            }
        } else if let Some(radepackage) = pkg.rade {
            if package.version != radepackage.version {
                info!(
                    "Updates are available for package {} ({} -> {})",
                    package.name, package.version, radepackage.version
                );
                continue;
            }
        }
    }
}
