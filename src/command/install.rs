use crate::{
    dependencies,
    download_file::download_package,
    exe_name::get_exec_name,
    info, install_from_git,
    installed_structs::{Installed, Package},
    package_list_structs::{LadePackage, RadePackage},
    search_package, unzip_file,
};
use colored::*;
use std::path::PathBuf;

pub fn install(packages: &mut [String]) -> Result<(), Box<dyn std::error::Error>> {
    info!("Resolving dependencies...");
    let resolved_dependencies = resolve_dependencies(packages)?;

    // 依存関係をリスト表示
    packages.iter().for_each(|f| {
        if Installed::is_installed(f) {
            info!("Package {} is already installed. Reinstalling...", f);
        }
    });

    resolved_dependencies
        .iter()
        .enumerate()
        .for_each(|(num, package)| {
            if num == 4 {
                println!();
            }

            let pkg = search_package::search(package);

            if let Some(pkg_lade) = pkg.lade {
                print!("{} (v{}) ", pkg_lade.name, pkg_lade.version.bright_yellow());
            }

            if let Some(pkg_rade) = pkg.rade {
                print!("{} ({}) ", package, pkg_rade.version.bright_yellow());
            }
        });

    println!();

    println!("Do you want to proceed with installation? [Y/n]");
    let mut line = rustyline::Editor::<(), rustyline::history::DefaultHistory>::new()?;
    let user_input = line
        .readline_with_initial("[y/n] ", ("y", ""))?
        .trim()
        .to_lowercase();

    if matches!(user_input.as_str(), "y" | "yes") {
        let mut installed = Installed::new();
        for pkg in resolved_dependencies.iter().rev() {
            if let Some(existing_pkg) = Installed::search_package(pkg) {
                installed.remove_package_by_name(&existing_pkg.name);
            }
            // install package
            install_package(&mut installed, pkg)?;
        }

        println!("Installation completed successfully.");
    } else {
        println!("Installation canceled.");
    }

    Ok(())
}

// 依存関係解決
fn resolve_dependencies(packages: &[String]) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut dependencies = Vec::new();

    for package in packages {
        let package_dependencies = resolve_dependencies_and_collect(package)?;
        dependencies.extend(package_dependencies);
    }

    let dependencies = dependencies::solve(&dependencies);

    Ok(dependencies)
}

// 依存関係を収集
fn resolve_dependencies_and_collect(
    package: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut dependencies = Vec::new();

    dependencies.push(package.to_string());

    if let Some(pkg_lade) = search_package::lade(package) {
        dependencies.extend(install_from_lade(pkg_lade)?);
    } else if let Some(pkg_rade) = search_package::rade(package) {
        dependencies.extend(install_from_rade(pkg_rade)?);
    } else {
        return Err(format!("Package not found: {}", package).into());
    }

    Ok(dependencies)
}

// パッケージインストール
fn install_package(
    installed: &mut Installed,
    package: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(pkg_lade) = search_package::lade(package) {
        info!(
            "Installing {} (v{})",
            pkg_lade.name,
            pkg_lade.version.bright_yellow()
        );
        if let Some(download_url) = pkg_lade.download.clone() {
            install_from_url(&download_url, package)?;
        } else {
            install_from_git::install_from_git(&pkg_lade.name, &pkg_lade.repository)?;
        }

        let inst = pkg_lade.download.clone();
        installed.add_package(Package::new(
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
    } else if let Some(pkg_rade) = search_package::rade(package) {
        info!(
            "Installing {} ({})",
            package,
            pkg_rade.version.bright_yellow()
        );

        let mut nv = None;
        #[allow(unused)]
        let mut exec_name = String::new();

        if pkg_rade.download {
            nv = Some(String::from("true"));
            install_from_url(package, package)?;
            exec_name = get_exec_name();
            if exec_name.is_empty() {
                exec_name = package.to_string();
            }
        } else {
            install_from_git::install_from_git(package, &pkg_rade.repository)?;

            exec_name = get_exec_name();
            if exec_name.trim().is_empty() {
                exec_name = package.to_string();
            }
        }

        installed.add_package(Package::new(
            package.to_string(),
            pkg_rade.version,
            String::new(),
            String::new(),
            Vec::new(),
            pkg_rade
                .dependencies
                .split(',')
                .map(str::to_string)
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

fn install_from_lade(pkg_lade: LadePackage) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let dependencies = pkg_lade
        .dependencies
        .into_iter()
        .filter(|deps| search_package::lade(deps).is_some() || search_package::rade(deps).is_some())
        .collect::<Vec<_>>();

    let dependencies = dependencies::solve(&dependencies);
    Ok(dependencies)
}

fn install_from_rade(pkg_rade: RadePackage) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let dependencies = pkg_rade
        .dependencies
        .split(',')
        .map(str::to_string)
        .collect::<Vec<_>>();

    let dependencies = dependencies::solve(&dependencies);
    Ok(dependencies)
}

fn install_from_url(url: &str, package: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let file = download_package(url)?;
    unzip_file::unzip_and_install_lade(&file, url, package);
    Ok(file)
}
