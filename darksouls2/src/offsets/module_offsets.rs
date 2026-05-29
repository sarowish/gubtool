use crate::{
    mem::is_scholar,
    resources::{scholar_patterns, vanilla_patterns, versions_module_offsets},
};
use anyhow::Result;
use engine::{aob_scanner, attached::version, game_version::DarkSouls2Version::*};

#[derive(Debug)]
pub struct ModuleOffsets {
    pub base_ptrs: BasePointers,
    pub functions: Functions,
    pub hooks: Hooks,
    pub patches: Patches,
    pub data: Data,
    pub external_fn_ptrs: ExternalFunctionPointers,
}

#[derive(Debug)]
pub struct BasePointers {
    pub game_manager_imp: u64,
    pub katana_main_app: u64,
}

#[derive(Debug)]
pub struct Functions {
    pub give_souls: u64,
    pub warp: u64,
    pub item_spawn: u64,
    pub build_item_dialogue: u64,
    pub show_item_dialogue: u64,
    pub current_item_quantity_check: u64,
    pub set_event: u64,
    pub get_map_entity_with_area_id_and_obj_id: u64,
    pub get_state_act_component: u64,
    pub make_sound: u64,
}

#[derive(Debug)]
pub struct Hooks {
    pub set_shared_flag: u64,
    pub locked_target_pointer: u64,
    pub credits_skip: u64,
    pub faster_menu: u64,
    pub event_log: u64,
    pub player_no_damage: u64,
    pub infinite_poise: u64,
}

#[derive(Debug)]
pub struct Patches {
    pub infinite_stamina: u64,
    pub infinite_consumables: u64,
    pub infinite_durability: u64,
    pub infinite_casts: u64,
    pub no_soul_gain: u64,
    pub no_hollowing: u64,
    pub no_soul_loss: u64,
    pub player_hidden: u64,
    pub player_silent: u64,
    pub menu_transition: u64,
}

#[derive(Debug)]
pub struct Data {
    pub map_id: u64,
}

#[derive(Debug)]
pub struct ExternalFunctionPointers {
    pub kernel32_create_thread: u64,
    pub kernel32_close_handle: u64,
    pub kernel32_sleep: u64,
    pub kernel32_load_library_w: u64,
}

pub fn module_offsets() -> &'static ModuleOffsets {
    match version() {
        Some(Vanilla1_0_7) => &versions_module_offsets::VANILLA_1_0_7,
        Some(Vanilla1_0_10) => &versions_module_offsets::VANILLA_1_0_10,
        Some(Vanilla1_0_11) => &versions_module_offsets::VANILLA_1_0_11,
        Some(Vanilla1_0_12) => &versions_module_offsets::VANILLA_1_0_12,
        Some(Scholar1_0_1) => &versions_module_offsets::SCHOLAR_1_0_1,
        Some(Scholar1_0_2) => &versions_module_offsets::SCHOLAR_1_0_2,
        Some(Scholar1_0_3) => &versions_module_offsets::SCHOLAR_1_0_3,
        _ => &versions_module_offsets::SCHOLAR_1_0_3,
    }
}

pub fn scan() -> Result<ModuleOffsets> {
    match is_scholar() {
        true => scan_scholar(),
        false => scan_vanilla(),
    }
}

fn scan_scholar() -> Result<ModuleOffsets> {
    Ok(ModuleOffsets {
        base_ptrs: BasePointers {
            game_manager_imp: aob_scanner::scan(scholar_patterns::GAME_MANAGER_IMP)?,
            katana_main_app: aob_scanner::scan(scholar_patterns::KATANA_MAIN_APP)?,
        },
        functions: Functions {
            give_souls: aob_scanner::scan(scholar_patterns::GIVE_SOULS)?,
            warp: aob_scanner::scan(scholar_patterns::WARP)?,
            item_spawn: aob_scanner::scan(scholar_patterns::ITEM_SPAWN)?,
            build_item_dialogue: aob_scanner::scan(scholar_patterns::BUILD_ITEM_DIALOGUE)?,
            show_item_dialogue: aob_scanner::scan(scholar_patterns::SHOW_ITEM_DIALOGUE)?,
            current_item_quantity_check: aob_scanner::scan(scholar_patterns::CURRENT_ITEM_QUANTITY_CHECK)?,
            set_event: aob_scanner::scan(scholar_patterns::SET_EVENT)?,
            get_map_entity_with_area_id_and_obj_id: aob_scanner::scan(scholar_patterns::GET_MAP_ENTITY_WITH_AREA_ID_AND_OBJ_ID)?,
            get_state_act_component: aob_scanner::scan(scholar_patterns::GET_MAP_OBJ_STATE_ACT_COMPONENT)?,
            make_sound: aob_scanner::scan(scholar_patterns::MAKE_SOUND)?,
        },
        hooks: Hooks {
            set_shared_flag: aob_scanner::scan(scholar_patterns::SET_SHARED_FLAG)?,
            locked_target_pointer: aob_scanner::scan(scholar_patterns::LOCKED_TARGET_POINTER)?,
            credits_skip: aob_scanner::scan(scholar_patterns::CREDITS_SKIP)?,
            faster_menu: aob_scanner::scan(scholar_patterns::FASTER_MENU)?,
            event_log: aob_scanner::scan(scholar_patterns::EVENT_LOG)?,
            player_no_damage: aob_scanner::scan(scholar_patterns::PLAYER_NO_DAMAGE)?,
            infinite_poise: aob_scanner::scan(scholar_patterns::INFINITE_POISE)?,
        },
        patches: Patches {
            infinite_stamina: aob_scanner::scan(scholar_patterns::INFINITE_STAMINA)?,
            infinite_consumables: aob_scanner::scan(scholar_patterns::INFINITE_CONSUMABLES)?,
            infinite_durability: aob_scanner::scan(scholar_patterns::INFINITE_DURABILITY)?,
            infinite_casts: aob_scanner::scan(scholar_patterns::INFINITE_CASTS)?,
            no_soul_gain: aob_scanner::scan(scholar_patterns::NO_SOUL_GAIN)?,
            no_hollowing: aob_scanner::scan(scholar_patterns::NO_HOLLOWING)?,
            no_soul_loss: aob_scanner::scan(scholar_patterns::NO_SOUL_LOSS)?,
            player_hidden: aob_scanner::scan(scholar_patterns::PLAYER_HIDDEN)?,
            player_silent: aob_scanner::scan(scholar_patterns::PLAYER_SILENT)?,
            menu_transition: aob_scanner::scan(scholar_patterns::MENU_TRANSITION)?,
        },
        data: Data {
            map_id: aob_scanner::scan(scholar_patterns::MAP_ID)?
        },
        external_fn_ptrs: ExternalFunctionPointers {
            kernel32_create_thread: aob_scanner::scan(scholar_patterns::KERNEL32_CREATE_THREAD)?,
            kernel32_close_handle: aob_scanner::scan(scholar_patterns::KERNEL32_CLOSE_HANDLE)?,
            kernel32_sleep: aob_scanner::scan(scholar_patterns::KERNEL32_SLEEP)?,
            kernel32_load_library_w: aob_scanner::scan(scholar_patterns::KERNEL32_LOAD_LIBRARY_W)?,
        },
    })
}

