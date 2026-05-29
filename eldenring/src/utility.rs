use crate::{
    event,
    mem::*,
    offsets::{
        code_cave::CaveOffset, cs_flipper_imp, damage_manager, dl_user_input_manager_impl, game_data_man,
        game_man, hooks, map_dbg_flags, patches,
    },
    resources::ASM,
    utils::character_loaded_check,
};
use anyhow::Result;
use shared::slice_ops::write_rel_i32;

pub fn quitout() -> Result<()> {
    character_loaded_check()?;
    read::<u64>(game_man::base_ptr())
        .and_then(|addr| write::<u8>(addr + 0x10, 0x1))
}

pub fn get_ng_cycle() -> Result<i32> {
    read::<u64>(game_data_man::base_ptr())
        .and_then(|addr| read::<i32>(addr + game_data_man::NEW_GAME))
}

const NG_EVENT_IDS: [u32; 8] = [50, 51, 52, 53, 54, 55, 56, 57];
pub fn set_ng_cycle(val: i32) -> Result<()> {
    read::<u64>(game_data_man::base_ptr())
        .and_then(|addr| write::<i32>(addr + game_data_man::NEW_GAME, val))?;

    let current_ng = get_ng_cycle()?.clamp(0, 7);
    NG_EVENT_IDS.iter().enumerate()
        .try_for_each(|(i, &id)| event::set_event(id, i == current_ng as usize))
}

pub fn trigger_new_game() -> Result<()> {
    character_loaded_check()?;
    read::<u64>(game_man::base_ptr())
        .and_then(|addr| write::<u8>(addr + game_man::start_new_game(), 0x1))
}

pub fn set_fps_cap(value: f32) -> Result<()> {
    write::<f32>(patches::fps_cap() + 0x3, 1_f32 / value)
}

pub fn get_fps_cap() -> Result<f32> {
    read::<f32>(patches::fps_cap() + 0x3)
        .map(|val| (1_f32 / val).round())
}

pub fn set_logo_patch(state: bool) -> Result<()> {
    match state {
        false => write_bytes(patches::no_logo(), &[0x74, 0x53]),
        true => write_bytes(patches::no_logo(), &[0x90, 0x90]),
    }
}

pub fn is_logo_patch() -> Result<bool> {
    read::<[u8; 2]>(patches::no_logo())
        .map(|val| val == [0x90, 0x90])
}

pub fn set_freeze_world(state: bool) -> Result<()> {
    match state {
        false => write_bytes(patches::pause_world(), &[0x0F, 0x84]),
        true => write_bytes(patches::pause_world(), &[0x0F, 0x85]),
    }
}

pub fn is_freeze_world_on() -> Result<bool> {
    read::<[u8; 2]>(patches::pause_world())
        .map(|val| val == [0x0F, 0x85])
}

pub fn mute_music(state: bool) -> Result<()> {
    match state {
        false => write_bytes(patches::mute_music(), &[0x0F, 0xB6, 0x48, 0x04]),
        true => write_bytes(patches::mute_music(), &[0x31, 0xC9, 0x90, 0x90]),
    }
}

pub fn is_music_muted() -> Result<bool> {
    read::<[u8; 4]>(patches::mute_music())
        .map(|val| val == [0x31, 0xC9, 0x90, 0x90])
}

pub fn draw_hitboxes(val: bool, is_view_b: bool) -> Result<()> {
    let offset = if is_view_b { damage_manager::HITBOXVIEW_B } else { damage_manager::HITBOXVIEW_A };
    read::<u64>(damage_manager::base_ptr())
        .map(|addr| write::<i64>(addr + offset, val as i64))?
}

pub fn is_hitboxes(is_view_b: bool) -> Result<bool> {
    let offset = if is_view_b { damage_manager::HITBOXVIEW_B } else { damage_manager::HITBOXVIEW_A };
    read::<u64>(damage_manager::base_ptr())
        .and_then(|addr| read::<i64>(addr + offset))
        .map(|val| val == 1)
}

