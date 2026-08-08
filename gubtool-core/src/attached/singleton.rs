use crate::{
    attached::{AddressSize, GameProcess},
    game_version::{Game, GameVersion, Version},
};
use std::path::PathBuf;

static mut ATTACHED_PROCESS: Option<GameProcess> = None;

#[expect(static_mut_refs)]
#[inline(always)]
fn attached_process() -> &'static Option<GameProcess> {
    unsafe { &ATTACHED_PROCESS }
}

pub fn pid() -> Option<crate::sys::Pid> {
    attached_process().as_ref().map(|process| process.pid)
}

pub fn game_version() -> Option<GameVersion> {
    attached_process()
        .as_ref()
        .map(|process| process.game_version)
}

pub fn game() -> Option<Game> {
    attached_process()
        .as_ref()
        .map(|process| process.game_version.game())
}

pub fn comm() -> Option<&'static str> {
    attached_process()
        .as_ref()
        .map(|process| process.comm.as_str())
}

pub fn path() -> Option<&'static PathBuf> {
    attached_process().as_ref().map(|process| &process.exe_path)
}

pub fn module_base() -> u64 {
    attached_process()
        .as_ref()
        .map(|process| process.module_base)
        .unwrap_or(0x0)
}

pub fn address_size() -> Option<AddressSize> {
    attached_process()
        .as_ref()
        .map(|process| process.address_size)
}

pub fn version<T: Version>() -> Option<T> {
    attached_process()
        .as_ref()
        .and_then(|process| T::from_game_version(&process.game_version))
}

pub fn is_32() -> bool {
    attached_process()
        .as_ref()
        .map(|process| process.address_size == AddressSize::Bits32)
        .unwrap_or(false)
}

#[cfg(windows)]
pub(crate) fn handle() -> Option<windows::Win32::Foundation::HANDLE> {
    attached_process().as_ref().map(|process| process.handle)
}

pub fn uptime() -> f64 {
    attached_process()
        .as_ref()
        .map(|process| process.uptime())
        .unwrap_or(0.0)
}

pub(crate) fn attach_to_process(process: GameProcess) {
    unsafe { ATTACHED_PROCESS = Some(process) }
}

pub fn detach() {
    unsafe { ATTACHED_PROCESS = None }
}

pub fn is_attached() -> bool {
    attached_process().as_ref().is_some()
}

pub fn process_exists() -> Option<bool> {
    attached_process().as_ref().map(|process| process.exists())
}
