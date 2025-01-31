use std::env;

pub fn set_env(key: &str, value: &str) {
    env::set_var(key, value);
}

pub fn get_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|e| {
        eprintln!("Failed to get {key}: {e}");
        std::process::exit(1);
    })
}
