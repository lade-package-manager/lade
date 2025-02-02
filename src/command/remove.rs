use std::{error::Error, fs};

use colored::Colorize;

use crate::{
    crash, dependencies, err, info, installed_structs::Installed, paths::lade_bin_path,
    search_package,
};

pub fn remove(package: &str) -> Result<(), Box<dyn Error>> {
    let pkg = Installed::search_package(package);

    if pkg.is_none() {
        err!(format!("Package not found: {}", package));
        crash!();
    }

    if let Some(pkg) = pkg {
        if confirm_removal(&pkg.name, &pkg.version, &pkg.description) {
            let path = lade_bin_path().join(pkg.clone().exec_name);
            fs::remove_file(path)?;
            let mut installed = Installed::new();
            installed.remove_package_by_name(package);

            info!("The package deletion was successfully completed without incident!");

            let wa = search_package::search(&pkg.name);
            if let Some(p) = wa.lade {
                let n = dependencies::solve(&p.dependencies);
                if !n.is_empty() {
                    info!("{} packages were installed for {} but are no longer needed. Use `lade autoclean` to remove them", n.len(), p.name);
                }
            } else if let Some(o) = wa.rade {
                let vector = o
                    .dependencies
                    .split(',')
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<String>>();
                let v = dependencies::solve(&vector);

                if !v.is_empty() {
                    info!("{} packages were installed for {} but are no longer needed. Use `lade autoclean` to remove them", v.len(), pkg.name);
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
        "Are you sure you want to remove this package? [y/N]".bold()
    );

    let mut line = rustyline::Editor::<(), rustyline::history::DefaultHistory>::new().unwrap();
    let input = line
        .readline_with_initial("[y/N] ", ("N", ""))
        .unwrap()
        .trim()
        .to_lowercase();

    matches!(input.as_str(), "y" | "yes")
}
