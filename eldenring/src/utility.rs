use crate::{
    event,
    mem::*,
    offsets::{
        ChainReadExt, code_cave::CaveOffset, cs_flipper_imp, damage_manager, dl_user_input_manager_impl, game_data_man, game_man, hooks, map_dbg_flags, patches
    },
    resources::ASM,
    utils::character_loaded_check,
};
use gubtool_core::sys::error::ProcResult;
use utils::slice_ops::write_rel_i32;

pub fn quitout() -> ProcResult {
    character_loaded_check()?;
    read::<u64>(game_man::base_ptr())
        .add_offset(game_man::QUITOUT)
        .write::<u8>(0x1)
}

pub fn get_ng_cycle() -> ProcResult<i32> {
    read::<u64>(game_data_man::base_ptr())
        .add_offset(game_data_man::NEW_GAME)
        .read::<i32>()
}

const NG_EVENT_IDS: [u32; 8] = [50, 51, 52, 53, 54, 55, 56, 57];
pub fn set_ng_cycle(val: i32) -> ProcResult {
    read::<u64>(game_data_man::base_ptr())
        .add_offset(game_data_man::NEW_GAME)
        .write::<i32>(val)?;

    let current_ng = get_ng_cycle()?.clamp(0, 7);
    NG_EVENT_IDS.iter().enumerate()
        .try_for_each(|(i, &id)| event::set_event(id, i == current_ng as usize))
}

pub fn trigger_new_game() -> ProcResult {
    character_loaded_check()?;
    read::<u64>(game_man::base_ptr())
        .add_offset(game_man::start_new_game())
        .write::<u8>(0x1)
}

pub fn set_fps_cap(value: f32) -> ProcResult {
    write::<f32>(patches::fps_cap() + 0x3, 1_f32 / value)
}

pub fn get_fps_cap() -> ProcResult<f32> {
    read::<f32>(patches::fps_cap() + 0x3)
        .map(|val| (1_f32 / val).round())
}

pub fn set_logo_patch(state: bool) -> ProcResult {
    match state {
        false => write_bytes(patches::no_logo(), &[0x74, 0x53]),
        true => write_bytes(patches::no_logo(), &[0x90, 0x90]),
    }
}

pub fn is_logo_patch() -> ProcResult<bool> {
    read::<[u8; 2]>(patches::no_logo())
        .map(|val| val == [0x90, 0x90])
}

pub fn set_freeze_world(state: bool) -> ProcResult {
    match state {
        false => write_bytes(patches::pause_world(), &[0x0F, 0x84]),
        true => write_bytes(patches::pause_world(), &[0x0F, 0x85]),
    }
}

pub fn is_freeze_world_on() -> ProcResult<bool> {
    read::<[u8; 2]>(patches::pause_world())
        .map(|val| val == [0x0F, 0x85])
}

pub fn mute_music(state: bool) -> ProcResult {
    match state {
        false => write_bytes(patches::mute_music(), &[0x0F, 0xB6, 0x48, 0x04]),
        true => write_bytes(patches::mute_music(), &[0x31, 0xC9, 0x90, 0x90]),
    }
}

pub fn is_music_muted() -> ProcResult<bool> {
    read::<[u8; 4]>(patches::mute_music())
        .map(|val| val == [0x31, 0xC9, 0x90, 0x90])
}

pub fn draw_hitboxes(val: bool, is_view_b: bool) -> ProcResult {
    let offset = if is_view_b { damage_manager::HITBOXVIEW_B } else { damage_manager::HITBOXVIEW_A };
    read::<u64>(damage_manager::base_ptr())
        .add_offset(offset)
        .write::<i64>(val as i64)
}

pub fn is_hitboxes(is_view_b: bool) -> ProcResult<bool> {
    let offset = if is_view_b { damage_manager::HITBOXVIEW_B } else { damage_manager::HITBOXVIEW_A };
    read::<u64>(damage_manager::base_ptr())
        .read_offset(offset)
        .map(|val| val != 0x0)
}

pub fn show_all_graces(val: bool) -> ProcResult {
    write::<u8>(map_dbg_flags::base() + map_dbg_flags::SHOW_ALL_GRACES, val as u8)
}

