use std::{fs, ops::ControlFlow, process::Stdio};

use crate::{
    crash, err, info,
    macros::UnwrapOrCrash,
    paths::{lade_bin_path, lade_build_path},
    write_log,
};

pub fn install_from_git(package: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    if lade_build_path().exists() {
        std::fs::remove_dir_all(lade_build_path()).unwrap_or_else(|e| {
            err!("Failed to remove build directory: {}", e);
            std::process::exit(1);
        });
    }

    git2::Repository::clone(url, lade_build_path())?;

    let install_sh = lade_build_path().join("install.sh");
    let install_comrade = lade_build_path().join(".comrade").join("build.sh");
    let install_lade = lade_build_path().join(".lade").join("build.sh");
    let install_rade = lade_build_path().join(".build.lade.sh");
    let installs = vec![install_lade, install_comrade, install_rade, install_sh];

    let installed = installs.into_iter().try_for_each(|install| {
        if install.exists() {
            std::process::Command::new("sh")
                .arg(install)
                .current_dir(lade_build_path())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .unwrap_or_crash(|e| {
                    err!("Failed to run install script. please see lade log file", e);
                    write_log!(format!(
                        "date: {}\nerror: Failed to run install script\nError_code: {}",
                        chrono::Local::now(),
                        e
                    ));
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
