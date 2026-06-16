use crate::offsets::{self, code_cave::CaveOffset};
use gubtool_core::{
    attached::{game, version},
    game_version::{DarkSouls2Version::*, Game},
    sys::{error::{ProcResult, ProcessError}, *},
};
use pelite::Pod;
use utils::slice_ops::*;
use std::sync::{LazyLock, Mutex};

pub static ITEM_SPAWN_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
pub static MASS_SPAWN_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
pub static TRAVEL_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

#[track_caller]
pub fn read<T: Pod>(address: u64) -> ProcResult<T> {
    ensure_ds2()?;
    read_unsafe(address)
}

#[track_caller]
pub fn write<T: Pod>(address: u64, value: T) -> ProcResult {
    ensure_ds2()?;
    write_unsafe(address, value)
}

#[track_caller]
pub fn write_bytes(address: u64, data: &[u8]) -> ProcResult {
    ensure_ds2()?;
    write_bytes_unsafe(address, data)
}

pub fn install_hook_without_code(code_location: u64, hook_location: u64, original_instruction_size: usize) -> ProcResult {
    let hookbytes = get_hook_bytes(code_location, hook_location, original_instruction_size)?;
    write_bytes(hook_location, &hookbytes)
}

pub fn install_hook(code: &[u8], code_location: u64, hook_location: u64, original_instruction_size: usize) -> ProcResult {
    let hookbytes = get_hook_bytes(code_location, hook_location, original_instruction_size)?;
    write_bytes(code_location, &code)?;
    write_bytes(hook_location, &hookbytes)
}

pub fn spawn_thread_join(thread_start_address: u64, thread_code: Vec<u8>) -> ProcResult {
    ensure_ds2()?;
    gubtool_core::sys::spawn_thread_join(
        CaveOffset::RunThreadAsm.addr(),
        thread_start_address,
        thread_code,
        read_address(offsets::kernel32_create_thread())?,
        read_address(offsets::kernel32_close_handle())?,
    )
}

pub fn spawn_thread_release(thread_start_address: u64, thread_code: Vec<u8>) -> ProcResult {
    ensure_ds2()?;
    gubtool_core::sys::spawn_thread_release(
        CaveOffset::RunThreadAsm.addr(),
        thread_start_address,
        thread_code,
        read_address(offsets::kernel32_create_thread())?,
        read_address(offsets::kernel32_close_handle())?,
    )
}

pub fn is_scholar() -> bool {
    game() != Some(Game::DarkSouls2 ) || matches!(
        version(),
        Some(Scholar1_0_1) | Some(Scholar1_0_2) | Some(Scholar1_0_3) | Some(ScholarUnknown)
    )
}

pub fn follow_pointers(pointers: &[u64], read_final: bool) -> ProcResult<u64> {
    let mut pointer = 0u64;
    let (last, rest) = pointers.split_last().unwrap();
    if is_scholar() {
        for offset in rest {
            pointer = read::<u64>(pointer + offset)?
        }
        if read_final {
            pointer = read::<u64>(pointer + last)?
        } else {
            pointer = pointer + last
        }
    } else {
        for offset in rest {
            pointer = read::<u32>(pointer + offset)? as u64
        }
        if read_final {
            pointer = read::<u32>(pointer + last)? as u64
        } else {
            pointer = pointer + last
        }
    }
    Ok(pointer)
}

#[track_caller]
pub fn read_address(address: u64) -> ProcResult<u64> {
    if is_scholar() {
        read::<u64>(address)
    } else {
        read::<u32>(address).map(|addr| addr as u64)
    }
}

#[track_caller]
pub fn read_addr_from_slice(array: &[u8], offset: u64) -> ProcResult<u64> {
    Ok(if is_scholar() {
        read_from_slice::<u64>(array, offset)?
    } else {
        read_from_slice::<u32>(array, offset).map(|addr| addr as u64)?
    })
}

#[track_caller]
pub fn is_bit_set(address: u64, mask: u8) -> ProcResult<bool> {
    read::<u8>(address)
        .map(|byte| byte & mask != 0)
}

#[track_caller]
pub fn set_bit(address: u64, mask: u8, value: bool) -> ProcResult<()> {
    let current_byte = read::<u8>(address)?;
    let new_byte = match value {
        true => current_byte | mask,
        false => current_byte & !mask,
    };
    write::<u8>(address, new_byte)
}

fn ensure_ds2() -> ProcResult {
    if game() != Some(Game::DarkSouls2) {
        Err(ProcessError::InvalidGame { expected: Game::DarkSouls2 })
    } else {
        Ok(())
    }
}