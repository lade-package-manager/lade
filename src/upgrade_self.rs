use std::{fs, process};

use crate::{
    error, info,
    paths::{lade_bin_path, lade_build_path},
};

pub fn self_upgrade() {
    info!("Updating lade");

    info!("Cloning lade");
    git2::Repository::clone("https://github.com/lade-package-manager/lade", lade_build_path()).unwrap_or_else(
        |e| {
            error!(
                "Failed to clone lade. see lade.log",
                format!("Failed to clone https://github.com/kaedehito/lade: {}", e)
            );
        },
    );

    info!("Checking building dependencies");

    check_dependencies();

    info!("Building lade");

    let status = process::Command::new("cargo")
        .arg("build")
        .arg("--release")
        .status()
        .unwrap_or_else(|e| {
            error!(
                "Failed to execute cargo",
                format!("Failed to execute cargo: {}", e)
            );
        });

    if !status.success() {
        error!("Building lade failed. Please submit this issue to the lade repository.");
    }

    let lade_executable = lade_build_path().join("release").join("lade");
    let lade_bin = lade_bin_path().join("lade");
    fs::rename(lade_executable, lade_bin).unwrap_or_else(|e| {
        error!("Failed to move lade", format!("Failed to move lade: {}", e));
    });

    info!("lade has been upgraded successfully!");
}

fn check_dependencies() {
    let depend = ["cargo", "rustc", "sh"];
    let depend_linux = ["cargo", "rustc", "sh", "pkg-config"];

    if cfg!(target_os = "linux") {
        for depen in depend_linux {
            let i = process::Command::new("which")
                .arg(depen)
                .output()
                .map(|output| !output.stdout.is_empty())
                .unwrap_or(false);

            if !i {
                error!(format!(
                    "{} is required, please install it and try again",
                    depen
                ));
            }
            info!(depen);
        }
    } else if !cfg!(target_os = "windows") {
        for dep in depend {
            let i = process::Command::new("which")
                .arg(dep)
                .output()
                .map(|output| !output.stdout.is_empty())
                .unwrap_or(false);

            if !i {
                error!(format!(
                    "{} is required, please install it and try again",
                    dep
                ));
            }
            info!(dep);
        }
    } else {
        for dep in depend {
            let i = process::Command::new("where")
                .arg(dep)
                .output()
                .map(|output| !output.stdout.is_empty())
                .unwrap_or(false);

            if !i {
                error!(format!(
                    "{} is required, please install it and try again",
                    dep
                ));
            }
            info!(dep);
        }
    }
}
