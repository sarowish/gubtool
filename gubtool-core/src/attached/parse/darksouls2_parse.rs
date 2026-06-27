use crate::{
    attached::{
        AddressSize, GameProcess,
        error::ParseError,
        parse::{
            DEFAULT_BASE_32, DEFAULT_BASE_64, parse_environ_for_path, pe_version_32, pe_version_64,
            scan_maps_for_path,
        },
    },
    game_version::{
        DarkSouls2Version::{self, *},
        Game, GameVersion,
    },
};
use nix::unistd::Pid;
use std::path::PathBuf;

pub(super) fn parse(pid: Pid, comm: &'static str) -> GameProcess {
    let mut parse_errors: Vec<ParseError> = Vec::new();

    let (exe_path, mut module_base) = match scan_maps_for_path(pid, Game::DarkSouls2) {
        Ok((exe_path, module_base)) => (exe_path, module_base),
        Err(err) => {
            parse_errors.push(err);

            let exe_path = match parse_environ_for_path(pid, Game::DarkSouls2) {
                Ok(path) => path,
                Err(err) => {
                    parse_errors.push(err);
                    PathBuf::default()
                }
            };

            (exe_path, 0)
        }
    };
    let mut version = ScholarUnknown;
    let mut address_size = AddressSize::Bits64;
    if let Ok(version_info) = pe_version_64(&exe_path) {
        match match_scholar_version(version_info) {
            Ok(v) => version = v,
            Err(err) => parse_errors.push(err),
        }
    } else if let Ok(version_info) = pe_version_32(&exe_path) {
        address_size = AddressSize::Bits32;
        match match_vanilla_version(version_info) {
            Ok(v) => version = v,
            Err(err) => {
                parse_errors.push(err);
                version = VanillaUnknown;
            }
        }
    }
    if module_base == 0 {
        match address_size {
            AddressSize::Bits64 => module_base = DEFAULT_BASE_64,
            AddressSize::Bits32 => module_base = DEFAULT_BASE_32,
        }
    }
    GameProcess {
        pid, comm, exe_path, module_base, address_size, parse_errors,
        game_version: GameVersion::DarkSouls2(version),
    }
}

fn match_vanilla_version(
    (major, minor, patch): (u16, u16, u16),
) -> Result<DarkSouls2Version, ParseError> {
    Ok(match (major, minor, patch) {
        (1, 0, 3) => Vanilla1_0_3,
        (1, 0, 4) => Vanilla1_0_4,
        (1, 0, 5) => Vanilla1_0_5,
        (1, 0, 6) => Vanilla1_0_5,
        (1, 0, 7) => Vanilla1_0_7,
        (1, 0, 10) => Vanilla1_0_10,
        (1, 0, 11) => Vanilla1_0_11,
        (1, 0, 12) => Vanilla1_0_12,
        _ => {
            return Err(ParseError::MatchProductVersion {
                product_version: (major, minor, patch),
            });
        }
    })
}

fn match_scholar_version(
    (major, minor, patch): (u16, u16, u16),
) -> Result<DarkSouls2Version, ParseError> {
    Ok(match (major, minor, patch) {
        (1, 0, 1) => Scholar1_0_1,
        (1, 0, 2) => Scholar1_0_2,
        (1, 0, 3) => Scholar1_0_3,
        _ => {
            return Err(ParseError::MatchProductVersion {
                product_version: (major, minor, patch),
            });
        }
    })
}
