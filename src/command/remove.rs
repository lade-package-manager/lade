use std::{error::Error, fs, io::ErrorKind};

use colored::Colorize;

use crate::{
    crash,
    dependencies::solve,
    err, error, info, package,
    paths::{lade_bin_path, lade_licenses_path},
    search_package::search_package_lade,
};

pub fn remove(package: &str) -> Result<(), Box<dyn Error>> {
    let pkg = package::find(package);

    if pkg.is_none() {
        err!(format!("Package not found: {}", package));
        crash!();
    }

    if let Some(pkg) = pkg {
        if confirm_removal(&pkg.name, &pkg.version.to_string(), &pkg.description) {
            let path = lade_bin_path().join(pkg.bin_name());
            fs::remove_file(path)?;
            package::remove_installed_by_name(package);
            fs::remove_dir_all(lade_licenses_path().join(&pkg.name)).unwrap_or_else(|e| {
                match e.kind() {
                    ErrorKind::NotFound => {}
                    _ => {
                        error!(format!("Failed to remove license directory: {}", e))
                    }
                }
            });

            info!("The package deletion was successfully completed without incident!");

            let wa = search_package_lade(&pkg.name);
            if let Some(p) = wa {
                let n = solve(&p.dependencies);
                if !n.is_empty() {
                    info!("{} packages were installed for {} but are no longer needed. Use `lade autoclean` to remove them", n.len(), p.name);
                }
            }
        }
    }
    Ok(())
}

fn confirm_removal(package_name: &str, version: &str, description: &str) -> bool {
    println!("{} {}", ">>>".green(), "Package details:".bold());
    println!("  - Name: {}", package_name);
    println!("  - Version: {}", version);
    println!("  - Description: {}", description);
    println!();
    println!(
        "{}{}",
        ">>> ".green(),
        "Are you sure you want to remove this package?".bold()
    );

    let mut line = rustyline::Editor::<(), rustyline::history::DefaultHistory>::new().unwrap();
    let input = line
        .readline_with_initial("[y/N] ", ("N", ""))
        .unwrap()
        .trim()
        .to_lowercase();

    matches!(input.as_str(), "y" | "yes")
}
