mod error;
mod parse;

use crate::{
    attached::error::ParseError,
    error_log::log_error,
    game_version::{Game, GameVersion, Version},
};
use nix::unistd::Pid;
use std::{
    fs,
    path::{Path, PathBuf},
};
use thiserror::Error;

static mut ATTACHED_PROCESS: Option<GameProcess> = None;

#[derive(Debug)]
pub struct GameProcess {
    pub pid: Pid,
    pub game_version: GameVersion,
    pub comm: &'static str,
    pub exe_path: PathBuf,
    pub module_base: u64,
    pub address_size: AddressSize,
    pub parse_errors: Vec<ParseError>,
}

#[derive(Debug, Clone, Copy)]
pub enum AddressSize { Bits32, Bits64 }

pub fn pid() -> Pid {
    unsafe {
        match &*std::ptr::addr_of!(ATTACHED_PROCESS) {
            Some(process) => process.pid,
            None => Pid::from_raw(-1),
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
            Some(process) => process.comm,
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

pub fn detach() {
    unsafe { ATTACHED_PROCESS = None }
}

#[derive(Debug, Error)]
#[error("{error_count} error(s) occurred while attaching to the process")]
pub struct AttachError {
    pub error_count: usize,
}

pub fn attach_to_process(process: GameProcess) -> Result<(), AttachError> {
    let len = process.parse_errors.len();
    for err in &process.parse_errors {
        log_error(err)
    }
    unsafe { ATTACHED_PROCESS = Some(process); }
    if len > 0 {
        return Err(AttachError { error_count: len });
    }
    Ok(())
}

pub fn auto_attach() -> Option<Result<(), AttachError>> {
    let mut processes = get_processes();
    if let Some(process) = processes.pop() {
        return Some(attach_to_process(process));
    }
    None
}

pub fn get_processes() -> Vec<GameProcess> {
    let mut processes = Vec::new();
    let entries = fs::read_dir("/proc").unwrap();
    for process in entries.flatten() {
        if let Some(valid_process) = parse::parse_process(process) {
            processes.push(valid_process);
        }
    }
    processes
}

pub fn is_pid_valid() -> bool {
    let pid_path = Path::new("/proc").join(pid().to_string());
    let comm_path = Path::new(&pid_path).join("comm");
    if Path::exists(&pid_path)
        && let Ok(comm) = fs::read_to_string(comm_path)
        && (comm.trim() == crate::attached::comm()) {
        return true;
    }
    false
}