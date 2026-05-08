use crate::{
    core::common::{rel_i32, write_to_slice},
    ds2::{
        mem::*,
        offsets::{code_cave, functions, game_manager_imp, hooks},
        resources::{scholar, vanilla},
        utils::is_scholar,
    },
};
use anyhow::Result;

pub fn quitout() -> Result<()> {
    read::<u64>(game_manager_imp::base())
        .and_then(|addr| write::<u8>(addr + game_manager_imp::quitout(), 6))
}

pub fn set_faster_menu(state: bool) -> Result<()> {
    match state {
        true => {
            if is_scholar() {
                install_menu_hook_scholar()
            } else {
                install_menu_hook_vanilla()
            }
        }
        false => {
            if is_scholar() {
                write_bytes(hooks::faster_menu(), &[0x48, 0x89, 0x84, 0x24, 0x50, 0x01, 0x00, 0x00])
            } else {
                write_bytes(hooks::faster_menu(), &[0x33, 0xC5, 0x89, 0x45, 0xFC])
            }
        }
    }
}

fn install_menu_hook_scholar() -> Result<()> {
    let location = code_cave::base() + code_cave::FASTER_MENU_HOOK;
    let mut asm = scholar::ASM.get_function("faster_menu").bytes.clone();
    write_to_slice::<i32>(&mut asm, 22, rel_i32(hooks::faster_menu() + 8, location + 26)?)?;

    let mut hookbytes = [0xE9, 0x00, 0x00, 0x00, 0x00, 0x90, 0x90, 0x90];
    write_to_slice::<i32>(&mut hookbytes, 1, rel_i32(location, hooks::faster_menu() + 5)?)?;

    write_bytes(location, &asm)?;
    write_bytes(hooks::faster_menu(), &hookbytes)
}

fn install_menu_hook_vanilla() -> Result<()> {
    let location = code_cave::base() + code_cave::FASTER_MENU_HOOK;
    let mut asm = vanilla::ASM.get_function("faster_menu").bytes.clone();
    write_to_slice::<i32>(&mut asm, 16, rel_i32(hooks::faster_menu() + 5, location + 20)?)?;

    let mut hookbytes = [0xE9, 0x00, 0x00, 0x00, 0x00];
    write_to_slice::<i32>(&mut hookbytes, 1, rel_i32(location, hooks::faster_menu() + 5)?)?;

    write_bytes(location, &asm)?;
    write_bytes(hooks::faster_menu(), &hookbytes)
}

pub fn is_faster_menu() -> Result<bool> {
    if is_scholar() {
        read::<[u8; 8]>(hooks::faster_menu())
            .map(|val| val != [0x48, 0x89, 0x84, 0x24, 0x50, 0x01, 0x00, 0x00])
    } else {
        read::<[u8; 5]>(hooks::faster_menu())
            .map(|val| val != [0x33, 0xC5, 0x89, 0x45, 0xFC])
    }
}

pub fn set_ivory_skip(state: bool) -> Result<()> {
    match state {
        true => {
            if is_scholar() {
                install_ivory_hooks_scholar()
            } else {
                install_ivory_hooks_vanilla()
            }
        }
        false => {
            if is_scholar() {
                write_bytes(functions::set_event(), &[0x48, 0x89, 0x74, 0x24, 0x10])?;
                write_bytes(hooks::set_shared_flag(), &[0x44, 0x88, 0x84, 0x08, 0xA1, 0x03, 0x00, 0x00])
            } else {
                write_bytes(functions::set_event(), &[0x55, 0x8B, 0xEC, 0x83, 0xEC, 0x08])?;
                write_bytes(hooks::set_shared_flag(), &[0x88, 0x94, 0x08, 0xA1, 0x02, 0x00, 0x00])
            }
        }
    }
}

fn install_ivory_hooks_scholar() -> Result<()> {
    let skip_location = code_cave::base() + code_cave::IVORY_SKIP_HOOK;
    let knights_location = code_cave::base() + code_cave::IVORY_KNIGHTS_HOOK;

    let mut skip_asm = scholar::ASM.get_function("ivory_skip").bytes.clone();
    write_to_slice::<u64>(&mut skip_asm, 94, functions::get_map_entity_with_area_id_and_obj_id())?;
    write_to_slice::<u64>(&mut skip_asm, 104, functions::get_map_obj_state_act_component())?;
    write_to_slice::<u64>(&mut skip_asm, 114, functions::set_event())?;
    write_to_slice::<i32>(&mut skip_asm, 215, rel_i32(functions::set_event() + 5, skip_location + 219)?)?;

    let mut knights_asm = scholar::ASM.get_function("ivory_knights").bytes.clone();
    write_to_slice::<i32>(&mut knights_asm, 32, rel_i32(hooks::set_shared_flag() + 8, knights_location + 36)?)?;

    let mut skip_hookbytes = [0xE9, 0x00, 0x00, 0x00, 0x00];
    write_to_slice::<i32>(&mut skip_hookbytes, 1, rel_i32(skip_location, functions::set_event() + 5)?)?;

    let mut knights_hookbytes = [0xE9, 0x00, 0x00, 0x00, 0x00, 0x90, 0x90, 0x90];
    write_to_slice::<i32>(&mut knights_hookbytes, 1, rel_i32(knights_location, hooks::set_shared_flag() + 5)?)?;

    write_bytes(skip_location, &skip_asm)?;
    write_bytes(knights_location, &knights_asm)?;
    write_bytes(functions::set_event(), &skip_hookbytes)?;
    write_bytes(hooks::set_shared_flag(), &knights_hookbytes)
}

