use std::{
    thread,
    time::{Duration, Instant},
};

use crate::{
    event::get_event,
    game_state::is_loaded,
    mem::*,
    offsets::{self, cs_dlc_imp},
    resources::ASM,
};
use anyhow::{Result, anyhow, bail, ensure};
use engine::{
    attached::{game, module_base, version},
    game_version::{
        EldenRingVersion::*,
        Game,
    },
};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("DLC not found")]
pub struct DlcError;

#[derive(Error, Debug)]
#[error("Requires version 1.12 or above")]
pub struct VersionError;

pub fn is_dlc_available() -> Result<bool> {
    read::<u64>(cs_dlc_imp::base_ptr())
        .and_then(|addr| read::<u8>(addr + cs_dlc_imp::BYTE_FLAGS + cs_dlc_imp::flags::DLC_CHECK))
        .map(|val| val == 1)
}

pub fn dlc_check() -> Result<()> {
    ensure!(is_dlc_available()?, DlcError);
    Ok(())
}

pub fn is_version_dlc_compat() -> bool {
    if game() == Some(Game::EldenRing) && module_base() != 0 {
        matches!(version(),
            Some(Version2_2_0) |
            Some(Version2_2_3) |
            Some(Version2_3_0) |
            Some(Version2_4_0) |
            Some(Version2_5_0) |
            Some(Version2_6_0) |
            Some(Version2_6_1) |
            Some(Version2_6_2))
    } else {
        true
    }
}

pub fn version_check() -> Result<()> {
    ensure!(is_version_dlc_compat(), VersionError);
    Ok(())
}

pub fn character_loaded_check() -> Result<()> {
    let loaded = is_loaded().map_err(|_| anyhow!("Character not loaded"))?;
    ensure!(loaded, "Character not loaded");
    Ok(())
}

pub(crate) fn wait_for_event(event_id: u32, state: bool, timeout_secs: u64) -> Result<()> {
    let start = Instant::now();
    let timeout = Duration::from_secs(timeout_secs);
    while get_event(event_id)? != state {
        if start.elapsed() > timeout {
            bail!("Event flag {} was not set to {} within {:#?}", event_id, state, timeout)
        }
        thread::sleep(Duration::from_millis(50));
    }
    Ok(())
}

pub(crate) fn wait_for_cutscence_completion() -> Result<()> {
    wait_for_event(2200, true, 5)?;
    wait_for_event(2200, false, 60)
}


pub fn scan_and_print_base_offsets() -> Result<()> {
    let base_offsets = offsets::module_offsets::scan()?;
    println!("{:#X?}", base_offsets);
    Ok(())
}

pub fn read_base_pointers() -> Result<()> {
    println!("world_chr_man: {:#X}", read::<u64>(offsets::world_chr_man::base_ptr())?);
    println!("field_area: {:#X}", read::<u64>(offsets::field_area::base_ptr())?);
    println!("game_man: {:#X}", read::<u64>(offsets::game_man::base_ptr())?);
    println!("game_data_man: {:#X}", read::<u64>(offsets::game_data_man::base_ptr())?);
    println!("cs_emk_system: {:#X}", read::<u64>(offsets::cs_emk_system::base_ptr())?);
    println!("virtual_memory_flag: {:#X}", read::<u64>(offsets::virtual_memory_flag::base_ptr())?);
    println!("damage_manager: {:#X}", read::<u64>(offsets::damage_manager::base_ptr())?);
    println!("map_item_impl: {:#X}", read::<u64>(offsets::map_item_impl::base_ptr())?);
    println!("user_input_manager: {:#X}", read::<u64>(offsets::dl_user_input_manager_impl::base_ptr())?);
    println!("cs_flipper_imp: {:#X}", read::<u64>(offsets::cs_flipper_imp::base_ptr())?);
    println!("cs_dlc_imp: {:#X}\n", read::<u64>(offsets::cs_dlc_imp::base_ptr())?);
    Ok(())
}

pub fn print_asm_sizes() {
    ASM.print_function_sizes();
}