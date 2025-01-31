use std::process;

pub(super) fn system(cmd: &str) {
    let status = process::Command::new(cmd).status().unwrap_or_else(|e| {
        eprintln!("Failed to execute {}: {}", cmd, e);
        std::process::exit(1);
    });

    if !status.success() {
        eprintln!("Failed to execute {}", cmd);
    }
}
