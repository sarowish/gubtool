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
use gubtool_core::{appdata::{AppDataError, app_data_dir, log_error}, game_version::Game};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs, path::PathBuf};

pub struct AttachConfigManager {
    pub config: AttachConfig,
    pub entries: AttachEntries,
}

pub struct AttachEntries {
    pub ds2: Ds2Entries,
    pub er: ErEntries,
}

impl AttachConfigManager {
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
            let _ = log_error(&err);
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
    fn get_path() -> Result<PathBuf, AppDataError> {
        let appdata_dir = app_data_dir()?;
        Ok(appdata_dir.join("attach_config.toml"))
    }

    fn read() -> Result<Self, AppDataError> {
        let config_path = Self::get_path()?;
        let contents = fs::read_to_string(config_path)?;
        let preferences: AttachConfig = toml::from_str(&contents)?;
        Ok(preferences)
    }

    fn write(&self) -> Result<(), AppDataError> {
        let path = Self::get_path()?;
        let toml = toml::to_string(self)?;
        fs::write(path, toml)?;
        Ok(())
    }

    fn update<F>(modifier: F) -> Result<(), AppDataError>
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