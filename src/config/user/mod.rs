pub mod ds2_attach;
pub mod er_attach;

use crate::config::{
    Config,
    user::{ds2_attach::Ds2Attach, er_attach::ErAttach},
};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AttachConfig {
    #[serde(rename = "dark_souls_2")]
    pub dark_souls_2: Ds2Attach,

    #[serde(rename = "elden_ring")]
    pub elden_ring: ErAttach,
}

impl Config for AttachConfig {
    fn get_path() -> Result<PathBuf> {
        let Some(home_dir) = env::home_dir() else {
            return Err(anyhow!("Home directory not found"));
        };
        Ok(home_dir
            .join(".local")
            .join("state")
            .join("gubtool")
            .join("attach_config.toml"))
    }

    fn read() -> Result<Self> {
        let config_path = Self::get_path()?;
        if !config_path.exists() {
            return Err(anyhow!("Config file not found"));
        }
        let contents = fs::read_to_string(config_path).map_err(|_| {
            anyhow!("Error while reading attach_config.toml. Preferences not initialized.")
        })?;

        let preferences: AttachConfig = toml::from_str(&contents).map_err(|_| {
            anyhow!("Error while parsing attach_config.toml. Preferences not initialized.")
        })?;
        Ok(preferences)
    }

    fn write(&self) -> Result<()> {
        let path = Self::get_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let toml = toml::to_string(self)?;
        fs::write(path, toml)?;
        Ok(())
    }

    fn update<F>(modifier: F) -> Result<()>
    where
        F: FnOnce(&mut AttachConfig),
    {
        let mut toml = Self::read().unwrap_or_default();
        modifier(&mut toml);
        Self::write(&toml)
    }
}