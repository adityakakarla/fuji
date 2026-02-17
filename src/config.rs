use anyhow::Result;
use directories::ProjectDirs;
use figment::{
    Figment,
    providers::{Format, Toml},
};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct AppConfig {
    grok_api_key: String,
    kalshi_api_key_path: String,
    kalshi_key_id: String,
}

fn get_config_path() -> Result<PathBuf> {
    let project_directory =
        ProjectDirs::from("com", "fuji", "fuji").expect("Failed to get project directory");

    let path = project_directory.config_dir().join("config.toml");
    let parent = path.parent().expect("Failed to get parent directory");
    fs::create_dir_all(parent)?;
    Ok(path)
}

fn load_config() -> Result<AppConfig> {
    let path = get_config_path()?;
    Figment::new()
        .merge(Toml::file(path))
        .extract::<AppConfig>()
        .map_err(anyhow::Error::from)
}

pub fn get_kalshi_api_key() -> Result<String> {
    let config = load_config()?;
    let path = config.kalshi_api_key_path;
    let api_key = fs::read_to_string(path)?;
    Ok(api_key)
}

pub fn set_kalshi_api_key_path(api_key_path: &String) -> Result<()> {
    let mut config = load_config().unwrap_or(AppConfig {
        grok_api_key: String::new(),
        kalshi_api_key_path: String::new(),
        kalshi_key_id: String::new(),
    });
    config.kalshi_api_key_path = api_key_path.clone();
    let path = get_config_path()?;
    fs::write(path, toml::to_string(&config)?).map_err(anyhow::Error::from)
}

pub fn get_kalshi_key_id() -> Result<String> {
    let config = load_config()?;
    Ok(config.kalshi_key_id)
}

pub fn set_kalshi_key_id(key_id: &String) -> Result<()> {
    let mut config = load_config().unwrap_or(AppConfig {
        grok_api_key: String::new(),
        kalshi_api_key_path: String::new(),
        kalshi_key_id: String::new(),
    });
    config.kalshi_key_id = key_id.clone();
    let path = get_config_path()?;
    fs::write(path, toml::to_string(&config)?).map_err(anyhow::Error::from)
}

pub fn get_grok_api_key() -> Result<String> {
    let config = load_config()?;
    Ok(config.grok_api_key)
}

pub fn set_grok_api_key(api_key: &String) -> Result<()> {
    let mut config = load_config().unwrap_or(AppConfig {
        grok_api_key: String::new(),
        kalshi_api_key_path: String::new(),
        kalshi_key_id: String::new(),
    });
    config.grok_api_key = api_key.clone();
    let path = get_config_path()?;
    fs::write(path, toml::to_string(&config)?).map_err(anyhow::Error::from)
}

pub fn view_config() -> Result<()> {
    let config = load_config().unwrap_or(AppConfig {
        grok_api_key: String::new(),
        kalshi_api_key_path: String::new(),
        kalshi_key_id: String::new(),
    });
    println!("Config: {:?}", config);
    Ok(())
}
