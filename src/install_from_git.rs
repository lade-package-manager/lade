use indicatif::{ProgressBar, ProgressStyle};
use std::{env, fs, ops::ControlFlow};

use crate::{
    crash, debug, err, error, info,
    macros::UnwrapOrCrash,
    paths::{lade_bin_path, lade_build_git_path},
    rhai_lade::execute,
    write_log,
};

pub fn install_preparation_git(package: &str, url: &str) -> anyhow::Result<()> {
    debug!(
        "Searching {}...",
        lade_build_git_path().join(package).display()
    );
    if lade_build_git_path().join(package).exists() {
        if let Err(e) = fs::create_dir_all(lade_build_git_path()) {
            error!(format!("Failed to remove build directory: {}", e));
        } else {
            debug!("Removed!");
        }
    }

    let into = lade_build_git_path().join(package);

    let mut callback = git2::RemoteCallbacks::new();

    // Progressbar
    let progress_bar = ProgressBar::new(0);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:40}] {msg}")
            .expect("Invalid progress bar template")
            .progress_chars("#>-"),
    );

    // Progress handling
    callback.transfer_progress({
        let progress_bar = progress_bar.clone();
        move |callback_progress| {
            let received = callback_progress.received_objects();
            let total = callback_progress.total_objects();

            if total > 0 {
                progress_bar.set_length(total as u64);
                progress_bar.set_position(received as u64);
		progress_bar.set_message(format!("{}/{}", received, total));
            }

            true
        }
    });

    // set fetch options
    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.remote_callbacks(callback);

    // set builder
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_options);

    // clone the repository
    let result = builder.clone(url, &into);

    if let Err(e) = result {
        error!(format!("Failed to clone repository: {}", e));
    }

    progress_bar.finish();

    Ok(())
}

pub fn install_from_git(package: &str, url: &str) -> anyhow::Result<()> {
    let path = lade_build_git_path().join(package);

    let install_sh = path.join("install.rhai");
    let install_lade = path.join(".lade").join("build.rhai");
    let install_rade = path.join(".build.lade.rhai");
    let installs = vec![install_lade, install_rade, install_sh];

    let installed = installs.into_iter().try_for_each(|install| {
        if install.exists() {
            let content = fs::read_to_string(install).unwrap_or_else(|e| {
                error!(
                    "Failed to read install script",
                    format!("Failed to read install script: {e}, url: {url}")
                );
            });

            let path = lade_build_git_path().join(package);
            debug!("Changing directory to: {}", path.display());
            env::set_current_dir(&path).unwrap_or_else(|e| {
                error!(
                    "Failed to set current directory!",
                    format!("Failed to set current directory: {}", e)
                );
            });

            debug!("> executeing rhai content={content}");
            execute::execute_rhai(&content).unwrap_or_else(|e| {
                error!(
                    "Failed to execute install script",
                    format!("Failed to execute install script: {e}, url: {url}")
                );
            });
            debug!("> Ok");
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

    let exec = path.join(package);
    debug!("search: {}", exec.display());
    if !exec.exists() {
        err!("Couldn't find the executable file");
        write_log!(format!(
            "date: {}\nerror: Failed to find the executable\n repository: {}",
            chrono::Local::now(),
            package
        ));
        crash!();
    }

    debug!(
        "move executable file: {} -> {}",
        exec.display(),
        lade_bin_path().join(&package).display()
    );
    env::set_current_dir(&path).unwrap_or_else(|e| {
        error!(
            "Failed to set current directory!",
            format!("Failed to set current directory: {}", e)
        );
    });

    fs::rename(exec, lade_bin_path().join(&package)).unwrap_or_crash(|e| {
        err!("Failed to move executable file", e);
        write_log!(format!(
            "date: {}\nerror: Failed to copy executable file\n Error_code: {}",
            chrono::Local::now(),
            e
        ));
    });

    fs::remove_dir_all(&path).unwrap_or_else(|e| {
        error!("Failed to remove directory: {}", e);
    });

    info!("{} is installed!", package);

    Ok(())
}
