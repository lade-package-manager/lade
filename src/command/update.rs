use std::fs;

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
use crate::{
    download_file, err, info, log,
    macros::UnwrapOrCrash,
    paths::{lade_package_list_main_path, lade_package_list_path_dir},
};

pub fn update() {
    info!("Updating package lists");

    if lade_package_list_path_dir().exists() {
        std::fs::remove_dir_all(lade_package_list_path_dir()).unwrap_or_crash(|e| {
            err!("Failed to retrieve package list.\nPlease run `lade update` to retrive package list. ", e);
            log!(format!("Failed to remove directory: {}", e), "Failed to remove directory");
        });
    }

    info!("Updating lade package list");

    info!("Downloading main.zip...");
    let path = download_file::download_file(
        "https://github.com/lade-package-manager/package-list-main/releases/download/main/main.zip",
        "main.zip",
    )
    .unwrap_or_crash(|e| {
        err!(
            "Failed to retrieve package list.\nPlease run `lade update` to retrive package list. ",
            e
        );
    });

    if !lade_package_list_path_dir().exists() {
        fs::create_dir_all(lade_package_list_path_dir()).unwrap_or_crash(|e| {
            err!("Failed to retrieve package list.\nPlease run `lade update` to retrive package list. ");
            log!(format!("Failed to remove {}: {}",path.display(),  e), "Failed to remove directory");
        });
    } else {
        fs::remove_dir_all(lade_package_list_path_dir()).unwrap_or_crash(|e| {
            err!("Failed to retrieve package list.\nPlease run `lade update` to retrive package list. ");
            log!(format!("Failed to remove {}: {}",path.display(),  e), "Failed to remove directory");
        });

        fs::create_dir_all(lade_package_list_path_dir()).unwrap_or_crash(|e| {
            err!("Failed to retrieve package list.\nPlease run `lade update` to retrive package list. ");
            log!(format!("Failed to remove {}: {}",path.display(),  e), "Failed to remove directory");
        });
    }

    fs::rename(&path, lade_package_list_main_path()).unwrap_or_crash(|e| {
        err!(
            "Failed to retrieve package list.\nPlease run `lade update` to retrive package list. "
        );
        log!(
            format!(
                "Failed to move {} to {}: {}",
                path.display(),
                lade_package_list_main_path().display(),
                e
            ),
            "Failed to move directory"
        );
    });

    info!("Update complete!");
}
