use crate::slice_ops::read_from_slice;
use anyhow::{Ok, Result, anyhow, ensure};
use chrono::{DateTime, Local};
use std::{
    collections::HashMap,
    env,
    fs::{OpenOptions, create_dir_all},
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Clone, Copy)]
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
    pub records: Vec<EventRecord>,
    excluded: Vec<u32>,
    push_duplicates: bool,
    dupe_map: HashMap<u32, bool>,
    read_idx: i32,
}

impl EventLog {
    pub fn poll(&mut self, write_idx: i32, buffer: &[u8]) -> Result<()> {
        let now = Local::now();
        let num_to_read = (write_idx - self.read_idx) & 511;
        for i in 0..num_to_read {
            let idx = (self.read_idx + i) & 511;
            let read_offset = idx * 5;

            let record = EventRecord::read_at(&buffer, read_offset as u64, now)?;

            if (self.push_duplicates || self.dupe_map.get(&record.event_id) != Some(&record.state))
                && !self.excluded.contains(&record.event_id)
                && self.records.len() <= 5000
            {
                self.records.push(record);
            }
            self.dupe_map.insert(record.event_id, record.state);
        }
        self.read_idx = write_idx;
        Ok(())
    }

    pub fn exclude(&mut self, event_id: u32) {
        if !self.excluded.contains(&event_id) {
            self.excluded.push(event_id);
            self.records.retain(|record| record.event_id != event_id);
        }
    }

    pub fn export(&self, file_prefix: &'static str) -> Result<String> {
        ensure!(!self.records.is_empty(), "Log is empty");
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

        for record in &self.records {
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
}

pub trait EventLogger {
    fn event_log(&self) -> &EventLog;
    fn event_log_mut(&mut self) -> &mut EventLog;
    fn file_prefix(&self) -> &'static str;
    fn read_buffer(&self) -> Result<[u8; 0x1000]>;
    fn write_idx(&self) -> Result<i32>;
    fn clear_cave(&self) -> Result<()>;

    fn poll(&mut self) -> Result<()> {
        let bytes = self.read_buffer()?;
        let write_idx = self.write_idx()?;
        self.event_log_mut().poll(write_idx, &bytes)
    }

    fn clear(&mut self) -> Result<()> {
        let log = self.event_log_mut();
        log.records.clear();
        log.dupe_map.clear();
        log.read_idx = 0;
        self.clear_cave()
    }

    fn export(&self) -> Result<String> {
        self.event_log().export(self.file_prefix())
    }

    fn entries(&self) -> &Vec<EventRecord> {
        &self.event_log().records
    }

    fn exclude(&mut self, event_id: u32) {
        self.event_log_mut().exclude(event_id)
    }

    fn get_excluded(&self) -> &Vec<u32> {
        &self.event_log().excluded
    }

    fn is_show_duplicates(&self) -> bool {
        self.event_log().push_duplicates
    }
    fn toggle_show_duplicates(&mut self) {
        let log = self.event_log_mut();
        log.push_duplicates = !log.push_duplicates
    }
}