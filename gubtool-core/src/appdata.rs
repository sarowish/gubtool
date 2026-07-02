use std::{fmt::Display, fs::OpenOptions, io::{Read, Seek, SeekFrom, Write}, path::PathBuf};

#[derive(Debug)]
pub enum AppDataError {
    Env(std::env::VarError),
    Io(std::io::Error),
    Serialize(toml::ser::Error),
    Deserialize(toml::de::Error),
}

pub fn app_data_dir() -> Result<PathBuf, AppDataError> {
    #[cfg(windows)]
    let mut dir = PathBuf::from(std::env::var("APPDATA")?);

    #[cfg(unix)]
    let mut dir = PathBuf::from(std::env::var("HOME")?)
        .join(".local")
        .join("state");

    dir.push("gubtool");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

const MAX_LINES: usize = 500;

pub fn log_error(err: &impl Display) -> Result<(), AppDataError>{
    let msg = format!("{err}");
    let appdata_dir = app_data_dir()?;
    let log_path = appdata_dir.join("errors.log");

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&log_path)?;

    file.seek(SeekFrom::Start(0))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut lines: Vec<&str> = contents.lines().collect();

    if let Some(line) = lines.last() && line == &msg {
        return Ok(())
    }

    lines.push(&msg);

    if lines.len() > MAX_LINES {
        let excess = lines.len() - MAX_LINES;
        lines.drain(0..excess);
    }

    let mut file = OpenOptions::new()
        .read(true)
        .truncate(true)
        .write(true)
        .create(true)
        .open(&log_path)?;

    writeln!(file, "{}", lines.join("\n"))?;
    Ok(())
}

impl From<std::env::VarError> for AppDataError {
    fn from(err: std::env::VarError) -> Self {
        Self::Env(err)
    }
}

impl From<std::io::Error> for AppDataError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<toml::ser::Error> for AppDataError {
    fn from(err: toml::ser::Error) -> Self {
        Self::Serialize(err)
    }
}

impl From<toml::de::Error> for AppDataError {
    fn from(err: toml::de::Error) -> Self {
        Self::Deserialize(err)
    }
}

impl std::error::Error for AppDataError {}

impl Display for AppDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Env(err) => write!(f, "Env error: {}", err),
            Self::Io(err) => write!(f, "I/O error: {err}"),
            Self::Serialize(err) => write!(f, "Serialize error: {err}"),
            Self::Deserialize(err) => write!(f, "Deserialize error: {err}"),
        }
    }
}