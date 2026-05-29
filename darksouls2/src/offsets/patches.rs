use crate::offsets::module_offsets::module_offsets;
use engine::attached::module_base;

pub fn infinite_stamina() -> u64 {
    module_base() + module_offsets().patches.infinite_stamina
}

pub fn infinite_consumables() -> u64 {
    module_base() + module_offsets().patches.infinite_consumables
}

pub fn infinite_durability() -> u64 {
    module_base() + module_offsets().patches.infinite_durability
}

pub fn infinite_casts() -> u64 {
    module_base() + module_offsets().patches.infinite_casts
}

pub fn no_soul_gain() -> u64 {
    module_base() + module_offsets().patches.no_soul_gain
}

pub fn no_hollowing() -> u64 {
    module_base() + module_offsets().patches.no_hollowing
}

pub fn no_soul_loss() -> u64 {
    module_base() + module_offsets().patches.no_soul_loss
}

pub fn player_hidden() -> u64 {
    module_base() + module_offsets().patches.player_hidden
}

pub fn player_silent() -> u64 {
    module_base() + module_offsets().patches.player_silent
}

pub fn menu_transition() -> u64 {
    module_base() + module_offsets().patches.menu_transition
}
