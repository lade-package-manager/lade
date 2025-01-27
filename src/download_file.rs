use crate::{paths::lade_cache_path, urls::urls};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::io::{Read, Write};
use std::{error::Error, path::PathBuf};

pub fn download_package(package: &str) -> Result<PathBuf, Box<dyn Error>> {
    let (url, download_filename) = urls(package);

    println!(
        "{} {} {}",
        ">>>".green().bold(),
        "Downloading".bold(),
        download_filename.bold()
    );

    // Create a new file to store the downloaded file
    let dest_path = lade_cache_path().join(download_filename);

    let client = reqwest::blocking::Client::new();

    // Get the total size of the download
    let response = client.head(&url).send()?;
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
            .template("[{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"),
    );

    // Download the file and update the progress bar
    let mut downloaded: u64 = 0;
    let mut buffer = vec![0; 8192];
    let mut response = client.get(&url).send()?;
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
