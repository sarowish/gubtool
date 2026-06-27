pub use crate::resources::print_asm_sizes;

use crate::{game_state::is_loaded, offsets};
use thiserror::Error;

#[derive(Error, std::fmt::Debug)]
#[error("Requires Scholar of the First Sin")]
pub struct ScholarError;

#[derive(Error, std::fmt::Debug)]
#[error("Player not loaded")]
pub struct LoadedError;

pub fn player_loaded_check() -> Result<(), LoadedError> {
    if is_loaded() {
        Ok(())
    } else {
        Err(LoadedError)
    }
}

pub fn scan_and_print_base_offsets() -> anyhow::Result<()> {
    let base_offsets = offsets::module_offsets::scan()?;
    println!("{:#X?}", base_offsets);
    Ok(())
}