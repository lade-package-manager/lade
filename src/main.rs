use clap::{Parser, Subcommand};
mod check;
mod clean;
mod consts;
mod dependencies;
mod download_file;
mod exe_name;
mod install;
mod install_from_git;
mod installed_structs;
mod list;
mod macros;
mod package_list_structs;
mod package_toml_for_download;
mod paths;
mod remove;
mod search;
mod search_package;
mod unzip_file;
mod update;
mod upgrade;
mod upgrade_self;
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
    Upgrade,
    Clean,
    Check,
    Autoclean,
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
            list::list();
        }
        Subcommands::Search { query } => {
            search::search_package(&query);
        }
        Subcommands::Upgrade => {
            upgrade::upgrade();
        }
        Subcommands::Clean => {
            clean::clean();
        }
        Subcommands::Check => {
            check::check();
        }
        Subcommands::Autoclean => {
            todo!("Autocleaning packages")
        }
    }
}
