use crate::{
    dependencies,
    download_file::download_package,
    info, install_from_git,
    package::{self, DownloadUrls, Package},
    search_package::search_package_lade,
    unzip_file,
};
use colored::*;
use std::path::PathBuf;

pub fn install(packages: &mut [String]) -> Result<(), Box<dyn std::error::Error>> {
    info!("Resolving dependencies...");
    let resolved_dependencies = resolve_dependencies(packages)?;

    // 依存関係をリスト表示
    packages.iter().for_each(|f| {
        if package::already_installed(f) {
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

            let pkg = search_package_lade(package);

            if let Some(pkg_lade) = pkg {
                print!(
                    "{} ({}{}) ",
                    pkg_lade.name,
                    "v".bright_yellow(),
                    pkg_lade.version.to_string().bright_yellow()
                );
            }
        });

    println!();

    println!("Do you want to proceed with installation?");
    let mut line = rustyline::Editor::<(), rustyline::history::DefaultHistory>::new()?;
    let user_input = line
        .readline_with_initial("[y/n] ", ("y", ""))?
        .trim()
        .to_lowercase();

    if matches!(user_input.as_str(), "y" | "yes") {
        for pkg in resolved_dependencies.iter().rev() {
            if let Some(existing_pkg) = package::find(pkg) {
                package::remove_installed_by_name(&existing_pkg.name);
            }
            // install package
            install_package(pkg)?;
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

    if let Some(pkg_lade) = search_package_lade(package) {
        dependencies.extend(install_from_lade(pkg_lade)?);
    } else {
        return Err(format!("Package not found: {}", package).into());
    }

    Ok(dependencies)
}

// パッケージインストール
fn install_package(package: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(pkg_lade) = search_package_lade(package) {
        info!(
            "Installing \"{}\" ({}{}{}",
            pkg_lade.name,
            "v".bright_yellow(),
            pkg_lade.version.to_string().bright_yellow(),
            ")".bold()
        );
        if let Some(download_url) = &pkg_lade.download_url {
            install_from_url(download_url, package, &pkg_lade.repository)?;
        } else {
            install_from_git::install_from_git(&pkg_lade.name, &pkg_lade.repository)?;
        }

        package::add_installed(pkg_lade);
    } else {
        return Err(format!("Package not found during installation: {}", package).into());
    }

    Ok(())
}

fn install_from_lade(pkg_lade: Package) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let dependencies = pkg_lade
        .dependencies
        .into_iter()
        .filter(|deps| search_package_lade(deps).is_some())
        .collect::<Vec<_>>();

    let dependencies = dependencies::solve(&dependencies);
    Ok(dependencies)
}

fn install_from_url(
    url: &DownloadUrls,
    package: &str,
    repo: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let file = download_package(url)?;
    unzip_file::unzip_and_install_lade(&file, repo, package);
    Ok(file)
}
