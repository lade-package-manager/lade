use std::{fs, ops::ControlFlow, process::Stdio};

use crate::{
    crash, err, error, info, macros::UnwrapOrCrash, paths::{lade_bin_path, lade_build_path}, rhai_lade::execute, write_log
};

pub fn install_from_git(package: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    if lade_build_path().exists() {
        std::fs::remove_dir_all(lade_build_path()).unwrap_or_else(|e| {
            err!("Failed to remove build directory: {}", e);
            std::process::exit(1);
        });
    }

    git2::Repository::clone(url, lade_build_path())?;

    let install_sh = lade_build_path().join("install.rhai");
    let install_lade = lade_build_path().join(".lade").join("build.rhai");
    let install_rade = lade_build_path().join(".build.lade.rhai");
    let installs = vec![install_lade, install_rade, install_sh];

    let installed = installs.into_iter().try_for_each(|install| {
        if install.exists() {
            let content = fs::read_to_string(install).unwrap_or_else(|e| {
                error!("Failed to read install script", format!("Failed to read install script: {e}, url: {url}"));
            });
            execute::execute_rhai(&content).unwrap_or_else(|e| {
                error!("Failed to execute install script", format!("Failed to execute install script: {e}, url: {url}"));
            });
            return ControlFlow::Break(());
        }
        ControlFlow::Continue(())
    });

    if let ControlFlow::Continue(_) = installed {
        err!("Failed to find install script");
        write_log!(format!(
            "date: {}\nerror: Failed to find install script\n repository: {}",
            chrono::Local::now(),
            package
        ));
        crash!();
    }

    let exec = lade_build_path().join(package);
    if !exec.exists() {
        err!("Couldn't find the executable file");
        write_log!(format!(
            "date: {}\nerror: Failed to find the executable\n repository: {}",
            chrono::Local::now(),
            package
        ));
        crash!();
    }

    fs::rename(exec, lade_bin_path().join(package)).unwrap_or_crash(|e| {
        err!("Failed to move executable file", e);
        write_log!(format!(
            "date: {}\nerror: Failed to copy executable file\n Error_code: {}",
            chrono::Local::now(),
            e
        ));
    });

    info!("{} is installed now!", package);

    Ok(())
}
