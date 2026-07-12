use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub players: Vec<String>,
    pub images: bool,
    pub dynamic: Vec<String>,
    pub special_commands: SpecialCommands,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct SpecialCommands {
    #[serde(default)]
    pub play_pause: HashMap<String, String>,
    #[serde(default)]
    pub next: HashMap<String, String>,
    #[serde(default)]
    pub previous: HashMap<String, String>,
    #[serde(default)]
    pub volume: HashMap<String, String>,
}

pub fn load_config(path: &Path) -> Result<Config> {
    if path.exists() {
        let contents = fs::read_to_string(path).context("Could not read config file")?;
        serde_json::from_str(&contents).context("Could not parse config file")
    } else {
        fs::create_dir_all(path.parent().unwrap())
            .context("Could not create config parent directory")?;
        let config = Config::default();
        let json = serde_json::to_string_pretty(&config)?;
        fs::write(path, json).context("Could not write to config.json")?;
        Ok(config)
    }
}
