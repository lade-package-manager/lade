use crate::{info, package, search_package::search_package_lade};

pub fn check() {
    for package in package::installed() {
        let pkg = search_package_lade(&package.name);

        if let Some(packagejson) = pkg {
            if package.name == packagejson.name && package.version != packagejson.version {
                info!(
                    "Updates are available for package {} ({} -> {})",
                    packagejson.name, package.version, packagejson.version
                );
            }
        }
    }
}
