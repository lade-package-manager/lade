pub fn windows() -> bool {
    cfg!(target_os = "windows")
}

pub fn linux() -> bool {
    cfg!(target_os = "linux")
}

pub fn macos() -> bool {
    cfg!(target_os = "macos")
}
