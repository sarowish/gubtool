use crate::offsets::module_offsets::module_offsets;
use gubtool_core::attached::module_base;

pub fn give_souls() -> u64 {
    module_base() + module_offsets().functions.give_souls
}

pub fn warp() -> u64 {
    module_base() + module_offsets().functions.warp
}

pub fn item_spawn() -> u64 {
    module_base() + module_offsets().functions.item_spawn
}

pub fn build_item_dialogue() -> u64 {
    module_base() + module_offsets().functions.build_item_dialogue
}

pub fn show_item_dialogue() -> u64 {
    module_base() + module_offsets().functions.show_item_dialogue
}

pub fn current_item_quantity_check() -> u64 {
    module_base() + module_offsets().functions.current_item_quantity_check
}

pub fn set_event() -> u64 {
    module_base() + module_offsets().functions.set_event
}

pub fn get_map_entity_with_area_id_and_obj_id() -> u64 {
    module_base() + module_offsets().functions.get_map_entity_with_area_id_and_obj_id
}

pub fn get_state_act_component() -> u64 {
    module_base() + module_offsets().functions.get_state_act_component
}

pub fn make_sound() -> u64 {
    module_base() + module_offsets().functions.make_sound
}

pub fn bonfire_rest() -> u64 {
    module_base() + module_offsets().functions.bonfire_rest
}

pub fn bonfire_unlock() -> u64 {
    module_base() + module_offsets().functions.bonfire_unlock
}

pub fn open_menu() -> u64 {
    module_base() + module_offsets().functions.open_menu
}

pub fn menu_chr_state() -> u64 {
    module_base() + module_offsets().functions.menu_chr_state
}