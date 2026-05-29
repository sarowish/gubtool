use crate::offsets::module_offsets;
use engine::attached::module_base;

pub fn no_logo() -> u64 {
    module_base() + module_offsets().patches.no_logo
}

pub fn fps_cap() -> u64 {
    module_base() + module_offsets().patches.fps_cap
}

pub fn mute_music() -> u64 {
    module_base() + module_offsets().patches.mute_music
}

pub fn pause_world() -> u64 {
    module_base() + module_offsets().patches.pause_world
}

pub fn torrent_disabled_underworld() -> u64 {
    module_base() + module_offsets().patches.torrent_disabled_underworld
}

pub fn whistle_disabled() -> u64 {
    module_base() + module_offsets().patches.whistle_disabled
}

pub fn open_map() -> u64 {
    module_base() + module_offsets().patches.open_map
}

pub fn close_map() -> u64 {
    module_base() + module_offsets().patches.close_map
}

pub fn can_fast_travel() -> u64 {
    module_base() + module_offsets().patches.can_fast_travel
}