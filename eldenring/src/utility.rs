use crate::{
    event,
    mem::*,
    offsets::{
        ChainReadExt,
        code_cave::CaveOffset,
        cs_flipper_imp, damage_manager, dl_user_input_manager_impl, game_data_man, game_man,
        map_dbg_flags,
        module_offsets::{BasePointer, Data, Hook, Patch},
    },
    resources::ASM,
    utils::player_loaded_check,
};
use gubtool_core::{address::Address, slice_ops::*, sys::error::ProcResult};

pub fn quitout() -> ProcResult {
    player_loaded_check()?;
    read::<u64>(BasePointer::GameMan)
        .add_offset(game_man::QUITOUT)
        .write::<u8>(0x1)
}

pub fn get_ng_cycle() -> ProcResult<i32> {
    read::<u64>(BasePointer::GameDataMan)
        .add_offset(game_data_man::NEW_GAME)
        .read::<i32>()
}

const NG_EVENT_IDS: [u32; 8] = [50, 51, 52, 53, 54, 55, 56, 57];
pub fn set_ng_cycle(val: i32) -> ProcResult {
    read::<u64>(BasePointer::GameDataMan)
        .add_offset(game_data_man::NEW_GAME)
        .write::<i32>(val)?;

    let current_ng = get_ng_cycle()?.clamp(0, 7);
    NG_EVENT_IDS.iter().enumerate()
        .try_for_each(|(i, &id)| event::set_event(id, i == current_ng as usize))
}

pub fn trigger_new_game() -> ProcResult {
    player_loaded_check()?;
    read::<u64>(BasePointer::GameMan)
        .add_offset(game_man::start_new_game())
        .write::<u8>(0x1)
}

pub fn set_fps_cap(value: f32) -> ProcResult {
    write::<f32>(Patch::FpsCap.add_offset(0x3), 1_f32 / value)
}

pub fn get_fps_cap() -> f32 {
    read::<f32>(Patch::FpsCap.add_offset(0x3))
        .map(|val| (1_f32 / val).round())
        .unwrap_or_default()
}

pub fn set_logo_patch(state: bool) -> ProcResult {
    match state {
        false => write_bytes(Patch::NoLogo, &[0x74, 0x53]),
        true => write_bytes(Patch::NoLogo, &[0x90, 0x90]),
    }
}

pub fn is_logo_patch() -> bool {
    read::<[u8; 2]>(Patch::NoLogo)
        .map(|val| val != [0x74, 0x53])
        .unwrap_or_default()
}

pub fn set_freeze_world(state: bool) -> ProcResult {
    match state {
        false => write_bytes(Patch::PauseWorld, &[0x0F, 0x84]),
        true => write_bytes(Patch::PauseWorld, &[0x0F, 0x85]),
    }
}

pub fn is_freeze_world_on() -> bool {
    read::<[u8; 2]>(Patch::PauseWorld)
        .map(|val| val != [0x0F, 0x84])
        .unwrap_or_default()
}

pub fn mute_music(state: bool) -> ProcResult {
    match state {
        false => write_bytes(Patch::MuteMusic, &[0x0F, 0xB6, 0x48, 0x04]),
        true => write_bytes(Patch::MuteMusic, &[0x31, 0xC9, 0x90, 0x90]),
    }
}

pub fn is_music_muted() -> bool {
    read::<[u8; 4]>(Patch::MuteMusic)
        .map(|val| val != [0x0F, 0xB6, 0x48, 0x04])
        .unwrap_or_default()
}

pub fn draw_hitboxes(val: bool, is_view_b: bool) -> ProcResult {
    let offset = if is_view_b { damage_manager::HITBOXVIEW_B } else { damage_manager::HITBOXVIEW_A };
    read::<u64>(BasePointer::DamageManager)
        .add_offset(offset)
        .write::<i64>(val as i64)
}

pub fn is_hitboxes(is_view_b: bool) -> bool {
    let offset = if is_view_b { damage_manager::HITBOXVIEW_B } else { damage_manager::HITBOXVIEW_A };
    read::<u64>(BasePointer::DamageManager)
        .read_offset(offset)
        .map(|val| val != 0x0)
        .unwrap_or_default()
}

pub fn show_all_graces(val: bool) -> ProcResult {
    write::<u8>(Data::MapDbgFlags.add_offset(map_dbg_flags::SHOW_ALL_GRACES), val as u8)
}

