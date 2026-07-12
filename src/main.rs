mod config;
mod control;
mod daemon;
mod metadata;

use anyhow::{Context, Result, bail};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "insop-media")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Run,
    Toggle,
    Next,
    Previous,
    Volume { change: String },
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let home = PathBuf::from(std::env::var("HOME").expect("HOME not set"));
    let config_path = home.join(".config/insop-media/config.json");
    let cache_dir = home.join(".cache/insop-media");

    fs::create_dir_all(&cache_dir).context("Failed to create cache directory")?;
    let config = config::load_config(&config_path)?;

    if config.players.is_empty() {
        bail!("Config not found");
    }
    match args.command {
        Commands::Run => daemon::run_event_loop(&config, &cache_dir)?,
        Commands::Toggle => control::toggle(&config),
        Commands::Next => control::next(&config),
        Commands::Previous => control::previous(&config),
        Commands::Volume { change } => control::volume(&config, &change),
    }
    Ok(())
}
