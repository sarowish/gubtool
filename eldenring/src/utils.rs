use crate::{
    event::get_event,
    game_state,
    mem::*,
    offsets::{self, ChainReadExt, cs_dlc_imp, module_offsets::BasePointer},
    resources::ASM,
};
use anyhow::bail;
use gubtool_core::{
    attached::version,
    game_version::EldenRingVersion::*,
    sys::error::{PointerType::PlayerIns, ProcResult, ProcessError},
};
use std::{
    thread,
    time::{Duration, Instant},
};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("DLC not found")]
pub struct DlcError;

#[derive(Error, Debug)]
#[error("Requires version 1.12 or above")]
pub struct VersionError;

pub fn is_dlc_available() -> bool {
    read::<u64>(BasePointer::CsDlcImp)
        .add_offset(cs_dlc_imp::BYTE_FLAGS)
        .add_offset(cs_dlc_imp::flags::DLC_CHECK)
        .read::<u8>()
        .map(|val| val == 1)
        .unwrap_or_default()
}

pub fn dlc_check() -> Result<(), DlcError> {
    if !is_dlc_available() {
        Err(DlcError)
    } else {
        Ok(())
    }
}

pub fn is_version_dlc_compat() -> bool {
    match version() {
        Some(Version2_2_0) |
        Some(Version2_2_3) |
        Some(Version2_3_0) |
        Some(Version2_4_0) |
        Some(Version2_5_0) |
        Some(Version2_6_0) |
        Some(Version2_6_1) |
        Some(Version2_6_2) |
        None => true,
        _ => false
    }
}

pub fn version_check() -> Result<(), VersionError> {
    if !is_version_dlc_compat() {
        Err(VersionError)
    } else {
        Ok(())
    }
}

pub fn player_loaded_check() -> ProcResult {
    if !game_state::is_loaded() {
        Err(ProcessError::InvalidPointer { pointer_type: PlayerIns })
    } else {
        Ok(())
    }
}

pub(crate) fn wait_for_event(event_id: u32, state: bool, timeout_secs: u64) -> anyhow::Result<()> {
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

pub(crate) fn wait_for_cutscence_completion() -> anyhow::Result<()> {
    wait_for_event(2200, true, 30)?;
    wait_for_event(2200, false, 120)
}


pub fn scan_and_print_base_offsets() -> anyhow::Result<()> {
    let base_offsets = offsets::module_offsets::scan()?;
    println!("{:#X?}", base_offsets);
    Ok(())
}

pub fn print_asm_sizes() {
    println!("Elden Ring");
    ASM.print_function_sizes();
    println!("\n");
}