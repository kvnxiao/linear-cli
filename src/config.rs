use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub api_key: Option<String>,
}

fn config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("Could not find config directory")?
        .join("linear-cli");

    fs::create_dir_all(&config_dir)?;
    Ok(config_dir.join("config.toml"))
}

pub fn load_config() -> Result<Config> {
    let path = config_path()?;
    if path.exists() {
        let content = fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    } else {
        Ok(Config::default())
    }
}

pub fn save_config(config: &Config) -> Result<()> {
    let path = config_path()?;
    let content = toml::to_string_pretty(config)?;
    fs::write(path, content)?;
    Ok(())
}

pub fn set_api_key(key: &str) -> Result<()> {
    let mut config = load_config()?;
    config.api_key = Some(key.to_string());
    save_config(&config)?;
    Ok(())
}

pub fn get_api_key() -> Result<String> {
    let config = load_config()?;
    config.api_key.context("API key not set. Run: linear config set-key <YOUR_KEY>")
}

pub fn show_config() -> Result<()> {
    let config = load_config()?;
    let path = config_path()?;

    println!("Config file: {}", path.display());
    println!();

    if let Some(key) = &config.api_key {
        let masked = format!("{}...{}", &key[..8], &key[key.len()-4..]);
        println!("API Key: {}", masked);
    } else {
        println!("API Key: (not set)");
    }

    Ok(())
}
