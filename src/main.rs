use clap::{Parser, Subcommand};
mod command;
mod consts;
mod dependencies;
mod download_file;
mod exe_name;
mod install_from_git;
mod installed_structs;
mod macros;
mod package_list_structs;
mod package_toml_for_download;
mod paths;
mod search_package;
mod unzip_file;
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
        Subcommands::Install { mut package } => command::install(&mut package).unwrap(),
        Subcommands::Remove { package } => command::remove(&package).unwrap_or_else(|e| {
            error!(format!("Error: {}", e), e);
        }),
        Subcommands::Update => command::update(),
        Subcommands::List => command::list(),
        Subcommands::Search { query } => command::search(&query),
        Subcommands::Upgrade => command::upgrade(),
        Subcommands::Clean => command::clean(),
        Subcommands::Check => command::check(),
        Subcommands::Autoclean => todo!("Autocleaning packages"),
    }
}
