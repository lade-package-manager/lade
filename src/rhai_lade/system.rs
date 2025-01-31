use std::process;

use super::structs::RhaiErr;



pub(super) fn system(cmd: &str) -> RhaiErr{
    let status = process::Command::new(cmd).status().unwrap_or_else(|e| {
        RhaiErr("Failed to execute cmd")
    });
}