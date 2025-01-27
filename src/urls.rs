use colored::Colorize;

pub fn urls(package: &str) -> (String, String) {
    let (url, download_filename) = if cfg!(target_os = "windows") {
        (
            format!(
                "https://github.com/rade-package-manager/rade-download-lists/releases/download/{}/{}-x86_64-pc-windows-gnu.radepkg",
                package, package
            ),
            format!("{}-x86_64-pc-windows-gnu.radepkg", package)
        )
    } else if cfg!(target_os = "macos") {
        (
            format!(
                "https://github.com/rade-package-manager/rade-download-lists/releases/download/{}/{}-aarch64-apple-darwin.radepkg",
                package, package
            ),
            format!("{}-aarch64-apple-darwin.radepkg", package)
        )
    } else if cfg!(target_os = "linux") {
        (
            format!(
                "https://github.com/rade-package-manager/rade-download-lists/releases/download/{}/{}-x86_64-unknown-linux-gnu.radepkg",
                package, package
            ),
            format!("{}-x86_64-unknown-linux-gnu.radepkg", package)
        )
    } else {
        panic!(
            "Unsupported operating system: {}",
            std::env::consts::OS.red()
        );
    };

    (url, download_filename)
}
