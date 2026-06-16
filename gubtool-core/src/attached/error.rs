use nix::unistd::Pid;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum ParseError {
    ScanMaps {
        pid: Pid,
        error_kind: Option<std::io::ErrorKind>,

    },
    ExeNotFound {
        pid: Pid,
        error_kind: Option<std::io::ErrorKind>,
    },
    ParsePe {
        error: ParsePeError,
    },
    MatchProductVersion {
        product_version: (u16, u16, u16)
    },
}

#[derive(Debug, Clone, Copy)]
pub enum ParsePeError {
    IoError(std::io::ErrorKind),
    PeError(pelite::resources::FindError),
}

impl std::error::Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MatchProductVersion { product_version: (major, minor, patch) } => {
                write!(f, "Could not match product version ({major}, {minor}, {patch})")
            }
            Self::ExeNotFound { pid, error_kind } => {
                if let Some(error_kind) = error_kind {
                    write!(f, "Could not read /proc/{pid}/environ: {error_kind}")
                } else {
                    write!(f, "Environment variable PWD not found in /proc/{pid}/environ")
                }
            }
            Self::ParsePe { error } => {
                match error {
                    ParsePeError::IoError(kind) => {
                        write!(f, "Couldn't open filemap: {kind}")
                    }
                    ParsePeError::PeError(err) => {
                        write!(f, "Error while parsing PE file: {err}")
                    }
                }
            }
            Self::ScanMaps { pid, error_kind } => {
                if let Some(error_kind) = error_kind {
                    write!(f, "Could not read /proc/{pid}/maps: {error_kind}")
                } else {
                    write!(f, ".exe not found in /proc/{pid}/maps")
                }
            }
        }
    }
}