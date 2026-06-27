use crate::{
    attached::{self, module_base, pid},
    sys::{error::ProcessError, read_unsafe},
};
use std::{
    fs,
    os::unix::fs::FileExt,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
};
use thiserror::Error;

const CHUNK_SIZE: usize = 0x5000;

pub enum ScanMode {
    Absolute,
    Direct32,
    Relative(i32),
}

pub struct AobScan {
    pub name: &'static str,
    pub pattern: &'static str,
    pub scan_origin: u64,
    pub offset: isize,
    pub scan_mode: ScanMode
}


#[derive(Debug, Error)]
pub enum ScanError {
    #[error("Could not parse IDA pattern: {failed_byte}")]
    ParsePattern {
        failed_byte: &'static str,
    },
    #[error("{scan_name} not found in memory")]
    NotFound {
        scan_name: &'static str,
    },
    #[error("Overflow when adding offset")]
    Overflow,
    #[error("Overflow when adding relative offset")]
    OverflowRelative,
    #[error("{err}")]
    ProcessError {
        err: ProcessError
    },
}

pub fn scan(scan: AobScan) -> Result<u64, ScanError> {
let pattern_bytes = parse_ida(scan.pattern)?;
    let mem_path = PathBuf::from(format!("/proc/{}/mem", pid()));
    let arc_file = fs::File::open(&mem_path).unwrap();
    let arc_found = Arc::new(AtomicUsize::new(usize::MAX));

    thread::scope(|scope| {
        let origin = module_base() + scan.scan_origin;
        let step = (CHUNK_SIZE - pattern_bytes.len()) as u64;

        let file = &arc_file;
        let found = arc_found.clone();
        let pattern = pattern_bytes.clone();
        scope.spawn(move || {
            let mut buffer = [0u8; CHUNK_SIZE];
            let mut offset = origin;
            while found.load(Ordering::Relaxed) == usize::MAX {
                if file.read_at(&mut buffer, offset).is_err() { return; };
                for i in 0..(buffer.len() - pattern.len()) {
                    if matches_pattern(&buffer[i..i + pattern.len()], &pattern) {
                        found.store(offset as usize + i, Ordering::Release);
                        return;
                    }
                }
                offset += step;
            }
        });

        let file = &arc_file;
        let found = arc_found.clone();
        let pattern = pattern_bytes.clone();
        scope.spawn(move || {
            let mut buffer = [0u8; CHUNK_SIZE];
            let mut offset = origin;
            while found.load(Ordering::Relaxed) == usize::MAX {
                let Some(next_offset) = offset.checked_sub(step) else { return; };
                if file.read_at(&mut buffer, next_offset).is_err() { return; }
                for i in (0 + pattern.len()..buffer.len()).rev() {
                    if matches_pattern(&buffer[i - pattern.len()..i], &pattern) {
                        found.store(offset as usize - (buffer.len() - i), Ordering::Release);
                        return;
                    }
                }
                offset = next_offset;
            }
        });
    });

    let addr = arc_found.load(Ordering::Relaxed);
    println!("{} pattern found at {:#X}", scan.name, addr.saturating_sub(attached::module_base() as usize));

    if addr != usize::MAX {
        let Some(addr) = addr.checked_add_signed(scan.offset) else {
            return Err(ScanError::Overflow)
        };
        let addr = addr as u64;
        let addr = match scan.scan_mode {
            ScanMode::Absolute => addr,
            ScanMode::Direct32 => read_unsafe::<u32>(addr)? as u64,
            ScanMode::Relative(bytes_to_next_instr) => {
                let offset = read_unsafe::<i32>(addr)?;
                let Some(addr) = addr.checked_add_signed((offset + bytes_to_next_instr) as i64) else {
                    return Err(ScanError::OverflowRelative)
                };
                addr
            }
        };
        println!("Final address at {:#X}\n", addr.saturating_sub(attached::module_base()));
        Ok(addr.saturating_sub(attached::module_base()))
    } else {
        Err(ScanError::NotFound { scan_name: scan.name })
    }
}

fn parse_ida(pattern: &'static str) -> Result<Vec<Option<u8>>, ScanError> {
    let mut bytes: Vec<Option<u8>> = Vec::new();
    for byte in pattern.split_whitespace() {
        if byte == "?" {
            bytes.push(None)
        } else {
            let b = u8::from_str_radix(byte, 16)
                .map_err(|_| ScanError::ParsePattern { failed_byte: byte })?;
            bytes.push(Some(b))
        }
    }
    Ok(bytes)
}

fn matches_pattern(slice: &[u8], pattern: &[Option<u8>]) -> bool {
    slice
        .iter()
        .zip(pattern)
        .all(|(&slice_byte, &pattern_byte)| {
            pattern_byte.map_or(true, |pattern_byte| pattern_byte == slice_byte)
        })
}

impl From<ProcessError> for ScanError {
    fn from(err: ProcessError) -> Self {
        Self::ProcessError { err }
    }
}