use crate::{
    dependencies::solve_dependencies,
    download_file::download_package,
    exe_name::get_exec_name,
    info, install_from_git,
    installed_structs::{self, Installed},
    package_list_structs::{PackageJson, RadePackage},
    search_package::{search_package, search_package_lade, search_package_rade},
    unzip_file,
};
use colored::*;
use std::path::PathBuf;

pub fn install(packages: &mut Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let resolved_dependencies = resolve_dependencies(packages)?;

    packages.iter().for_each(|f| {
        if Installed::is_installed(&f) {
            info!(format!(
                "Package {} is already installed. Reinstalling...",
                f
            ));
        }
    });

    let mut num = 0;
    for package in &resolved_dependencies {
        if num == 4 {
            println!();
        }
        let pkg = search_package(package);

        if let Some(pkg_lade) = pkg.lade {
            print!("{} (v{}) ", pkg_lade.name, pkg_lade.version.bright_yellow());
        }

        if let Some(pkg_rade) = pkg.rade {
            print!("{} ({}) ", package, pkg_rade.version.bright_yellow());
        }
        num += 1;
    }
    println!();

    println!("Do you want to proceed with installation? [Y/n]");
    let mut line = rustyline::Editor::<(), rustyline::history::DefaultHistory>::new()?;
    let user_input = line
        .readline_with_initial("[y/n] ", ("y", ""))?
        .trim()
        .to_lowercase();

    if user_input == "y" || user_input == "yes" {
        let mut installed = Installed::new();
        for pkg in resolved_dependencies {
            let lrpkg = search_package(&pkg);
            match lrpkg.lade {
                Some(_) => {
                    if let Some(pkg) = Installed::search_package(&pkg) {
                        installed.remove_package(pkg);
                    }
                }
                None => {}
            }

            match lrpkg.rade {
                Some(_) => {
                    if let Some(pkg) = Installed::search_package(&pkg) {
                        installed.remove_package(pkg);
                    }
                }
                None => {}
            }

            install_package(&pkg)?;
        }
    } else {
        println!("Installation canceled.");
    }

    Ok(())
}

fn resolve_dependencies(packages: &[String]) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut dependencies = Vec::new();

    for package in packages {
        let package_dependencies = resolve_dependencies_and_collect(package)?;
        dependencies.extend(package_dependencies);
    }

    solve_dependencies(&mut dependencies);

    Ok(dependencies)
}

fn resolve_dependencies_and_collect(
    package: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut dependencies = Vec::new();

    dependencies.push(package.to_string());

    if let Some(pkg_lade) = search_package_lade(package) {
        dependencies.extend(install_from_lade(pkg_lade)?);
    } else if let Some(pkg_rade) = search_package_rade(package) {
        dependencies.extend(install_from_rade(pkg_rade)?);
    } else {
        return Err(format!("Package not found: {}", package).into());
    }

    Ok(dependencies)
}

fn install_package(package: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(pkg_lade) = search_package_lade(package) {
        info!(format!(
            "Installing {} (v{})",
            pkg_lade.name,
            pkg_lade.version.bright_yellow()
        ));
        if let Some(download_url) = pkg_lade.download.clone() {
            install_from_url(&download_url)?;
        } else {
            install_from_git::install_from_git(&pkg_lade.name, &pkg_lade.repository)?;
        }

        let inst = match pkg_lade.download {
            None => None,
            Some(s) => Some(s),
        };
        installed_structs::Installed::new().add_package(installed_structs::Package::new(
            pkg_lade.name,
            pkg_lade.version,
            pkg_lade.description,
            pkg_lade.license,
            pkg_lade.authors,
            pkg_lade.dependencies,
            pkg_lade.repository,
            inst,
            package.to_owned(),
        ));
    } else if let Some(pkg_rade) = search_package_rade(package) {
        info!(format!(
            "Installing {} ({})",
            package,
            pkg_rade.version.bright_yellow()
        ));

        let mut nv = None;

        #[allow(unused)]
        let mut exec_name = String::new();

        if pkg_rade.download {
            nv = Some(String::from("true"));
            install_from_url(package)?;
            exec_name = get_exec_name();
            if exec_name == "" || exec_name == "\n" {
                exec_name = package.to_string();
            }
        } else {
            install_from_git::install_from_git(&package, &pkg_rade.repository)?;

            exec_name = get_exec_name();
            if exec_name == "" || exec_name == "\n" {
                exec_name = package.to_string();
            }
        }

        installed_structs::Installed::new().add_package(installed_structs::Package::new(
            package.to_string(),
            pkg_rade.version,
            String::new(),
            String::new(),
            Vec::new(),
            pkg_rade
                .dependencies
                .split(',')
                .collect::<Vec<&str>>()
                .iter()
                .map(|s| s.to_string())
                .collect(),
            pkg_rade.repository,
            nv,
            exec_name.trim().to_string(),
        ));
    } else {
        return Err(format!("Package not found during installation: {}", package).into());
    }

    Ok(())
}

fn install_from_lade(pkg_lade: PackageJson) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut dependencies = Vec::new();
    for dependency in pkg_lade.dependencies {
        if search_package_lade(&dependency).is_some() || search_package_rade(&dependency).is_some()
        {
            dependencies.push(dependency);
        }
    }

    solve_dependencies(&mut dependencies);
    Ok(dependencies)
}

fn install_from_rade(pkg_rade: RadePackage) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut dependencies = pkg_rade
        .dependencies
        .split(',')
        .map(str::to_string)
        .collect::<Vec<_>>();

    solve_dependencies(&mut dependencies);
    Ok(dependencies)
}

fn install_from_url(url: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let file = download_package(url)?;
    unzip_file::unzip_and_install_lade(&file, url);
    Ok(file)
}
