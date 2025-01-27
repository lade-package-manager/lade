use clap::{Parser, Subcommand};
mod dependencies;
mod download_file;
mod exe_name;
mod install;
mod install_from_git;
mod installed_structs;
mod macros;
mod package_list_structs;
mod package_toml_for_download;
mod paths;
mod remove;
mod search;
mod search_package;
mod unzip_file;
mod update;
mod urls;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    commands: Subcommands,
}

#[derive(Subcommand)]
enum Subcommands {
    Install { package: Vec<String> },
    Remove { package: String },
    Update,
    List,
    Search { query: String },
    Info { package: String },
    Upgrade,
    Config,
    Clean,
    Check,
}

fn main() {
    let args = Cli::parse();

    match args.commands {
        Subcommands::Install { mut package } => {
            install::install(&mut package).unwrap();
        }
        Subcommands::Remove { package } => {
            remove::remove(&package).unwrap_or_else(|e| {
                error!(format!("Error: {}", e), e);
            });
        }
        Subcommands::Update => {
            update::update();
        }
        Subcommands::List => {
            todo!("Listing installed packages");
        }
        Subcommands::Search { query } => {
            search::search_package(&query);
        }
        Subcommands::Info { package } => {
            todo!("Getting info for package: {}", package);
        }
        Subcommands::Upgrade => {
            todo!("Upgrading all packages");
        }
        Subcommands::Config => {
            todo!("Getting configuration");
        }
        Subcommands::Clean => {
            todo!("Cleaning up cache");
        }
        Subcommands::Check => {
            todo!("Checking for updates");
        }
    }
}
