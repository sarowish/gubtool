use crate::{
    attached::{
        AddressSize, GameProcess,
        error::ParseError,
        parse::{DEFAULT_BASE_64, parse_environ_for_path, pe_version_64, scan_maps_for_path},
    },
    game_version::{
        EldenRingVersion::{self, *},
        Game, GameVersion,
    },
};
use nix::unistd::Pid;
use std::path::PathBuf;

pub(super) fn parse(pid: Pid, comm: &'static str) -> GameProcess {
    let mut parse_errors: Vec<ParseError> = Vec::new();

    let (exe_path, module_base) = match scan_maps_for_path(pid, Game::EldenRing) {
        Ok((exe_path, module_base)) => (exe_path, module_base),
        Err(err) => {
            parse_errors.push(err);

            let exe_path = match parse_environ_for_path(pid, Game::EldenRing) {
                Ok(path) => path,
                Err(err) => {
                    parse_errors.push(err);
                    PathBuf::default()
                }
            };

            (exe_path, DEFAULT_BASE_64)
        }
    };
    let version = pe_version_64(&exe_path)
        .and_then(match_version)
        .unwrap_or_else(|err| {
            parse_errors.push(err);
            VersionUnknown
    });
    GameProcess {
        pid, comm, exe_path, module_base, parse_errors,
        game_version: GameVersion::EldenRing(version),
        address_size: AddressSize::Bits64,
    }
}

fn match_version((major, minor, patch): (u16, u16, u16)) -> Result<EldenRingVersion, ParseError> {
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
        _ => {
            return Err(ParseError::MatchProductVersion {
                product_version: (major, minor, patch),
            });
        }
    })
}