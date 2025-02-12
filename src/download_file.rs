use crate::package::DownloadUrls;
use crate::paths::lade_downloaded_package_path;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

pub fn download_package(url: &DownloadUrls) -> anyhow::Result<PathBuf> {
    #[allow(unused_assignments)]
    let mut dl_url = String::new();

    if cfg!(target_os = "linux") {
        dl_url = url.linux.clone();
    } else if cfg!(target_os = "macos") {
        dl_url = url.macos.clone();
    } else if cfg!(target_os = "windows") {
        dl_url = url.windows.clone();
    } else {
        panic!("Not Support os: {}", std::env::consts::OS);
    }

    let download_filename = dl_url.split('/').last().unwrap();

    println!(
        "{} {} {}",
        ">>>".green().bold(),
        "Downloading".bold(),
        download_filename.bold()
    );

    // Create a new file to store the downloaded file
    let dest_path = lade_downloaded_package_path().join(download_filename);

    let client = reqwest::blocking::Client::new();

    // Get the total size of the download
    let response = client.head(&dl_url).send()?;
    let total_size = response
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|len| len.to_str().ok())
        .and_then(|len| len.parse().ok())
        .unwrap_or_default();

    // Create a new progress bar
    let progress_bar = ProgressBar::new(total_size);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:40}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"),
    );

    // Download the file and update the progress bar
    let mut downloaded: u64 = 0;
    let mut buffer = vec![0; 8192];
    let mut response = client.get(&dl_url).send()?;
    let mut file = fs::File::create(&dest_path)?;

    // Read the response body and write it to the file
    while let Ok(n) = response.read(&mut buffer) {
        if n == 0 {
            break;
        }
        file.write_all(&buffer[..n])?;
        downloaded += n as u64;

        // Update the progress bar
        progress_bar.set_position(downloaded);
    }

    file.flush()?;
    progress_bar.finish();
    Ok(dest_path)
}

pub fn download_file(url: &str, file_name: &str) -> anyhow::Result<PathBuf> {
    // Create a new file to store the downloaded file
    let dest_path = lade_downloaded_package_path().join(file_name);
    if !dest_path.exists(){
	fs::create_dir_all(lade_downloaded_package_path())?;
    }

    let client = reqwest::blocking::Client::new();

    // Get the total size of the download
    let response = client.head(url).send()?;
    let total_size = response
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|len| len.to_str().ok())
        .and_then(|len| len.parse().ok())
        .unwrap_or_default();

    // Create a new progress bar
    let progress_bar = ProgressBar::new(total_size);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:40}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"),
    );

    // Download the file and update the progress bar
    let mut downloaded: u64 = 0;
    let mut buffer = vec![0; 8192];
    let mut response = client.get(url).send()?;
    let mut file = fs::File::create(&dest_path)?;

    // Read the response body and write it to the file
    while let Ok(n) = response.read(&mut buffer) {
        if n == 0 {
            break;
        }
        file.write_all(&buffer[..n])?;
        downloaded += n as u64;

        // Update the progress bar
        progress_bar.set_position(downloaded);
    }

    file.flush()?;
    progress_bar.finish();
    Ok(dest_path)
}
