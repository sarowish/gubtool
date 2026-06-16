use gubtool_core::attached::module_base;

use crate::offsets::module_offsets;

pub fn give_runes() -> u64 {
    module_base() + module_offsets().functions.give_runes
}

pub fn get_player_item_quantity_by_id() -> u64 {
    module_base() + module_offsets().functions.get_player_item_quantity_by_id
}

pub fn item_spawn() -> u64 {
    module_base() + module_offsets().functions.item_spawn
}

pub fn grace_warp() -> u64 {
    module_base() + module_offsets().functions.grace_warp
}

pub fn block_warp() -> u64 {
    module_base() + module_offsets().functions.block_warp
}

pub fn get_chr_ins_by_entity_id() -> u64 {
    module_base() + module_offsets().functions.get_chr_ins_by_entity_id
}

pub fn set_event() -> u64 {
    module_base() + module_offsets().functions.set_event
}

pub fn get_event() -> u64 {
    module_base() + module_offsets().functions.get_event
}

pub fn external_event_temp_ctor() -> u64 {
    module_base() + module_offsets().functions.external_event_temp_ctor
}

pub fn execute_talk_command() -> u64 {
    module_base() + module_offsets().functions.execute_talk_command
}

pub fn emevd_switch() -> u64 {
    module_base() + module_offsets().functions.emevd_switch
}

pub fn emk_event_ins_ctor() -> u64 {
    module_base() + module_offsets().functions.emk_event_ins_ctor
}

pub fn set_speffect() -> u64 {
    module_base() + module_offsets().functions.set_speffect
}

pub fn remove_speffect() -> u64 {
    module_base() + module_offsets().functions.remove_speffect
}
