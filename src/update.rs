/// Updates the rade and lade package lists by performing the following steps:
///
/// 1. Prints a message indicating the start of the update process.
/// 2. Checks if the rade package list path exists:
///    - If it exists, removes the directory and handles any errors that occur.
///    - Clones the rade package list repository from GitHub to the specified path.
/// 3. Checks if the lade package list path directory exists:
///    - If it exists, removes the directory and handles any errors that occur.
///    - Prints a message indicating the update of the lade package list.
///    - Clones the lade package list repository from GitHub to the specified path.
/// 4. Prints a message indicating the completion of the update process.
use colored::Colorize;

use crate::{
    err, info,
    paths::{lade_package_list_path_dir, rade_package_list_path},
};

pub fn update() {
    info!("Updating package lists");

    if rade_package_list_path().exists() {
        std::fs::remove_dir_all(rade_package_list_path()).unwrap_or_else(|e| {
            err!("Failed to remove package list directory: ", e);
            std::process::exit(1);
        });
    }

    git2::Repository::clone(
        "https://github.com/rade-package-manager/rade-package-list",
        rade_package_list_path(),
    )
    .unwrap_or_else(|e| {
        err!("Failed to update package list", e);
        std::process::exit(1);
    });

    if lade_package_list_path_dir().exists() {
        std::fs::remove_dir_all(lade_package_list_path_dir()).unwrap_or_else(|e| {
            err!("Failed to retrieve package list.\nPlease run `lade update` to retrive package list. ", e);
            std::process::exit(1);
        });
    }

    info!("Updating lade package list");
    git2::Repository::clone(
        "https://github.com/kaedehito/lade-package-list/",
        lade_package_list_path_dir(),
    )
    .unwrap_or_else(|e| {
        eprintln!(
            "{} {}: {}",
            ">>>".red().bold(),
            "Failed to update package list".bold(),
            e
        );
        std::process::exit(1);
    });

    println!("{} {}", ">>>".green().bold(), "Update complete!".bold());
}
