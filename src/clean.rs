use std::fs;

use crate::{
    crash, err, error,
    macros::UnwrapOrCrash,
    paths::{lade_build_path, lade_cache_path, lade_log_path},
};

pub fn clean() {
    println!("This operation will clean the log file, build directory, and cache directory. Do you want to accept?");

    let mut line = rustyline::Editor::<(), rustyline::history::DefaultHistory>::new().unwrap();
    let input = line
        .readline_with_initial("[y/N] ", ("N", ""))
        .unwrap()
        .trim()
        .to_lowercase();

    if matches!(input.as_str(), "y" | "yes") {
        let cache = lade_cache_path();
        let build = lade_build_path();
        let log = lade_log_path();
        let removes = [cache, build, log];

        for dirs in removes {
            if dirs.is_dir() {
                if dirs.exists() {
                    fs::remove_dir_all(&dirs).unwrap_or_else(|e| {
                        error!(
                            format!("Failed to remove {}: {}", dirs.display(), e),
                            format!("Failed to remove {}: {}", dirs.display(), e)
                        );
                    });
                }

                fs::create_dir_all(&dirs).unwrap_or_else(|e| {
                    error!(
                        format!("Failed to create {}: {}", dirs.display(), e),
                        format!("Failed to create {}: {}", dirs.display(), e)
                    );
                });
            } else {
                if dirs.exists() {
                    fs::remove_file(&dirs).unwrap_or_else(|e| {
                        error!(
                            format!("Failed to remove {}: {}", dirs.display(), e),
                            format!("Failed to remove {}: {}", dirs.display(), e)
                        );
                    });
                }
                fs::File::create(&dirs).unwrap_or_crash_by_status(404, |e| {
                    err!(format!(
                        "Failed to create log file! Please create {}! {}",
                        dirs.display(),
                        e
                    ));
                });
            }
        }
    } else {
        println!("clean is canceled");
    }
}
