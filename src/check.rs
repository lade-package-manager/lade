use crate::{info, installed_structs::Installed, search_package::{self}};

pub fn check() {
    let installed = Installed::new();

        let pkg = search_package::search_package(&pkg);
        println!("{:?}", pkg);


        if let Some(packagejson) = pkg.lade {
            for installed_pkg in &installed.packages {
                if installed_pkg.name == packagejson.name && installed_pkg.version != packagejson.version {
                    info!(format!("Updates are available for package {} ({} -> {})", packagejson.name, installed_pkg.version, packagejson.version));
                    continue;
                }
            }
        } else if let Some(radepackage) = pkg.rade {
            for installed_pkg in &installed.packages {
                if installed_pkg.version != radepackage.version {
                    info!(format!("Updates are available for package {} ({} -> {})", installed_pkg.name, installed_pkg.version, radepackage.version));
                    continue;
                }
            }
    }
}
