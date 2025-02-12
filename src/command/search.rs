use crate::{crash, search_package::search_package_lade};
use colored::Colorize;

pub fn search_package(package: &str) {
    let lade_result = search_package_lade(package);
    if lade_result.is_none() {
        println!(
            "{} {}",
            ">>>".green().bold(),
            "Package found in package list.".bold()
        );
        crash!();
    }

    if let Some(result) = lade_result {
        println!(
            "{} {}",
            ">>>".green().bold(),
            "Package found in lade package list.".bold()
        );

        println!("Name: {}", result.name);
        match result.older_versions {
            Some(older_versions) => {
                println!(
                    "{}",
                    older_versions
                        .into_iter()
                        .fold(result.version.to_string(), |b, v| format!("{b}, {v}"))
                )
            }
            None => println!("Available Version: {}", result.version),
        }

        println!("Repository: {}", result.repository);

        if result.download_url.is_some() {
            println!("Download: true");
        }

        println!("Description: {}", result.description);
        println!("License: {}", result.license);

        if !result.authors.is_empty() {
            print!("Authors: ");
            for author in &result.authors {
                if author == result.authors.last().unwrap() {
                    print!("{}", author);
                } else {
                    print!("{}, ", author);
                }
            }
        }
        println!();

        if !result.dependencies.is_empty() {
            print!("Dependencies: ");
            println!("{}", result.dependencies.join(", "));
        }
        println!();
    }
}
