use std::process;

pub(super) fn system(cmd: &str, args: Vec<String>) {
    let status = process::Command::new(cmd).args(args).status().unwrap_or_else(|e| {
        eprintln!("Failed to execute {}: {}", cmd, e);
        std::process::exit(1);
    });

    if !status.success() {
        eprintln!("Failed to execute {}", cmd);
    }
}
