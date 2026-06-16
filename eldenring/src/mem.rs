use crate::offsets::{code_cave::CaveOffset, external_function_pointers};
use gubtool_core::{
    attached::game,
    game_version::Game,
    sys::{
        error::{ProcResult, ProcessError},
        *,
    },
};
use pelite::Pod;
use std::sync::{LazyLock, Mutex};
use utils::slice_ops::*;

pub static ITEM_SPAWN_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
pub static EXECUTE_EMEVD_COMMAND_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

#[track_caller]
pub fn read<T: Pod>(address: u64) -> ProcResult<T> {
    ensure_eldenring()?;
    read_unsafe(address)
}

#[track_caller]
pub fn write<T: Pod>(address: u64, value: T) -> ProcResult {
    ensure_eldenring()?;
    write_unsafe(address, value)
}

#[track_caller]
pub fn write_bytes(address: u64, data: &[u8]) -> ProcResult {
    ensure_eldenring()?;
    write_bytes_unsafe(address, data)
}

pub fn spawn_thread_join(thread_start_address: u64, thread_code: Vec<u8>) -> ProcResult {
    ensure_eldenring()?;
    gubtool_core::sys::spawn_thread_join(
        CaveOffset::RunThreadAsm.addr(),
        thread_start_address,
        thread_code,
        read::<u64>(external_function_pointers::kernel32_create_thread())?,
        read::<u64>(external_function_pointers::kernel32_close_handle())?,
    )
}

pub fn is_bit_set(address: u64, mask: u8) -> ProcResult<bool> {
    read::<u8>(address)
        .map(|byte| byte & mask != 0)
}

pub fn set_bit(address: u64, mask: u8, value: bool) -> ProcResult {
    let current_byte = read::<u8>(address)?;
    let new_byte = match value {
        true => current_byte | mask,
        false => current_byte & !mask,
    };
    write::<u8>(address, new_byte)
}

pub fn install_hook(code: &[u8], code_location: u64, hook_location: u64, original_instruction_size: usize) -> ProcResult {
    let hookbytes = get_hook_bytes(code_location, hook_location, original_instruction_size)?;
    write_bytes(code_location, &code)?;
    write_bytes(hook_location, &hookbytes)
}

fn ensure_eldenring() -> ProcResult {
    if game() != Some(Game::EldenRing) {
        Err(ProcessError::InvalidGame { expected: Game::EldenRing })
    } else {
        Ok(())
    }
}