use crate::{
    attached::GameProcess,
    game_version::{
        EldenRingVersion::{self, *},
        Game, GameVersion,
    },
    parse::{DEFAULT_BASE_64, pe_version_64, scan_maps_for_path_and_base},
};
use anyhow::{Result, bail};
use nix::unistd::Pid;
use std::path::PathBuf;

pub(super) fn parse(pid: Pid, comm: &'static str) -> GameProcess {
    let mut error_string: Option<String> = None;
    let (path, module_base) = scan_maps_for_path_and_base(pid, Game::EldenRing)
        .unwrap_or_else(|err| {
            error_string = Some(err.to_string());
            (PathBuf::default(), DEFAULT_BASE_64)
    });
    let version = match pe_version_64(&path) {
        Ok(version_info) => match match_version(version_info) {
            Ok(version) => version,
            Err(err) => {
                if error_string.is_none() {
                    error_string = Some(err.to_string());
                }
                VersionUnknown
            }
        },
        Err(err) => {
            if error_string.is_none() {
                error_string = Some(err.to_string());
            }
            VersionUnknown
        }
    };
    GameProcess {
        pid,
        game_version: GameVersion::EldenRing(version),
        comm,
        path,
        module_base,
        attach_error: error_string,
    }
}

fn match_version((major, minor, patch): (u16, u16, u16)) -> Result<EldenRingVersion> {
    Ok(match (major, minor, patch) {
        (1, 2, 0) => Version1_2_0,
        (1, 2, 1) => Version1_2_1,
        (1, 2, 2) => Version1_2_2,
        (1, 2, 3) => Version1_2_3,
        (1, 3, 0) => Version1_3_0,
        (1, 3, 1) => Version1_3_1,
        (1, 3, 2) => Version1_3_2,
        (1, 4, 0) => Version1_4_0,
        (1, 4, 1) => Version1_4_1,
        (1, 5, 0) => Version1_5_0,
        (1, 6, 0) => Version1_6_0,
        (1, 7, 0) => Version1_7_0,
        (1, 8, 0) => Version1_8_0,
        (1, 8, 1) => Version1_8_1,
        (1, 9, 0) => Version1_9_0,
        (1, 9, 1) => Version1_9_1,
        (2, 0, 0) => Version2_0_0,
        (2, 0, 1) => Version2_0_1,
        (2, 2, 0) => Version2_2_0,
        (2, 2, 3) => Version2_2_3,
        (2, 3, 0) => Version2_3_0,
        (2, 4, 0) => Version2_4_0,
        (2, 5, 0) => Version2_5_0,
        (2, 6, 0) => Version2_6_0,
        (2, 6, 1) => Version2_6_1,
        (2, 6, 2) => Version2_6_2,
        _ => bail!("Could not match product version ({}, {}, {})", major, minor, patch)
    })
}