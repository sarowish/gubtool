pub mod code_cave;
pub mod functions;
pub mod game_manager_imp;
pub mod hooks;

use crate::core::attach::{Version, module_handle, version};

pub fn kernel32_create_thread() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x13726FC,
        Version::Vanilla1_0_12 => 0x14546FC,
        Version::Scholar1_0_2 => 0x1A631E4,
        Version::Scholar1_0_3 => 0x1AAE26C,
        _ => 0x0,
    }
}

pub fn kernel32_sleep() -> u64 {
    module_handle() + match version() {
        Version::Scholar1_0_1 => 0x0,
        Version::Scholar1_0_2 => 0x1A6328C,
        Version::Scholar1_0_3 => 0x1AAE314,
        _ => 0x0,
    }
}

pub fn area_id() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x10CD2D0,
        Version::Vanilla1_0_12 => 0x10D42D8,
        Version::Scholar1_0_2 => 0x15641B4,
        Version::Scholar1_0_3 => 0x156D1C4,
        _ => 0x0,
    }
}