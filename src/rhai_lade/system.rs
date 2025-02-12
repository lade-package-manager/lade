use rhai::Array;
use std::process::{exit, Command};

use crate::debug;

pub fn system_rhai(cmd: &str, args: Array) {
    let args: Vec<String> = args
        .into_iter()
        .filter_map(|arg| arg.into_string().ok())
        .collect();

    debug!("system(cmd, args) cmd: {cmd}, args: {args:?}");
    let status = Command::new(cmd).args(&args).status();

    match status {
        Ok(status) if status.success() => {}
        Ok(_) => {
            eprintln!("Command failed: {}", cmd);
            exit(1);
        }
        Err(e) => {
            eprintln!("Failed to execute {}: {}", cmd, e);
            exit(1);
        }
    }
}