fn install_ivory_hooks_vanilla() -> Result<()> {
    let skip_location = code_cave::base() + code_cave::IVORY_SKIP_HOOK;
    let knights_location = code_cave::base() + code_cave::IVORY_KNIGHTS_HOOK;

    let mut skip_asm = vanilla::ASM.get_function("ivory_skip").bytes.clone();
    write_to_slice::<u32>(&mut skip_asm, 38, functions::set_event())?;
    write_to_slice::<u32>(&mut skip_asm, 73, functions::get_map_entity_with_area_id_and_obj_id())?;
    write_to_slice::<u32>(&mut skip_asm, 80, functions::get_map_obj_state_act_component())?;
    write_to_slice::<i32>(&mut skip_asm, 162, rel_i32(functions::set_event() + 6, skip_location + 167)?)?;

    let mut knights_asm = vanilla::ASM.get_function("ivory_knights").bytes.clone();
    write_to_slice::<i32>(&mut knights_asm, 28, rel_i32(hooks::set_shared_flag() + 7, knights_location + 33)?)?;

    let mut skip_hookbytes = [0xE9, 0x00, 0x00, 0x00, 0x00, 0x90];
    write_to_slice::<i32>(&mut skip_hookbytes, 1, rel_i32(skip_location, functions::set_event() + 5)?)?;

    let mut knights_hookbytes = [0xE9, 0x00, 0x00, 0x00, 0x00, 0x90, 0x90];
    write_to_slice::<i32>(&mut knights_hookbytes, 1, rel_i32(knights_location, hooks::set_shared_flag() + 5)?)?;

    write_bytes(skip_location, &skip_asm)?;
    write_bytes(knights_location, &knights_asm)?;
    write_bytes(functions::set_event(), &skip_hookbytes)?;
    write_bytes(hooks::set_shared_flag(), &knights_hookbytes)
}

pub fn is_ivory_skip() -> Result<bool> {
    if is_scholar() {
        read::<[u8; 5]>(functions::set_event())
            .map(|val| val != [0x48, 0x89, 0x74, 0x24, 0x10])
    } else {
        read::<[u8; 6]>(functions::set_event())
            .map(|val| val != [0x55, 0x8B, 0xEC, 0x83, 0xEC, 0x08])
    }
}

pub fn set_credits_skip(state: bool) -> Result<()> {
    match state {
        true => {
            if is_scholar() {
                install_credits_hook_scholar()
            } else {
                install_credits_hook_vanilla()
            }
        }
        false => {
            if is_scholar() {
                write_bytes(hooks::credits_skip(), &[0x48, 0x81, 0xEC, 0x20, 0x02, 0x00, 0x00])
            } else {
                write_bytes(hooks::credits_skip(), &[0x81, 0xEC, 0xFC, 0x01, 0x00, 0x00])
            }
        }
    }
}

fn install_credits_hook_scholar() -> Result<()> {
    let location = code_cave::base() + code_cave::CREDITS_SKIP_HOOK;
    let mut asm = scholar::ASM.get_function("credits_skip").bytes.clone();
    let modify_once = code_cave::base() + code_cave::CREDITS_MODIFY_ONCE_FLAG;
    write::<u8>(modify_once, 0)?;

    write_to_slice::<i32>(&mut asm, 9, rel_i32(modify_once, location + 14)?)?;
    write_to_slice::<i32>(&mut asm, 25, rel_i32(modify_once, location + 33)?)?;
    write_to_slice::<i32>(&mut asm, 34, rel_i32(hooks::credits_skip() + 7, location + 38)?)?;

    let mut hookbytes = [0xE9, 0x00, 0x00, 0x00, 0x00, 0x90, 0x90];
    write_to_slice::<i32>(&mut hookbytes, 1, rel_i32(location, hooks::credits_skip() + 5)?)?;

    write_bytes(location, &asm)?;
    write_bytes(hooks::credits_skip(), &hookbytes)
}

fn install_credits_hook_vanilla() -> Result<()> {
    let location = code_cave::base() + code_cave::CREDITS_SKIP_HOOK;
    let mut asm = vanilla::ASM.get_function("credits_skip").bytes.clone();
    let modify_once = code_cave::base() + code_cave::CREDITS_MODIFY_ONCE_FLAG;
    write::<u8>(modify_once, 0)?;

    write_to_slice::<u32>(&mut asm, 8, modify_once)?;
    write_to_slice::<u32>(&mut asm, 24, modify_once)?;
    write_to_slice::<i32>(&mut asm, 33, rel_i32(hooks::credits_skip() + 6, location + 37)?)?;

    let mut hookbytes = [0xE9, 0x00, 0x00, 0x00, 0x00, 0x90];
    write_to_slice::<i32>(&mut hookbytes, 1, rel_i32(location, hooks::credits_skip() + 5)?)?;

    write_bytes(location, &asm)?;
    write_bytes(hooks::credits_skip(), &hookbytes)
}

pub fn is_credits_skip() -> Result<bool> {
    if is_scholar() {
        read::<[u8; 7]>(hooks::credits_skip())
            .map(|val| val != [0x48, 0x81, 0xEC, 0x20, 0x02, 0x00, 0x00])
    } else {
        read::<[u8; 6]>(hooks::credits_skip())
            .map(|val| val != [0x81, 0xEC, 0xFC, 0x01, 0x00, 0x00])
    }
}