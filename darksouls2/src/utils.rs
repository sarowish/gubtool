pub use crate::resources::print_asm_sizes;
pub use crate::offsets::module_offsets::scan::*;

use crate::is_player_loaded;
use gubtool_core::attached::is_32;
use thiserror::Error;

#[derive(Error, std::fmt::Debug)]
#[error("Requires Scholar of the First Sin")]
pub struct ScholarError;

#[derive(Error, std::fmt::Debug)]
#[error("Player not loaded")]
pub struct LoadedError;

pub fn player_loaded_check() -> anyhow::Result<()> {
    crate::mem::ensure_ds2()?;
    if is_player_loaded() {
        Ok(())
    } else {
        Err(LoadedError)
    }?;
    Ok(())
}

pub fn scholar_check() -> anyhow::Result<()> {
    if is_32() {
        Err(ScholarError)
    } else {
        Ok(())
    }?;
    Ok(())
}