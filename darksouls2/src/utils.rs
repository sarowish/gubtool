use crate::game_state::is_loaded;
use anyhow::{Result, anyhow, ensure};
use thiserror::Error;

#[derive(Error, std::fmt::Debug)]
#[error("Requires Scholar of the First Sin")]
pub struct ScholarError;

pub fn character_loaded_check() -> Result<()> {
    let loaded = is_loaded().map_err(|_| anyhow!("Character not loaded"))?;
    ensure!(loaded, "Character not loaded");
    Ok(())
}