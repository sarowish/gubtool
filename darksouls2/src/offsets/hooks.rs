use crate::offsets::module_offsets::module_offsets;
use engine::attached::module_base;

pub fn set_shared_flag() -> u64 {
    module_base() + module_offsets().hooks.set_shared_flag
}

pub fn locked_target_pointer() -> u64 {
    module_base() + module_offsets().hooks.locked_target_pointer
}

pub fn credits_skip() -> u64 {
    module_base() + module_offsets().hooks.credits_skip
}

pub fn faster_menu() -> u64 {
    module_base() + module_offsets().hooks.faster_menu
}

pub fn event_log() -> u64 {
    module_base() + module_offsets().hooks.event_log
}

pub fn infinite_poise() -> u64 {
    module_base() + module_offsets().hooks.infinite_poise
}

pub fn player_no_damage() -> u64 {
    module_base() + module_offsets().hooks.player_no_damage
}
