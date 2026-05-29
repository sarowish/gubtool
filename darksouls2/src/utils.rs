use crate::{game_state::is_loaded, offsets, resources::scholar};
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

pub fn scan_and_print_base_offsets() -> Result<()> {
    let base_offsets = offsets::module_offsets::scan()?;
    println!("{:#X?}", base_offsets);
    Ok(())
}

pub fn print_asm_sizes_64() {
    scholar::ASM.print_function_sizes()
}