pub fn is_show_all_graces_on() -> ProcResult<bool> {
    read::<u8>(map_dbg_flags::base() + map_dbg_flags::SHOW_ALL_GRACES)
        .map(|val| val != 0x0)
}

pub fn show_all_maps(val: bool) -> ProcResult {
    write::<u8>(map_dbg_flags::base() + map_dbg_flags::SHOW_ALL_MAPS, val as u8)
}

pub fn is_show_all_maps_on() -> ProcResult<bool> {
    read::<u8>(map_dbg_flags::base() + map_dbg_flags::SHOW_ALL_MAPS)
        .map(|val| val == 1)
}

pub fn set_stutter_fix(val: bool) -> ProcResult {
    read::<u64>(dl_user_input_manager_impl::base_ptr())
        .add_offset(dl_user_input_manager_impl::STEAM_INPUT)
        .write::<u8>(val as u8)
}

pub fn is_stutter_fix() -> ProcResult<bool> {
    read::<u64>(dl_user_input_manager_impl::base_ptr())
        .add_offset(dl_user_input_manager_impl::STEAM_INPUT)
        .read::<u8>()
        .map(|val| val != 0x0)
}

pub fn set_game_speed(val: f32) -> ProcResult {
    read::<u64>(cs_flipper_imp::base_ptr())
        .add_offset(cs_flipper_imp::game_speed())
        .write::<f32>(val)
}

pub fn get_game_speed() -> ProcResult<f32> {
    read::<u64>(cs_flipper_imp::base_ptr())
        .add_offset(cs_flipper_imp::game_speed())
        .read::<f32>()
}

pub fn set_map_anywhere(state: bool) -> ProcResult {
    match state {
        true => {
            write::<u8>(patches::open_map(), 0xEB)?;
            write_bytes(patches::close_map(), &[0x90; 3])
        }
        false => {
            write::<u8>(patches::open_map(), 0x74)?;
            write_bytes(patches::close_map(), &[0xFF, 0x50, 0x60])
        }
    }
}

pub fn is_map_anywhere() -> ProcResult<bool> {
    read::<u8>(patches::open_map())
        .map(|val| val != 0x74)
}

pub fn set_travel_anywhere(state: bool) -> ProcResult {
    match state {
        true => write_bytes(patches::can_fast_travel(), &[0xB0, 0x01, 0x90, 0x90, 0x90]),
        false => write_bytes(patches::can_fast_travel(), &[0x84, 0xC0, 0x0F, 0x94, 0xC0]),
    }
}

pub fn is_travel_anywhere() -> ProcResult<bool> {
    read::<[u8; 5]>(patches::can_fast_travel())
        .map(|val| val != [0x84, 0xC0, 0x0F, 0x94, 0xC0])
}


fn install_action_hook() -> ProcResult {
    let location = CaveOffset::ActionHook.addr();
    let roll_flag = CaveOffset::DisableRollFlag.addr();
    let jump_flag = CaveOffset::DisableJumpFlag.addr();
    let backstep_flag = CaveOffset::DisableBackstepFlag.addr();

    let fun = ASM.get_function("action_hook");
    let mut asm = fun.get_bytes();

    write_rel_i32(&mut asm, location, fun.reloc("roll_flag"), roll_flag, 5)?;
    write_rel_i32(&mut asm, location, fun.reloc("jump_flag"), jump_flag, 5)?;
    write_rel_i32(&mut asm, location, fun.reloc("backstep_flag"), backstep_flag, 5)?;

    install_hook(&asm, location, hooks::set_action_requested(), 5)
}

pub enum ControlFlag {
    Roll,
    Jump,
    Backstep,
}

impl ControlFlag {
    fn addr(&self) -> u64 {
        match self {
            Self::Roll => CaveOffset::DisableRollFlag.addr(),
            Self::Jump => CaveOffset::DisableJumpFlag.addr(),
            Self::Backstep => CaveOffset::DisableBackstepFlag.addr(),
        }
    }
}

pub fn is_control_disabled(flag: ControlFlag) -> ProcResult<bool> {
    read::<u8>(flag.addr())
        .map(|val| val != 0x0)
}

pub fn set_control(flag: ControlFlag, state: bool) -> ProcResult {
    if state {
        install_action_hook()?;
    }
    write::<u8>(flag.addr(), state as u8)
}