pub fn show_all_graces(val: bool) -> Result<()> {
    write::<u8>(map_dbg_flags::base() + map_dbg_flags::SHOW_ALL_GRACES, val as u8)
}

pub fn is_show_all_graces_on() -> Result<bool> {
    read::<u8>(map_dbg_flags::base() + map_dbg_flags::SHOW_ALL_GRACES)
        .map(|val| val == 1)
}

pub fn show_all_maps(val: bool) -> Result<()> {
    write::<u8>(map_dbg_flags::base() + map_dbg_flags::SHOW_ALL_MAPS, val as u8)
}

pub fn is_show_all_maps_on() -> Result<bool> {
    read::<u8>(map_dbg_flags::base() + map_dbg_flags::SHOW_ALL_MAPS)
        .map(|val| val == 1)
}

pub fn set_stutter_fix(val: bool) -> Result<()> {
    read::<u64>(dl_user_input_manager_impl::base_ptr())
        .and_then(|addr| write::<u8>(addr + dl_user_input_manager_impl::STEAM_INPUT, val as u8))
}

pub fn is_stutter_fix_on() -> Result<bool> {
    read::<u64>(dl_user_input_manager_impl::base_ptr())
        .and_then(|addr| read::<u8>(addr + dl_user_input_manager_impl::STEAM_INPUT))
        .map(|val| val == 1)
}

pub fn set_game_speed(val: f32) -> Result<()> {
    read::<u64>(cs_flipper_imp::base_ptr())
        .and_then(|addr| write::<f32>(addr + cs_flipper_imp::game_speed(), val))
}

pub fn get_game_speed() -> Result<f32> {
    read::<u64>(cs_flipper_imp::base_ptr())
        .and_then(|addr| read::<f32>(addr + cs_flipper_imp::game_speed()))
}

pub fn set_map_anywhere(state: bool) -> Result<()> {
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

pub fn is_map_anywhere() -> Result<bool> {
    read::<u8>(patches::open_map())
        .map(|val| val != 0x74)
}

pub fn set_travel_anywhere(state: bool) -> Result<()> {
    match state {
        true => write_bytes(patches::can_fast_travel(), &[0xB0, 0x01, 0x90, 0x90, 0x90]),
        false => write_bytes(patches::can_fast_travel(), &[0x84, 0xC0, 0x0F, 0x94, 0xC0]),
    }
}

pub fn is_travel_anywhere() -> Result<bool> {
    read::<[u8; 5]>(patches::can_fast_travel())
        .map(|val| val != [0x84, 0xC0, 0x0F, 0x94, 0xC0])
}


fn install_action_hook() -> Result<()> {
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

pub fn set_disable_roll(state: bool) -> Result<()> {
    if state {
        install_action_hook()?;
    }
    write::<u8>(CaveOffset::DisableRollFlag.addr(), state as u8)
}

pub fn is_roll_disabled() -> Result<bool> {
    read::<u8>(CaveOffset::DisableRollFlag.addr())
        .map(|val| val != 0x0)
}

pub fn set_disable_jump(state: bool) -> Result<()> {
    if state {
        install_action_hook()?;
    }
    write::<u8>(CaveOffset::DisableJumpFlag.addr(), state as u8)
}

pub fn is_jump_disabled() -> Result<bool> {
    read::<u8>(CaveOffset::DisableJumpFlag.addr())
        .map(|val| val != 0x0)
}

pub fn set_disable_backstep(state: bool) -> Result<()> {
    if state {
        install_action_hook()?;
    }
    write::<u8>(CaveOffset::DisableBackstepFlag.addr(), state as u8)
}

pub fn is_backstep_disabled() -> Result<bool> {
    read::<u8>(CaveOffset::DisableBackstepFlag.addr())
        .map(|val| val != 0x0)
}

/*
pub fn set_music(val: u8) -> Result<()> {
    read::<u64>(game_data_man::base())
        .and_then(|addr| read::<u64>(addr + game_data_man::OPTIONS))
        .and_then(|addr| write::<u8>(addr + game_data_man::options_offsets::MUSIC, val))
}
*/