pub fn is_show_all_graces_on() -> bool {
    read::<u8>(Data::MapDbgFlags.add_offset(map_dbg_flags::SHOW_ALL_GRACES))
        .map(|val| val != 0x0)
        .unwrap_or_default()
}

pub fn show_all_maps(val: bool) -> ProcResult {
    write::<u8>(Data::MapDbgFlags.add_offset(map_dbg_flags::SHOW_ALL_MAPS), val as u8)
}

pub fn is_show_all_maps_on() -> bool {
    read::<u8>(Data::MapDbgFlags.add_offset(map_dbg_flags::SHOW_ALL_MAPS))
        .map(|val| val != 0x0)
        .unwrap_or_default()
}

pub fn set_stutter_fix(val: bool) -> ProcResult {
    read::<u64>(BasePointer::DlUserInputManagerImpl)
        .add_offset(dl_user_input_manager_impl::STEAM_INPUT)
        .write::<u8>(val as u8)
}

pub fn is_stutter_fix() -> bool {
    read::<u64>(BasePointer::DlUserInputManagerImpl)
        .add_offset(dl_user_input_manager_impl::STEAM_INPUT)
        .read::<u8>()
        .map(|val| val != 0x0)
        .unwrap_or_default()
}

pub fn set_game_speed(val: f32) -> ProcResult {
    read::<u64>(BasePointer::CsFlipperImp)
        .add_offset(cs_flipper_imp::game_speed())
        .write::<f32>(val)
}

pub fn get_game_speed() -> f32 {
    read::<u64>(BasePointer::CsFlipperImp)
        .add_offset(cs_flipper_imp::game_speed())
        .read::<f32>()
        .unwrap_or_default()
}

pub fn set_map_anywhere(state: bool) -> ProcResult {
    match state {
        true => {
            write::<u8>(Patch::OpenMap, 0xEB)?;
            write_bytes(Patch::CloseMap, &[0x90; 3])
        }
        false => {
            write::<u8>(Patch::OpenMap, 0x74)?;
            write_bytes(Patch::CloseMap, &[0xFF, 0x50, 0x60])
        }
    }
}

pub fn is_map_anywhere() -> bool {
    read::<u8>(Patch::OpenMap)
        .map(|val| val != 0x74)
        .unwrap_or_default()
}

pub fn set_travel_anywhere(state: bool) -> ProcResult {
    match state {
        true => write_bytes(Patch::CanFastTravel, &[0xB0, 0x01, 0x90, 0x90, 0x90]),
        false => write_bytes(Patch::CanFastTravel, &[0x84, 0xC0, 0x0F, 0x94, 0xC0]),
    }
}

pub fn is_travel_anywhere() -> bool {
    read::<[u8; 5]>(Patch::CanFastTravel)
        .map(|val| val != [0x84, 0xC0, 0x0F, 0x94, 0xC0])
        .unwrap_or_default()
}


fn install_action_hook() -> ProcResult {
    let location = CaveOffset::ActionHook;

    let mut fun = ASM.get_function("action_hook");
    let mut asm = fun.take_bytes();

    write_rel_i32(&mut asm, location, fun.reloc("roll_flag"), CaveOffset::DisableRollFlag, 5)?;
    write_rel_i32(&mut asm, location, fun.reloc("jump_flag"), CaveOffset::DisableJumpFlag, 5)?;
    write_rel_i32(&mut asm, location, fun.reloc("backstep_flag"), CaveOffset::DisableBackstepFlag, 5)?;

    install_hook(&asm, location, Hook::SetRequestedAction, 5)
}

pub enum ControlFlag {
    Roll,
    Jump,
    Backstep,
}

impl ControlFlag {
    fn addr(&self) -> impl Address {
        match self {
            Self::Roll => CaveOffset::DisableRollFlag,
            Self::Jump => CaveOffset::DisableJumpFlag,
            Self::Backstep => CaveOffset::DisableBackstepFlag,
        }
    }
}

pub fn is_control_disabled(flag: ControlFlag) -> bool {
    read::<u8>(flag.addr())
        .map(|val| val != 0x0)
        .unwrap_or_default()
}

pub fn set_control(flag: ControlFlag, state: bool) -> ProcResult {
    if state {
        install_action_hook()?;
    }
    write::<u8>(flag.addr(), state as u8)
}