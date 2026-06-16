mod darksouls2_parse;
mod eldenring_parse;

use crate::{attached::{GameProcess, error::{ParseError, ParsePeError}}, game_version::Game};
use nix::unistd::Pid;
use pelite::{
    FileMap,
    pe32::{self, Pe as Pe32},
    pe64::{self, Pe as Pe64}, resources::FindError,
};
use std::{
    fs::{self, DirEntry},
    io::{BufRead, BufReader, Read},
    path::PathBuf,
};

const DEFAULT_BASE_64: u64 = 0x140000000;
const DEFAULT_BASE_32: u64 = 0x400000;
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
    let comm_path = format!("/proc/{pid}/comm");
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

fn parse_environ_for_path(pid: Pid, game: Game) -> Result<PathBuf, ParseError> {
    let path = format!("/proc/{pid}/environ");
    let target_field = "PWD";
    let mut file = fs::File::open(path).map_err(|err| {
        ParseError::ExeNotFound { pid, error_kind: Some(err.kind()) }
    })?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|err| {
        ParseError::ExeNotFound { pid, error_kind: Some(err.kind()) }
    })?;

    for env_var_bytes in buffer.split(|&b| b == 0) {
        if env_var_bytes.is_empty() {
            continue;
        }

        let env_var_str = String::from_utf8_lossy(env_var_bytes);

        if let Some((field, value)) = env_var_str.split_once('=') {
            if field == target_field {
                for name in valid_exe_names(game) {
                    let exe_path = PathBuf::from(value).join(name);
                    if exe_path.exists() {
                        return Ok(exe_path);
                    }
                }
            }
        }
    }
    Err(ParseError::ExeNotFound { pid, error_kind: None })
}

fn scan_maps_for_path(pid: Pid, game: Game) -> Result<(PathBuf, u64), ParseError> {
    let path = format!("/proc/{pid}/maps");
    let file = fs::File::open(path).map_err(|err| {
        return ParseError::ScanMaps { pid, error_kind: Some(err.kind()) }
    })?;
    let reader = BufReader::new(file);
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
                return Ok((exe_path, base.unwrap()))
            }
        }
    }
    Err(ParseError::ScanMaps { pid, error_kind: None })
}

fn valid_exe_names(game: Game) -> &'static [&'static str] {
    match game {
        Game::EldenRing => &["eldenring.exe", "start_protected_game.exe"],
        Game::DarkSouls2 => &["DarkSoulsII.exe"],
    }
}

fn pe_version_64(path: &PathBuf) -> Result<(u16, u16, u16), ParseError> {
    let file_map = FileMap::open(path).map_err(|err| {
        ParseError::ParsePe { error: ParsePeError::IoError(err.kind()) }
    })?;
    let version_info = pe64::PeFile::from_bytes(&file_map)
        .and_then(|pe| pe.resources()) .map_err(|err| { FindError::from(err) })
        .and_then(|resources| resources.version_info())
        .map_err(|err| {
            ParseError::ParsePe { error: ParsePeError::PeError(FindError::from(err)) }
        })?;
    let product_version = version_info.fixed().unwrap().dwProductVersion;
    Ok((
        product_version.Major,
        product_version.Minor,
        product_version.Patch,
    ))
}

fn pe_version_32(path: &PathBuf) -> Result<(u16, u16, u16), ParseError> {
    let file_map = FileMap::open(path).map_err(|err| {
        ParseError::ParsePe { error: ParsePeError::IoError(err.kind()) }
    })?;
    let version_info = pe32::PeFile::from_bytes(&file_map)
        .and_then(|pe| pe.resources()) .map_err(|err| { FindError::from(err) })
        .and_then(|resources| resources.version_info())
        .map_err(|err| {
            ParseError::ParsePe { error: ParsePeError::PeError(FindError::from(err)) }
        })?;
    let product_version = version_info.fixed().unwrap().dwProductVersion;
    Ok((
        product_version.Major,
        product_version.Minor,
        product_version.Patch,
    ))
}