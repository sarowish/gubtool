use crate::{
    game_version::{Game, GameVersion, Version},
    parse,
};
use nix::unistd::Pid;
use std::{
    fs,
    path::{Path, PathBuf},
};

static mut ATTACHED_PROCESS: Option<GameProcess> = None;

pub struct GameProcess {
    pub pid: Pid,
    pub game_version: GameVersion,
    pub comm: &'static str,
    pub path: PathBuf,
    pub module_base: u64,
    pub attach_error: Option<String>,
}

pub fn pid() -> Pid {
    unsafe {
        let ptr = std::ptr::addr_of!(ATTACHED_PROCESS);
        match &*ptr {
            Some(process) => process.pid,
            None => Pid::from_raw(-1)
        }
    }
}

pub fn game_version() -> Option<GameVersion> {
    unsafe {
        let ptr = std::ptr::addr_of!(ATTACHED_PROCESS);
        match &*ptr {
            Some(process) => Some(process.game_version),
            None => None,
        }
    }
}

pub fn game() -> Option<Game> {
    unsafe {
        let ptr = std::ptr::addr_of!(ATTACHED_PROCESS);
        match &*ptr {
            Some(process) => Some(process.game_version.game()),
            None => None,
        }
    }
}

pub fn comm() -> &'static str {
    unsafe {
        let ptr = std::ptr::addr_of!(ATTACHED_PROCESS);
        match &*ptr {
            Some(process) => process.comm,
            None => "",
        }
    }
}

pub fn module_base() -> u64 {
    unsafe {
        let ptr = std::ptr::addr_of!(ATTACHED_PROCESS);
        match &*ptr {
            Some(process) => process.module_base,
            None => 0x0,
        }
    }
}

pub fn version<T: Version>() -> Option<T> {
    unsafe {
        let ptr = std::ptr::addr_of!(ATTACHED_PROCESS);
        match &*ptr {
            Some(process) => T::from_game_version(&process.game_version),
            None => None,
        }
    }
}

pub fn detach() {
    unsafe { ATTACHED_PROCESS = None }
}

pub fn attach_to_process(process: GameProcess) -> Option<String> {
    unsafe {
        let error = process.attach_error.clone();
        ATTACHED_PROCESS = Some(process);
        error
    }
}

pub fn auto_attach() -> Option<Option<String>> {
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