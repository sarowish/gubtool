use crate::offsets::module_offsets;
use engine::{
    attached::{module_base, version},
    game_version::EldenRingVersion::*,
};

pub fn base_ptr() -> u64 {
    module_base() + module_offsets().base_ptrs.world_chr_man
}

pub fn chr_set_pool() -> u64 {
    match version() {
        Some(Version1_2_0) | Some(Version1_2_1) | Some(Version1_2_2) |
        Some(Version1_2_3) | Some(Version1_3_0) | Some(Version1_3_1) |
        Some(Version1_3_2) | Some(Version1_4_0) | Some(Version1_4_1) |
        Some(Version1_5_0) | Some(Version1_6_0) => 0x18038,
        _ => 0x1DED8,
    }
}

pub mod chr_set_offsets {
    pub const CHR_SET_ENTRIES: u64 = 0x18;
}

pub fn player_ins() -> u64 {
    match version() {
        Some(Version1_2_0) | Some(Version1_2_1) | Some(Version1_2_2) |
        Some(Version1_2_3) | Some(Version1_3_0) | Some(Version1_3_1) |
        Some(Version1_3_2) | Some(Version1_4_0) | Some(Version1_4_1) |
        Some(Version1_5_0) | Some(Version1_6_0) => 0x18468,
        _ => 0x1E508,
    }
}

pub mod player_ins_offsets {
    use engine::{attached::version, game_version::EldenRingVersion::*};


    pub fn current_block_id() -> u64 {
        match version() {
            Some(Version1_2_0) | Some(Version1_2_1) | Some(Version1_2_2) |
            Some(Version1_2_3) | Some(Version1_3_0) | Some(Version1_3_1) |
            Some(Version1_3_2) => 0x6C8,
            Some(Version1_4_0) | Some(Version1_4_1) | Some(Version1_5_0) |
            Some(Version1_6_0) | Some(Version1_7_0) => 0x6C0,
            _ => 0x6D0,
        }
    }
    pub fn current_map_coords() -> u64 {
        match version() {
            Some(Version1_2_0) | Some(Version1_2_1) | Some(Version1_2_2) |
            Some(Version1_2_3) | Some(Version1_3_0) | Some(Version1_3_1) |
            Some(Version1_3_2) => 0x6B8,
            Some(Version1_4_0) | Some(Version1_4_1) | Some(Version1_5_0) |
            Some(Version1_6_0) | Some(Version1_7_0) => 0x6B0,
            _ => 0x6C0,
        }
    }
    pub fn current_map_angle() -> u64 {
        match version() {
            Some(Version1_2_0) | Some(Version1_2_1) | Some(Version1_2_2) |
            Some(Version1_2_3) | Some(Version1_3_0) | Some(Version1_3_1) |
            Some(Version1_3_2) => 0x6C4,
            Some(Version1_4_0) | Some(Version1_4_1) | Some(Version1_5_0) |
            Some(Version1_6_0) | Some(Version1_7_0) => 0x6BC,
            _ => 0x6CC,
        }
    }
}
