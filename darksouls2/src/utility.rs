use crate::{
    mem::*,
    offsets::{
        self, ChainReadExt,
        code_cave::CaveOffset,
        game_manager_imp::{
            self,
            game_data_manager_offsets::{self, clearcount_ptr_offsets},
        },
        hooks, patches,
    },
    resources::{scholar, vanilla},
};
use gubtool_core::sys::error::ProcResult;
use utils::slice_ops::*;

pub fn quitout() -> ProcResult {
    read_address(game_manager_imp::base_ptr())
        .add_offset(game_manager_imp::QUITOUT)
        .write::<u8>(0x6)
}

pub fn get_area_id() -> ProcResult<u32> {
    read::<u32>(offsets::map_id())
}

pub fn get_ng() -> ProcResult<u8> {
    read_address(game_manager_imp::base_ptr())
        .read_offset(game_manager_imp::GAME_DATA_MANAGER)
        .read_offset(game_data_manager_offsets::CLEARCOUNT_PTR)
        .add_offset(clearcount_ptr_offsets::CLEARCOUNT)
        .read::<u8>()
}

pub fn set_ng(count: u8) -> ProcResult {
    read_address(game_manager_imp::base_ptr())
        .read_offset(game_manager_imp::GAME_DATA_MANAGER)
        .read_offset(game_data_manager_offsets::CLEARCOUNT_PTR)
        .add_offset(clearcount_ptr_offsets::CLEARCOUNT)
        .write::<u8>(count)
}

pub fn trigger_ng() -> ProcResult {
    Ok(())
}

pub fn set_faster_menu(state: bool) -> ProcResult {
    let location = CaveOffset::FasterMenuHook.addr();
    match (state, is_scholar()) {
        (true, true) => {
                write_bytes(patches::menu_transition(), &[0x74, 0xEA])?;
                install_menu_hook_scholar(location)
        }
        (true, false) => {
                write_bytes(patches::menu_transition(), &[0x0F, 0x84])?;
                install_menu_hook_vanilla(location)
        }
        (false, true) => {
                write_bytes(patches::menu_transition(), &[0x75, 0xEA])?;
                write_bytes(hooks::faster_menu(), &[0x48, 0x89, 0x84, 0x24, 0x50, 0x01, 0x00, 0x00])
        }
        (false, false) => {
                write_bytes(patches::menu_transition(), &[0x0F, 0x85])?;
                write_bytes(hooks::faster_menu(), &[0x33, 0xC5, 0x89, 0x45, 0xFC])
        }
    }
}

fn install_menu_hook_scholar(location: u64) -> ProcResult {
    let mut asm = scholar::ASM.get_function("faster_menu").get_bytes();
    write_rel_i32(&mut asm, location, 22, hooks::faster_menu() + 8, 4)?;

    install_hook(&asm, location, hooks::faster_menu(), 8)
}

fn install_menu_hook_vanilla(location: u64) -> ProcResult {
    let mut asm = vanilla::ASM.get_function("faster_menu").get_bytes();
    write_rel_i32(&mut asm, location, 16, hooks::faster_menu() + 5, 4)?;

    install_hook(&asm, location, hooks::faster_menu(), 5)
}

pub fn is_faster_menu() -> ProcResult<bool> {
    if is_scholar() {
        read::<[u8; 8]>(hooks::faster_menu())
            .map(|val| val != [0x48, 0x89, 0x84, 0x24, 0x50, 0x01, 0x00, 0x00])
    } else {
        read::<[u8; 5]>(hooks::faster_menu())
            .map(|val| val != [0x33, 0xC5, 0x89, 0x45, 0xFC])
    }
}

pub fn set_credits_skip(state: bool) -> ProcResult {
    let location = CaveOffset::CreditsSkipHook.addr();
    let modify_once = CaveOffset::CreditsModifyOnceFlag.addr();
    match (state, is_scholar()) {
        (true, true) => install_credits_hook_scholar(location, modify_once),
        (true, false) => install_credits_hook_vanilla(location, modify_once),
        (false, true) => write_bytes(hooks::credits_skip(), &[0x48, 0x81, 0xEC, 0x20, 0x02, 0x00, 0x00]),
        (false, false) => write_bytes(hooks::credits_skip(), &[0x81, 0xEC, 0xFC, 0x01, 0x00, 0x00]),
    }
}

fn install_credits_hook_scholar(location: u64, modify_once: u64) -> ProcResult {
    let mut asm = scholar::ASM.get_function("credits_skip").get_bytes();

    write_rel_i32(&mut asm, location, 9, modify_once, 5)?;
    write_rel_i32(&mut asm, location, 25, modify_once, 8)?;
    write_rel_i32(&mut asm, location, 34, hooks::credits_skip() + 7, 4)?;

    write::<u8>(modify_once, 0)?;
    install_hook(&asm, location, hooks::credits_skip(), 7)
}

fn install_credits_hook_vanilla(location: u64, modify_once: u64) -> ProcResult {
    let mut asm = vanilla::ASM.get_function("credits_skip").get_bytes();

    write_to_slice::<u32>(&mut asm, 8, modify_once)?;
    write_to_slice::<u32>(&mut asm, 24, modify_once)?;
    write_rel_i32(&mut asm, location, 33, hooks::credits_skip() + 6, 4)?;

    write::<u8>(modify_once, 0)?;
    install_hook(&asm, location, hooks::credits_skip(), 6)
}

pub fn is_credits_skip() -> ProcResult<bool> {
    if is_scholar() {
        read::<[u8; 7]>(hooks::credits_skip())
            .map(|val| val != [0x48, 0x81, 0xEC, 0x20, 0x02, 0x00, 0x00])
    } else {
        read::<[u8; 6]>(hooks::credits_skip())
            .map(|val| val != [0x81, 0xEC, 0xFC, 0x01, 0x00, 0x00])
    }
}

pub fn set_disable_roll(state: bool) -> ProcResult {
    let bytes = if state {
        [0x30, 0xC0]
    } else {
        [0xB0, 0x01]
    };
    write_bytes(patches::no_roll(), &bytes)
}

pub fn is_disable_roll() -> ProcResult<bool> {
    read::<[u8; 2]>(patches::no_roll())
        .map(|val| val != [0xB0, 0x01])
}

pub fn set_disable_backstep(state: bool) -> ProcResult {
    let bytes = if state {
        [0x30, 0xC0, 0x90]
    } else {
        [0x0F, 0x95, 0xC0]
    };
    write_bytes(patches::no_backstep(), &bytes)
}

pub fn is_disable_backstep() -> ProcResult<bool> {
    read::<[u8; 3]>(patches::no_backstep())
        .map(|val| val != [0x0F, 0x95, 0xC0])
}