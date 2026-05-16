use crate::core::attach::{Version, module_handle, version};

pub fn warp_coord_write() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x7F9FB0,
        Version::Vanilla1_0_12 => 0x8015B0,
        Version::Scholar1_0_2 => 0x711939,
        Version::Scholar1_0_3 => 0x718E99,
        _ => 0x0,
    }
}

pub fn set_shared_flag() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x43120B,
        Version::Vanilla1_0_12 => 0x43849B,
        Version::Scholar1_0_2 => 0x41F452,
        Version::Scholar1_0_3 => 0x4265D2,
        _ => 0x0,
    }
}

pub fn locked_target() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x49E271,
        Version::Vanilla1_0_12 => 0x4A54F1,
        Version::Scholar1_0_2 => 0x495FB2,
        Version::Scholar1_0_3 => 0x49D192,
        _ => 0x0,
    }
}

pub fn credits_skip() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x11BD53,
        Version::Vanilla1_0_12 => 0x11BE23,
        Version::Scholar1_0_2 => 0x599D4,
        Version::Scholar1_0_3 => 0x59A64,
        _ => 0x0,
    }
}

pub fn faster_menu() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x19979E,
        Version::Vanilla1_0_12 => 0x1999BE,
        Version::Scholar1_0_2 => 0x1053B3,
        Version::Scholar1_0_3 => 0x105473,
        _ => 0x0,
    }
}

pub fn event_log() -> u64 {
    module_handle() + match version() {
        Version::Vanilla1_0_11 => 0x47884B,
        Version::Vanilla1_0_12 => 0x47FAEB,
        Version::Scholar1_0_2 => 0x46DED0,
        Version::Scholar1_0_3 => 0x4750C0,
        _ => 0x0,
    }
}