use crate::{
    attached::{AddressSize, GameProcess},
    game_version::{Game, GameVersion, Version},
};
use std::path::PathBuf;

static mut ATTACHED_PROCESS: Option<GameProcess> = None;

pub fn pid() -> Option<crate::sys::Pid> {
    unsafe {
        match &*std::ptr::addr_of!(ATTACHED_PROCESS) {
            Some(process) => Some(process.pid),
            None => None,
        }
    }
}

pub fn game_version() -> Option<GameVersion> {
    unsafe {
        match &*std::ptr::addr_of!(ATTACHED_PROCESS) {
            Some(process) => Some(process.game_version),
            None => None,
        }
    }
}

pub fn game() -> Option<Game> {
    unsafe {
        match &*std::ptr::addr_of!(ATTACHED_PROCESS) {
            Some(process) => Some(process.game_version.game()),
            None => None,
        }
    }
}

pub fn comm() -> &'static str {
    unsafe {
        match &*std::ptr::addr_of!(ATTACHED_PROCESS) {
            Some(process) => &process.comm,
            None => "",
        }
    }
}

pub fn path() -> PathBuf {
    unsafe {
        match &*std::ptr::addr_of!(ATTACHED_PROCESS) {
            Some(process) => process.exe_path.to_owned(),
            None => PathBuf::default(),
        }
    }
}

pub fn module_base() -> u64 {
    unsafe {
        match &*std::ptr::addr_of!(ATTACHED_PROCESS) {
            Some(process) => process.module_base,
            None => 0x0,
        }
    }
}

pub fn address_size() -> Option<AddressSize> {
    unsafe {
        match &*std::ptr::addr_of!(ATTACHED_PROCESS) {
            Some(process) => Some(process.address_size),
            None => None,
        }
    }
}

pub fn version<T: Version>() -> Option<T> {
    unsafe {
        match &*std::ptr::addr_of!(ATTACHED_PROCESS) {
            Some(process) => T::from_game_version(&process.game_version),
            None => None,
        }
    }
}

pub fn is_32() -> bool {
    unsafe {
        match &*std::ptr::addr_of!(ATTACHED_PROCESS) {
            Some(process) => {
                match process.address_size {
                    AddressSize::Bits32 => true,
                    AddressSize::Bits64 => false,
                }
            }
            None => false
        }
    }
}

#[cfg(windows)]
pub(crate) fn handle() -> Option<windows::Win32::Foundation::HANDLE> {
    unsafe {
        match &*std::ptr::addr_of!(ATTACHED_PROCESS) {
            Some(process) => Some(process.handle),
            None => None,
        }
    }
}

pub fn uptime() -> f64 {
    unsafe {
        match &*std::ptr::addr_of!(ATTACHED_PROCESS) {
            Some(process) => process.uptime(),
            None => 0.0,
        }
    }
}

pub fn attach_to_process(process: GameProcess) {
    unsafe { ATTACHED_PROCESS = Some(process) }
}

pub fn detach() {
    unsafe { ATTACHED_PROCESS = None }
}

pub fn is_attached() -> bool {
    unsafe {
        match &*std::ptr::addr_of!(ATTACHED_PROCESS) {
            Some(_) => true,
            None => false,
        }
    }
}

pub fn process_exists() -> Option<bool> {
    unsafe {
        match &*std::ptr::addr_of!(ATTACHED_PROCESS) {
            Some(proc) => Some(proc.exists()),
            None => None,
        }
    }
}