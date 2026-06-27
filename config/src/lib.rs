pub mod attach;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub trait Config: Serialize + for<'a> Deserialize<'a> + Default + Clone {
    fn get_path() -> Result<PathBuf>;
    fn read() -> Result<Self> where Self: Sized;
    fn write(&self) -> Result<()>;
    fn update<F>(modifier: F) -> Result<()>
    where F: FnOnce(&mut Self);
}