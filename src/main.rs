use clap::{Parser, Subcommand};
use std::fs;
mod command;
mod consts;
mod dependencies;
mod download_file;
mod install_from_git;
mod macros;
mod package;
mod package_toml_for_download;
mod paths;
mod rhai_lade;
mod search_package;
mod unzip_file;
mod upgrade_self;
mod version;

use command::*;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    commands: Subcommands,
}

#[derive(Subcommand)]
enum Subcommands {
    Install {
        package: Vec<String>,
    },
    Remove {
        package: String,
    },
    Update,
    List,
    Search {
        query: String,
    },
    Upgrade,
    Clean,
    Check,
    Autoclean,
    /// execute the rhai
    Rhai {
        file: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args.commands {
        Subcommands::Install { mut package } => {
            install::install(&mut package).unwrap_or_else(|e| {
                err_with_fmt!("Failed to install package: {}", e);
                crash!();
            })
        }
        Subcommands::Remove { package } => remove::remove(&package).unwrap_or_else(|e| {
            error!(format!("Error: {}", e), e);
        }),
        Subcommands::Update => update::update(),
        Subcommands::List => list::list(),
        Subcommands::Search { query } => search::search_package(&query),
        Subcommands::Upgrade => upgrade::upgrade(),
        Subcommands::Clean => clean::clean(),
        Subcommands::Check => check::check(),
        Subcommands::Autoclean => todo!("Autocleaning packages"),

        Subcommands::Rhai { file } => {
            rhai_lade::execute::execute_rhai(&fs::read_to_string(file).unwrap()).unwrap();
        }
    }
}
