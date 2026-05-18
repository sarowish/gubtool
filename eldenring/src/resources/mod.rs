pub mod aow;
pub mod bosses;
pub mod graces;
pub mod items;
pub mod talk_commands;

use shared::object::AsmFolder;
use std::{env, sync::LazyLock};

static ASM_LIB_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/eldenring.bin"));

pub static ASM: LazyLock<AsmFolder> = LazyLock::new(|| bincode::deserialize(ASM_LIB_BYTES).unwrap());
