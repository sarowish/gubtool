use crate::{
    core::common::{write_rel_i32, write_to_slice},
    ds2::{
        mem::*,
        offsets::{self, code_cave, game_manager_imp, hooks},
        resources::{scholar, vanilla},
        utils::is_scholar,
    },
};
use anyhow::Result;

pub fn quitout() -> Result<()> {
    read::<u64>(game_manager_imp::base())
        .and_then(|addr| write::<u8>(addr + game_manager_imp::quitout(), 6))
}

pub fn get_area_id() -> Result<u32> {
    read::<u32>(offsets::area_id())
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
    write_rel_i32(&mut asm, location, 22, hooks::faster_menu() + 8, 4)?;

    write_bytes(location, &asm)?;
    install_hook(location, hooks::faster_menu(), 8)
}

fn install_menu_hook_vanilla() -> Result<()> {
    let location = code_cave::base() + code_cave::FASTER_MENU_HOOK;
    let mut asm = vanilla::ASM.get_function("faster_menu").bytes.clone();
    write_rel_i32(&mut asm, location, 16, hooks::faster_menu() + 5, 4)?;

    write_bytes(location, &asm)?;
    install_hook(location, hooks::faster_menu(), 5)
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

    write_rel_i32(&mut asm, location, 9, modify_once, 5)?;
    write_rel_i32(&mut asm, location, 25, modify_once, 8)?;
    write_rel_i32(&mut asm, location, 34, hooks::credits_skip() + 7, 4)?;

    write_bytes(location, &asm)?;
    install_hook(location, hooks::credits_skip(), 7)
}

fn install_credits_hook_vanilla() -> Result<()> {
    let location = code_cave::base() + code_cave::CREDITS_SKIP_HOOK;
    let mut asm = vanilla::ASM.get_function("credits_skip").bytes.clone();
    let modify_once = code_cave::base() + code_cave::CREDITS_MODIFY_ONCE_FLAG;
    write::<u8>(modify_once, 0)?;

    write_to_slice::<u32>(&mut asm, 8, modify_once)?;
    write_to_slice::<u32>(&mut asm, 24, modify_once)?;
    write_rel_i32(&mut asm, location, 33, hooks::credits_skip() + 6, 4)?;

    write_bytes(location, &asm)?;
    install_hook(location, hooks::credits_skip(), 6)
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