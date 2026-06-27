pub mod attach_config_error;
pub mod ds2_attach;
pub mod er_attach;

use crate::{
    Config,
    attach::{
        attach_config_error::AttachConfigError,
        ds2_attach::{Ds2AttachConfig, Ds2Entries},
        er_attach::{ErAttachConfig, ErEntries},
    },
};
use anyhow::{Result, anyhow};
use gubtool_core::{error_log::log_error, game_version::Game};
use serde::{Deserialize, Serialize};
use std::{env, fmt::Display, fs, path::PathBuf};

pub struct AttachManager {
    pub config: AttachConfig,
    pub entries: AttachEntries,
}

pub struct AttachEntries {
    pub ds2: Ds2Entries,
    pub er: ErEntries,
}

impl AttachManager {
    pub fn new() -> Self {
        Self {
            config: AttachConfig::read().unwrap_or_default(),
            entries: AttachEntries {
                ds2: Ds2Entries::new(),
                er: ErEntries::new(),
            }
        }
    }

    pub fn update(&mut self) {
        self.config = AttachConfig::read().unwrap_or_default();
    }

    pub fn attach(&mut self, game: Game) -> Result<(), AttachConfigError> {
        let entries = match game {
            Game::DarkSouls2 => self.entries.ds2.get_iter(),
            Game::EldenRing => self.entries.er.get_iter(),
        };
        let mut errors = Vec::new();
        for entry in entries {
            if let Err(err) = entry.apply(&mut self.config) {
                errors.push(err);
            }
        }
        let len = errors.len();
        for err in errors {
            log_error(&err)
        }
        if len > 0 {
            return Err(AttachConfigError { error_count: len });
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AttachConfig {
    #[serde(rename = "dark_souls_2")]
    pub dark_souls_2: Ds2AttachConfig,

    #[serde(rename = "elden_ring")]
    pub elden_ring: ErAttachConfig,
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

pub trait BoolEntryTrait: 'static + Send + Sync + Display {
    fn get<'a>(&self, conf: &'a AttachConfig) -> &'a bool;
    fn get_mut<'a>(&self, conf: &'a mut AttachConfig) -> &'a mut bool;
    fn apply(&self, conf: &mut AttachConfig) -> anyhow::Result<()>;

    fn toggle(&self, conf: &mut AttachConfig) {
        let val = self.get_mut(conf);
        *val = !*val
    }
    fn to_attach_entry(self) -> AttachEntry
    where
        Self: Sized + 'static,
    {
        AttachEntry::Bool(Box::new(self))
    }
}

pub trait FloatEntryTrait: 'static + Send + Sync + Display {
    fn get<'a>(&self, conf: &'a AttachConfig) -> &'a Option<f32>;
    fn get_mut<'a>(&self, conf: &'a mut AttachConfig) -> &'a mut Option<f32>;
    fn apply(&self, conf: &mut AttachConfig) -> anyhow::Result<()>;

    fn set(&self, conf: &mut AttachConfig, val: Option<f32>) {
        let old = self.get_mut(conf);
        *old = val
    }
    fn to_attach_entry(self) -> AttachEntry
    where
        Self: Sized + 'static,
    {
        AttachEntry::Float(Box::new(self))
    }
}

pub enum AttachEntry {
    Bool(Box<dyn BoolEntryTrait>),
    Float(Box<dyn FloatEntryTrait>),
}

impl AttachEntry {
    fn apply(&self, conf: &mut AttachConfig) -> anyhow::Result<()> {
        match self {
            Self::Bool(entry) => entry.apply(conf),
            Self::Float(entry) => entry.apply(conf),
        }
    }
}

impl Display for AttachEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(val) => write!(f, "{val}"),
            Self::Float(val) => write!(f, "{val}"),
        }
    }
}