use crate::slice_ops::read_from_slice;
use anyhow::{Ok, Result, anyhow};
use chrono::{DateTime, Local};
use std::{
    env,
    fs::{OpenOptions, create_dir_all},
    io::Write,
    path::{Path, PathBuf},
};

pub struct EventRecord {
    pub event_id: u32,
    pub state: bool,
    pub time_stamp: DateTime<Local>,
}

impl EventRecord {
    fn read_at(bytes: &[u8], offset: u64, time_stamp: DateTime<Local>) -> Result<Self> {
        Ok(Self {
            event_id: read_from_slice::<u32>(bytes, offset)?,
            state: read_from_slice::<u8>(bytes, offset + 4)? != 0x0,
            time_stamp,
        })
    }
}

#[derive(Default)]
pub struct EventLog {
    pub event_records: Vec<EventRecord>,
    read_idx: u64,
}

impl EventLog {
    pub fn poll(&mut self, bytes: &[u8]) -> Result<()> {
        let now = Local::now();
        for i in self.read_idx..bytes.len() as u64 {
            self.read_idx = i;
            let record = EventRecord::read_at(&bytes, i * 5, now)?;
            if record.event_id == 0x0 {
                break;
            }
            self.event_records.push(record);
        }
        Ok(())
    }
    pub fn export(&self, file_prefix: &'static str) -> Result<String> {
        let Some(home_dir) = env::home_dir() else {
            return Err(anyhow!("Home directory not found"));
        };

        let time = Local::now().format("%H:%M:%S");

        let from_home = PathBuf::new()
            .join(".local")
            .join("state")
            .join("gubtool")
            .join("logs")
            .join(format!("{file_prefix}_event_{time}.log"));
        let log_path = home_dir.join(&from_home);

        let parent = Path::new(&log_path).parent().expect("Invalid path");

        create_dir_all(parent)?;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)?;

        for record in &self.event_records {
            let time_stamp = record.time_stamp.format("%H:%M:%S");
            writeln!(
                file,
                "{} {:<10} {}",
                time_stamp,
                record.event_id,
                record.state.to_string().to_uppercase(),
            )?;
        }
        Ok(format!("~/{}", from_home.display().to_string()))
    }
    pub fn reset(&mut self) {
        self.event_records.clear();
        self.read_idx = 0;
    }
}

pub trait EventLogger {
    fn get_entries(&self) -> &Vec<EventRecord>;
    fn poll(&mut self) -> Result<()>;
    fn clear(&mut self) -> Result<()>;
    fn export(&self) -> Result<String>;
}
