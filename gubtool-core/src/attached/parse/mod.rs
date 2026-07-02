use crate::{
    attached::{AddressSize, ParseState},
    game_version::{DarkSouls2Version, EldenRingVersion, Game, GameVersion},
};
use pelite::{
    FileMap,
    pe32::{self, Pe as Pe32},
    pe64::{self, Pe as Pe64},
    resources::FindError,
};
use std::{error::Error, fmt::Display, path::PathBuf};


#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::*;
#[cfg(windows)]
mod win;
#[cfg(windows)]
pub use win::*;

// const SHADPS4_BASE: u64 = 0x800000000;

pub const VALID_COMMS: &[(&str, Game); 4] = &[
    ("eldenring.exe", Game::EldenRing),
    ("start_protected", Game::EldenRing),
    ("start_protected_game.exe", Game::EldenRing),
    ("DarkSoulsII.exe", Game::DarkSouls2),
];


#[derive(Debug, Clone, Copy)]
pub enum ParsePeError {
    IoError(std::io::ErrorKind),
    PeError(pelite::resources::FindError),
}

fn parse_file_for_version_and_address_size(
    game: &Game,
    exe_path: &PathBuf,
    mut parse_errors: Vec<ParseError>,
) -> (AddressSize, GameVersion, ParseState) {
    let mut address_size = AddressSize::Bits64;

    let version_info = match pe_version_64(&exe_path) {
        Ok(v) => v,
        Err(ParseError::ParsePe {
            error: ParsePeError::PeError(pelite::resources::FindError::Pe(pelite::Error::PeMagic))
        }) => {
            address_size = AddressSize::Bits32;
            match pe_version_32(&exe_path) {
                Ok(v) => v,
                Err(err) => {
                    parse_errors.push(err);
                    Default::default()
                }
            }
        }
        Err(err) => {
            parse_errors.push(err);
            Default::default()
        }
    };

    let game_version = match game {
        Game::DarkSouls2 => {
            let version = match address_size {
                AddressSize::Bits32 => match match_vanilla(version_info) {
                    Ok(v) => v,
                    Err(err) => {
                        parse_errors.push(err);
                        DarkSouls2Version::VanillaUnknown
                    }
                }
                AddressSize::Bits64 => match match_scholar(version_info) {
                    Ok(v) => v,
                    Err(err) => {
                        parse_errors.push(err);
                        DarkSouls2Version::ScholarUnknown
                    }
                }
            };
            GameVersion::DarkSouls2(version)
        }
        Game::EldenRing => {
            let version = match match_eldenring(version_info) {
                Ok(v) => v,
                Err(err) => {
                    parse_errors.push(err);
                    EldenRingVersion::VersionUnknown
                }
            };
            GameVersion::EldenRing(version)
        }
    };

    let parse_state = if parse_errors.is_empty() {
        ParseState::Valid
    } else {
        ParseState::Invalid(parse_errors)
    };

    (address_size, game_version, parse_state)
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

fn match_vanilla(
    (major, minor, patch): (u16, u16, u16),
) -> Result<DarkSouls2Version, ParseError> {
    Ok(match (major, minor, patch) {
        (1, 0, 3) => DarkSouls2Version::Vanilla1_0_3,
        (1, 0, 4) => DarkSouls2Version::Vanilla1_0_4,
        (1, 0, 5) => DarkSouls2Version::Vanilla1_0_5,
        (1, 0, 6) => DarkSouls2Version::Vanilla1_0_5,
        (1, 0, 7) => DarkSouls2Version::Vanilla1_0_7,
        (1, 0, 10) => DarkSouls2Version::Vanilla1_0_10,
        (1, 0, 11) => DarkSouls2Version::Vanilla1_0_11,
        (1, 0, 12) => DarkSouls2Version::Vanilla1_0_12,
        _ => {
            return Err(ParseError::MatchProductVersion {
                product_version: (major, minor, patch),
            });
        }
    })
}

fn match_scholar(
    (major, minor, patch): (u16, u16, u16),
) -> Result<DarkSouls2Version, ParseError> {
    Ok(match (major, minor, patch) {
        (1, 0, 1) => DarkSouls2Version::Scholar1_0_1,
        (1, 0, 2) => DarkSouls2Version::Scholar1_0_2,
        (1, 0, 3) => DarkSouls2Version::Scholar1_0_3,
        _ => {
            return Err(ParseError::MatchProductVersion {
                product_version: (major, minor, patch),
            });
        }
    })
}

fn match_eldenring((major, minor, patch): (u16, u16, u16)) -> Result<EldenRingVersion, ParseError> {
    Ok(match (major, minor, patch) {
        (1, 2, 0) => EldenRingVersion::Version1_2_0,
        (1, 2, 1) => EldenRingVersion::Version1_2_1,
        (1, 2, 2) => EldenRingVersion::Version1_2_2,
        (1, 2, 3) => EldenRingVersion::Version1_2_3,
        (1, 3, 0) => EldenRingVersion::Version1_3_0,
        (1, 3, 1) => EldenRingVersion::Version1_3_1,
        (1, 3, 2) => EldenRingVersion::Version1_3_2,
        (1, 4, 0) => EldenRingVersion::Version1_4_0,
        (1, 4, 1) => EldenRingVersion::Version1_4_1,
        (1, 5, 0) => EldenRingVersion::Version1_5_0,
        (1, 6, 0) => EldenRingVersion::Version1_6_0,
        (1, 7, 0) => EldenRingVersion::Version1_7_0,
        (1, 8, 0) => EldenRingVersion::Version1_8_0,
        (1, 8, 1) => EldenRingVersion::Version1_8_1,
        (1, 9, 0) => EldenRingVersion::Version1_9_0,
        (1, 9, 1) => EldenRingVersion::Version1_9_1,
        (2, 0, 0) => EldenRingVersion::Version2_0_0,
        (2, 0, 1) => EldenRingVersion::Version2_0_1,
        (2, 2, 0) => EldenRingVersion::Version2_2_0,
        (2, 2, 3) => EldenRingVersion::Version2_2_3,
        (2, 3, 0) => EldenRingVersion::Version2_3_0,
        (2, 4, 0) => EldenRingVersion::Version2_4_0,
        (2, 5, 0) => EldenRingVersion::Version2_5_0,
        (2, 6, 0) => EldenRingVersion::Version2_6_0,
        (2, 6, 1) => EldenRingVersion::Version2_6_1,
        (2, 6, 2) => EldenRingVersion::Version2_6_2,
        _ => {
            return Err(ParseError::MatchProductVersion {
                product_version: (major, minor, patch),
            });
        }
    })
}

impl Display for ParsePeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(kind) => {
                write!(f, "Couldn't open filemap: {kind}")
            }
            Self::PeError(err) => {
                write!(f, "Error while parsing PE file: {err}")
            }
        }
    }
}

impl Error for ParsePeError {}