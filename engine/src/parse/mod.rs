mod darksouls2_parse;
mod eldenring_parse;

use crate::{attached::GameProcess, game_version::Game};
use anyhow::{Result, anyhow, bail};
use nix::unistd::Pid;
use pelite::{
    FileMap,
    pe32::{self, Pe as Pe32},
    pe64::{self, Pe as Pe64},
};
use std::{
    fs::{self, DirEntry},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

const DEFAULT_BASE_64: u64 = 0x140000000;
// const DEFAULT_BASE_32: u64 = 0x400000;
// const SHADPS4_BASE: u64 = 0x800000000;

const VALID_COMMS: &[(&str, Game); 4] = &[
    ("eldenring.exe", Game::EldenRing),
    ("start_protected", Game::EldenRing),
    ("start_protected_game.exe", Game::EldenRing),
    ("DarkSoulsII.exe", Game::DarkSouls2),
];

pub(crate) fn parse_process(process: DirEntry) -> Option<GameProcess> {
    let pid = process.file_name().into_string().unwrap();
    if !pid.chars().all(|c| c.is_numeric()) {
        return None;
    }
    let comm_path = Path::new("/proc").join(&pid).join("comm");
    let Ok(comm) = fs::read_to_string(comm_path) else {
        return None;
    };

    for (valid_comm, game) in VALID_COMMS {
        if comm.trim() == *valid_comm {
            let pid = Pid::from_raw(pid.parse::<i32>().unwrap());

            let process = match game {
                Game::EldenRing => eldenring_parse::parse(pid, valid_comm),
                Game::DarkSouls2 => darksouls2_parse::parse(pid, valid_comm),
            };
            return Some(process);
        }
    }
    None
}

fn scan_maps_for_path_and_base(pid: Pid, game: Game) -> Result<(PathBuf, u64)> {
    let maps_path = Path::new("/proc").join(pid.to_string()).join("maps");
    let maps_file = fs::File::open(maps_path).map_err(|err| anyhow!("Could not read /proc/{}/maps ({})", pid, err))?;
    let reader = BufReader::new(maps_file);
    let valid_exe_names = valid_exe_names(game);

    for line in reader.lines() {
        let line = line.unwrap();
        for name in valid_exe_names {
            if line.contains(name) {
                let base = line.split_once('-')
                    .map(|(handle, _)| u64::from_str_radix(handle, 16))
                    .unwrap();

                let pos = line.find('/').unwrap();
                let exe_path = PathBuf::from(&line[pos..]);
                return Ok((exe_path, base?))
            }
        }
    }
    bail!("exe not found in memory maps")
}

fn valid_exe_names(game: Game) -> &'static [&'static str] {
    match game {
        Game::EldenRing => &["eldenring.exe", "start_protected_game.exe"],
        Game::DarkSouls2 => &["DarkSoulsII.exe"],
    }
}

fn pe_version_64(path: &PathBuf) -> Result<(u16, u16, u16)> {
    let file_map = FileMap::open(path)?;
    let pe = pe64::PeFile::from_bytes(&file_map)?;
    let resources = pe.resources()?;
    let version_info = resources.version_info()?;
    let product_version = version_info.fixed().unwrap().dwProductVersion;
    Ok((
        product_version.Major,
        product_version.Minor,
        product_version.Patch,
    ))
}

fn pe_version_32(path: &PathBuf) -> Result<(u16, u16, u16)> {
    let file_map = FileMap::open(path)?;
    let pe = pe32::PeFile::from_bytes(&file_map)?;
    let resources = pe.resources()?;
    let version_info = resources.version_info()?;
    let product_version = version_info.fixed().unwrap().dwProductVersion;
    Ok((
        product_version.Major,
        product_version.Minor,
        product_version.Patch,
    ))
}