fn scan_vanilla() -> Result<ModuleOffsets> {
    Ok(ModuleOffsets {
        base_ptrs: BasePointers {
            game_manager_imp: aob_scanner::scan(vanilla_patterns::GAME_MANAGER_IMP)?,
            katana_main_app: aob_scanner::scan(vanilla_patterns::KATANA_MAIN_APP)?,
        },
        functions: Functions {
            give_souls: aob_scanner::scan(vanilla_patterns::GIVE_SOULS)?,
            warp: aob_scanner::scan(vanilla_patterns::WARP)?,
            item_spawn: aob_scanner::scan(vanilla_patterns::ITEM_SPAWN)?,
            build_item_dialogue: aob_scanner::scan(vanilla_patterns::BUILD_ITEM_DIALOGUE)?,
            show_item_dialogue: aob_scanner::scan(vanilla_patterns::SHOW_ITEM_DIALOGUE)?,
            current_item_quantity_check: aob_scanner::scan(vanilla_patterns::CURRENT_ITEM_QUANTITY_CHECK)?,
            set_event: aob_scanner::scan(vanilla_patterns::SET_EVENT)?,
            get_map_entity_with_area_id_and_obj_id: aob_scanner::scan(vanilla_patterns::GET_MAP_ENTITY_WITH_AREA_ID_AND_OBJ_ID)?,
            get_state_act_component: aob_scanner::scan(vanilla_patterns::GET_STATE_ACT_COMPONENT)?,
            make_sound: aob_scanner::scan(vanilla_patterns::MAKE_SOUND)?,
        },
        hooks: Hooks {
            set_shared_flag: aob_scanner::scan(vanilla_patterns::SET_SHARED_FLAG)?,
            locked_target_pointer: aob_scanner::scan(vanilla_patterns::LOCKED_TARGET_POINTER)?,
            credits_skip: aob_scanner::scan(vanilla_patterns::CREDITS_SKIP).unwrap_or_default(),
            faster_menu: aob_scanner::scan(vanilla_patterns::FASTER_MENU)?,
            event_log: aob_scanner::scan(vanilla_patterns::EVENT_LOG)?,
            player_no_damage: aob_scanner::scan(vanilla_patterns::PLAYER_NO_DAMAGE)?,
            infinite_poise: aob_scanner::scan(vanilla_patterns::INFINITE_POISE)?,
        },
        patches: Patches {
            infinite_stamina: aob_scanner::scan(vanilla_patterns::INFINITE_STAMINA)?,
            infinite_consumables: aob_scanner::scan(vanilla_patterns::INFINITE_CONSUMABLES)?,
            infinite_durability: aob_scanner::scan(vanilla_patterns::INFINITE_DURABILITY)?,
            infinite_casts: aob_scanner::scan(vanilla_patterns::INFINITE_CASTS)?,
            no_soul_gain: aob_scanner::scan(vanilla_patterns::NO_SOUL_GAIN)?,
            no_hollowing: aob_scanner::scan(vanilla_patterns::NO_HOLLOWING)?,
            no_soul_loss: aob_scanner::scan(vanilla_patterns::NO_SOUL_LOSS)?,
            player_hidden: aob_scanner::scan(vanilla_patterns::PLAYER_HIDDEN)?,
            player_silent: aob_scanner::scan(vanilla_patterns::PLAYER_SILENT).unwrap_or_default(),
            menu_transition: aob_scanner::scan(vanilla_patterns::MENU_TRANSITION)?,
        },
        data: Data {
            map_id: aob_scanner::scan(vanilla_patterns::MAP_ID)?
        },
        external_fn_ptrs: ExternalFunctionPointers {
            kernel32_create_thread: aob_scanner::scan(vanilla_patterns::KERNEL32_CREATE_THREAD)?,
            kernel32_close_handle: aob_scanner::scan(vanilla_patterns::KERNEL32_CLOSE_HANDLE)?,
            kernel32_sleep: aob_scanner::scan(vanilla_patterns::KERNEL32_SLEEP)?,
            kernel32_load_library_w: aob_scanner::scan(vanilla_patterns::KERNEL32_LOAD_LIBRARY_W)?,
        },
    })
}
