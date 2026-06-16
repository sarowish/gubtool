use std::env;
use std::fmt::Display;
use std::fs::{OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

const MAX_LINES: usize = 500;

pub fn log_error(err: &impl Display) {
    let msg = format!("{err}");

    let Some(home_dir) = env::home_dir() else {
        return
    };

    let from_home = PathBuf::new()
        .join(".local")
        .join("state")
        .join("gubtool")
        .join("errors.log");
    let log_path = home_dir.join(&from_home);

    let mut file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&log_path)
    {
        Ok(f) => f,
        Err(_) => return,
    };

    let _ = file.seek(SeekFrom::Start(0));
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    let mut lines: Vec<&str> = contents.lines().collect();

    if let Some(line) = lines.last() && line == &msg {
        return
    }

    lines.push(&msg);

    if lines.len() > MAX_LINES {
        let excess = lines.len() - MAX_LINES;
        lines.drain(0..excess);
    }

    let mut file = match OpenOptions::new()
        .read(true)
        .truncate(true)
        .write(true)
        .create(true)
        .open(&log_path)
    {
        Ok(f) => f,
        Err(_) => return,
    };

    let _ = writeln!(file, "{}", lines.join("\n"));
}