use crate::{
    attached::GameProcess,
    game_version::{
        DarkSouls2Version::{self, *},
        Game, GameVersion,
    },
    parse::{DEFAULT_BASE_64, pe_version_32, pe_version_64, scan_maps_for_path_and_base},
};
use anyhow::{Result, bail};
use nix::unistd::Pid;
use std::path::PathBuf;

pub(super) fn parse(pid: Pid, comm: &'static str) -> GameProcess {
    let mut error_string: Option<String> = None;
    let (path, module_base) = scan_maps_for_path_and_base(pid, Game::DarkSouls2)
        .unwrap_or_else(|err| {
            error_string = Some(err.to_string());
            (PathBuf::default(), DEFAULT_BASE_64)
    });
    let mut version = ScholarUnknown;
    if let Ok(version_info) = pe_version_64(&path) {
        match match_scholar_version(version_info) {
            Ok(v) => version = v,
            Err(err) => {
                if error_string.is_none() {
                    error_string = Some(err.to_string());
                }
            }
        }
    } else if let Ok(version_info) = pe_version_32(&path) {
        match match_vanilla_version(version_info) {
            Ok(v) => version = v,
            Err(err) => {
                if error_string.is_none() {
                    error_string = Some(err.to_string());
                }
                version = VanillaUnknown;
            }
        }
    }
    GameProcess {
        pid,
        game_version: GameVersion::DarkSouls2(version),
        comm,
        path,
        module_base,
        attach_error: error_string,
    }
}

fn match_vanilla_version((major, minor, patch): (u16, u16, u16)) -> Result<DarkSouls2Version> {
    Ok(match (major, minor, patch) {
        (1, 0, 3) => Vanilla1_0_3,
        (1, 0, 4) => Vanilla1_0_4,
        (1, 0, 5) => Vanilla1_0_5,
        (1, 0, 6) => Vanilla1_0_5,
        (1, 0, 7) => Vanilla1_0_7,
        (1, 0, 10) => Vanilla1_0_10,
        (1, 0, 11) => Vanilla1_0_11,
        (1, 0, 12) => Vanilla1_0_12,
        _ => bail!("Could not match product version ({}, {}, {})", major, minor, patch)
    })
}

fn match_scholar_version((major, minor, patch): (u16, u16, u16)) -> Result<DarkSouls2Version> {
    Ok(match (major, minor, patch) {
        (1, 0, 1) => Scholar1_0_1,
        (1, 0, 2) => Scholar1_0_2,
        (1, 0, 3) => Scholar1_0_3,
        _ => bail!("Could not match product version ({}, {}, {})", major, minor, patch)
    })
}