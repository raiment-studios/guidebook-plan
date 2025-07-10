mod app;
mod commands;
mod model;
pub mod util;

pub mod internal {
    pub use super::app::App;
    pub use super::model::*;
    pub use super::util::*;

    pub use anyhow::{anyhow, Result};
    pub use regex::Regex;
    pub use serde::{Deserialize, Serialize};
    pub use std::collections::HashMap;
}

use clap::Parser;
use commands::*;

#[derive(Parser)]
#[command(name = "guidebook-plan")]
#[command(about = "a daily routine planning tool")]
#[command(version = "v0.1.1")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// initialize the data repository
    Init,
    /// clone a remote data repository
    Clone,
    /// open the plan file in the editor
    Open,
    /// push changes to the remote repository
    Push,
    /// show the current plan
    Show,
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::new();
    let cli = Cli::parse();
    let command = cli.command.unwrap_or(Commands::Show);

    // Check the data directory exists before proceeding with any commands
    // that require it.
    match command {
        // These commands do not require the guidebook data dir
        Commands::Init | Commands::Clone => {}
        _ => {
            if !app.guidebook_root_exists() {
                cprintln!(
                    "",
                    r#"
[ERROR: guidebook data directory not found](error)

Run the following command to initialize your guidebook repository:
[guidebook-plan init](command)

This will create a remote and local data directory as needed.
"#
                );
                std::process::exit(1);
            };
        }
    }

    let result = match command {
        Commands::Init => command_init().await,
        Commands::Clone => command_clone(),
        Commands::Open => command_open(),
        Commands::Push => command_push(),
        Commands::Show => command_show(),
    };
    if let Err(e) = result {
        eprintln!("ERROR: {}", e);
        std::process::exit(1);
    }
    Ok(())
}
