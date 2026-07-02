use gubtool_core::{
    address::Address,
    attached::{game, version},
    game_version::{DarkSouls2Version::*, Game},
    slice_ops::*,
    sys::{
        error::{ProcResult, ProcessError},
        *,
    },
};
use pelite::Pod;
use std::sync::{LazyLock, Mutex};

pub static MASS_SPAWN_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

#[track_caller]
pub fn read<T: Pod>(address: impl Address) -> ProcResult<T> {
    ensure_ds2()?;
    read_unsafe(address)
}

#[track_caller]
pub fn write<T: Pod>(address: impl Address, value: T) -> ProcResult {
    ensure_ds2()?;
    write_unsafe(address, value)
}

#[track_caller]
pub fn write_bytes(address: impl Address, data: &[u8]) -> ProcResult {
    ensure_ds2()?;
    write_bytes_unsafe(address, data)
}

pub fn install_hook_without_code(code_location: impl Address, hook_location: impl Address, original_instruction_size: u64) -> ProcResult {
    let hookbytes = get_hook_bytes(code_location, hook_location, original_instruction_size)?;
    write_bytes(hook_location, &hookbytes)
}

pub fn install_hook(code: &[u8], code_location: impl Address, hook_location: impl Address, original_instruction_size: u64) -> ProcResult {
    let hookbytes = get_hook_bytes(code_location, hook_location, original_instruction_size)?;
    write_bytes(code_location, &code)?;
    write_bytes(hook_location, &hookbytes)
}

pub fn spawn_thread_join(thread_start_address: impl Address, thread_code: Vec<u8>) -> ProcResult {
    ensure_ds2()?;
    #[cfg(unix)]
    gubtool_core::sys::spawn_thread_join(
        crate::offsets::code_cave::CaveOffset::RunThreadAsm.addr(),
        thread_start_address,
        thread_code,
        crate::offsets::module_offsets::ExternalFunctionPointer::Kernel32CreateThread,
        crate::offsets::module_offsets::ExternalFunctionPointer::Kernel32CloseHandle,
    )?;
    #[cfg(windows)]
    gubtool_core::sys::spawn_thread_join(
        thread_start_address,
        thread_code,
    )?;
    Ok(())
}

pub fn spawn_thread_release(thread_start_address: impl Address, thread_code: Vec<u8>) -> ProcResult {
    ensure_ds2()?;
    #[cfg(unix)]
    gubtool_core::sys::spawn_thread_release(
        crate::offsets::code_cave::CaveOffset::RunThreadAsm.addr(),
        thread_start_address,
        thread_code,
        crate::offsets::module_offsets::ExternalFunctionPointer::Kernel32CreateThread,
        crate::offsets::module_offsets::ExternalFunctionPointer::Kernel32CloseHandle,
    )?;
    #[cfg(windows)]
    gubtool_core::sys::spawn_thread_release(
        thread_start_address,
        thread_code,
    )?;
    Ok(())
}

pub fn read_address(address: impl Address) -> ProcResult<u64> {
    ensure_ds2()?;
    read_address_unsafe(address)
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
pub fn is_bit_set(address: impl Address, mask: u8) -> ProcResult<bool> {
    read::<u8>(address)
        .map(|byte| byte & mask != 0)
}

#[track_caller]
pub fn set_bit(address: impl Address, mask: u8, value: bool) -> ProcResult<()> {
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