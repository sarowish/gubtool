use crate::{mem::is_scholar, offsets::module_offsets::module_offsets};

pub fn base_ptr() -> u64 {
    engine::attached::module_base() + module_offsets().base_ptrs.game_manager_imp
}

pub fn event_manager() -> u64 {
    match is_scholar() {
        true => 0x70,
        false => 0x44,
    }
}

pub fn player_ctrl() -> u64 {
    match is_scholar() {
        true => 0xD0,
        false => 0x74,
    }
}

pub fn loading_flag() -> u64 {
    match is_scholar() {
        true => 0x24BC,
        false => 0xDFC,
    }
}

pub mod event_manager_offsets {
    pub fn event_flag_manager() -> u64 {
        match crate::mem::is_scholar() {
            true => 0x20,
            false => 0x10,
        }
    }

    pub fn event_warp_manager() -> u64 {
        match crate::mem::is_scholar() {
            true => 0x70,
            false => 0x38,
        }
    }
}

pub fn quitout() -> u64 {
    match is_scholar() {
        true => 0x24B1,
        false => 0xDF1,
    }
}

pub fn px_world() -> u64 {
    match is_scholar() {
        true => 0x660,
        false => 0x280,
    }
}

pub mod px_world_offsets {
    pub fn player_coords_chain() -> [u64; 5] {
        match crate::mem::is_scholar() {
            true => [0x18, 0x1F8, 0x18, 0x8, 0x1A0],
            false => [0xC, 0x168, 0xC, 0x4, 0x120],
        }
    }
}