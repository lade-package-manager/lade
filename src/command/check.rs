use crate::{info, package, search_package::search_package_lade};

pub fn check() {
    let mut check = false;
    for package in package::installed() {
        let pkg = search_package_lade(&package.name);

        if let Some(packagejson) = pkg {
            if package.name == packagejson.name && package.version != packagejson.version {
                check = true;
                info!(
                    "Updates are available for package {} ({} -> {})",
                    packagejson.name, package.version, packagejson.version
                );
            }
        }
    }

    if !check{
        info!("All packages are already up to date");
    }
}
