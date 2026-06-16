use crate::offsets::module_offsets;
use gubtool_core::attached::module_base;

pub fn locked_target_pointer() -> u64 {
    module_base() + module_offsets().hooks.locked_target_pointer
}

pub fn target_no_stagger() -> u64 {
    module_base() + module_offsets().hooks.target_no_stagger
}

pub fn player_no_grab() -> u64 {
    module_base() + module_offsets().hooks.player_no_grab
}

pub fn player_infinite_poise() -> u64 {
    module_base() + module_offsets().hooks.player_infinite_poise
}

pub fn warp_coord_write() -> u64 {
    module_base() + module_offsets().hooks.warp_coord_write
}

pub fn warp_angle_write() -> u64 {
    module_base() + module_offsets().hooks.warp_angle_write
}

pub fn get_force_act_idx() -> u64 {
    module_base() + module_offsets().hooks.get_force_act_idx
}

pub fn set_action_requested() -> u64 {
    module_base() + module_offsets().hooks.set_requested_action
}
