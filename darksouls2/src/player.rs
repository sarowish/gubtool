use crate::{
    chr_ctrl::{ChrCtrl, ChrCtrlExt},
    mem::*,
    offsets::{ChainReadExt, code_cave::CaveOffset, functions, game_manager_imp, hooks, patches},
    resources::{scholar, vanilla},
};
use gubtool_core::{attached::version, game_version::DarkSouls2Version, sys::error::ProcResult};
use utils::slice_ops::*;

pub fn player_ctrl() -> ChrCtrl {
    read_address(game_manager_imp::base_ptr())
        .read_offset(game_manager_imp::PLAYER_CTRL)
}

pub fn give_souls(amount: i32) -> ProcResult {
    let asm = if is_scholar() {
        let mut asm = scholar::ASM.get_function("give_souls").get_bytes();
        write_to_slice::<u64>(&mut asm, 2, player_ctrl().stats_pointer()?)?;
        write_to_slice::<i64>(&mut asm, 12, amount)?;
        write_to_slice::<u64>(&mut asm, 22, functions::give_souls())?;
        asm
    } else {
        let mut asm = vanilla::ASM.get_function("give_souls").get_bytes();
        write_to_slice::<i32>(&mut asm, 1, amount)?;
        write_to_slice::<u32>(&mut asm, 7, player_ctrl().stats_pointer()?)?;
        write_to_slice::<u32>(&mut asm, 12, functions::give_souls())?;
        asm
    };
    spawn_thread_join(CaveOffset::GiveSoulsAsm.addr(), asm)
}

pub fn player_position() -> ProcResult<[f32; 16]> {
    let pointer = follow_pointers(&game_manager_imp::player_coords_chain(), false)?;
    read::<[f32; 16]>(pointer)
}

pub fn set_infinite_poise(state: bool) -> ProcResult {
    let location = CaveOffset::InfinitePoiseHook.addr();
    match (state, is_scholar()) {
        (true, true) => install_infinite_poise_scholar(location),
        (true, false) => install_infinite_poise_vanilla(location),
        (false, true) => write_bytes(hooks::infinite_poise(), &[0x39, 0x9D, 0xEC, 0x05, 0x00, 0x00]),
        (false, false) => write_bytes(hooks::infinite_poise(), &[0x83, 0xBB, 0xEC, 0x05, 0x00, 0x00, 0x00]),
    }
}

pub fn is_infinite_poise() -> ProcResult<bool> {
    if is_scholar() {
        read::<[u8; 6]>(hooks::infinite_poise())
            .map(|val| val != [0x39, 0x9D, 0xEC, 0x05, 0x00, 0x00])
    } else {
        read::<[u8; 7]>(hooks::infinite_poise())
            .map(|val| val != [0x83, 0xBB, 0xEC, 0x05, 0x00, 0x00, 0x00])
    }
}

fn install_infinite_poise_scholar(location: u64) -> ProcResult {
    let fun = scholar::ASM.get_function("infinite_poise_hook");
    let mut asm = fun.get_bytes();

    write_to_slice::<u64>(&mut asm, fun.reloc("game_man"), game_manager_imp::base_ptr())?;
    write_rel_i32(&mut asm, location, fun.reloc("hook_loc"), hooks::infinite_poise() + 6, 4)?;

    install_hook(&asm, location, hooks::infinite_poise(), 6)
}

fn install_infinite_poise_vanilla(location: u64) -> ProcResult {
    let fun = vanilla::ASM.get_function("infinite_poise_hook");
    let mut asm = fun.get_bytes();

    write_to_slice::<u32>(&mut asm, fun.reloc("game_man"), game_manager_imp::base_ptr())?;
    write_rel_i32(&mut asm, location, fun.reloc("hook_loc"), hooks::infinite_poise() + 7, 4)?;

    install_hook(&asm, location, hooks::infinite_poise(), 7)
}

pub fn set_no_damage(state: bool) -> ProcResult {
    let location = CaveOffset::PlayerNoDamageHook.addr();
    match (state, is_scholar()) {
        (true, true) => install_no_damage_scholar(location),
        (true, false) => install_no_damage_vanilla(location),
        (false, true) => write_bytes(hooks::player_no_damage(), &[0x89, 0x83, 0x68, 0x01, 0x00, 0x00]),
        (false, false) => write_bytes(hooks::player_no_damage(), &[0x89, 0x8E, 0xFC, 0x00, 0x00, 0x00]),
    }
}

pub fn is_no_damage() -> ProcResult<bool> {
    if is_scholar() {
        read::<[u8; 6]>(hooks::player_no_damage())
            .map(|val| val != [0x89, 0x83, 0x68, 0x01, 0x00, 0x00])
    } else {
        read::<[u8; 6]>(hooks::player_no_damage())
            .map(|val| val != [0x89, 0x8E, 0xFC, 0x00, 0x00, 0x00])
    }
}

fn install_no_damage_scholar(location: u64) -> ProcResult {
    let fun = scholar::ASM.get_function("player_no_damage");
    let mut asm = fun.get_bytes();

    write_to_slice::<u64>(&mut asm, fun.reloc("game_man"), game_manager_imp::base_ptr())?;
    write_rel_i32(&mut asm, location, fun.reloc("hook_loc"), hooks::player_no_damage() + 6, 4)?;

    install_hook(&asm, location, hooks::player_no_damage(), 6)
}

fn install_no_damage_vanilla(location: u64) -> ProcResult {
    let fun = vanilla::ASM.get_function("player_no_damage");
    let mut asm = fun.get_bytes();

    write_to_slice::<u32>(&mut asm, fun.reloc("game_man"), game_manager_imp::base_ptr())?;
    write_rel_i32(&mut asm, location, fun.reloc("hook_loc"), hooks::player_no_damage() + 6, 4)?;

    install_hook(&asm, location, hooks::player_no_damage(), 6)
}

pub fn set_infinite_consumables(state: bool) -> ProcResult {
    let bytes: &[u8] = match (state, is_scholar()) {
        (true, true) | (true, false) => &[0x90; 4],
        (false, true) => &[0x66, 0x29, 0x73, 0x20],
        (false, false) => &[0x66, 0x29, 0x5E, 0x18],
    };
    write_bytes(patches::infinite_consumables(), bytes)
}

pub fn is_infinite_consumables() -> ProcResult<bool> {
    read::<[u8; 4]> (patches::infinite_consumables())
        .map(|val| val == [0x90; 4])
}

pub fn set_no_hollowing(state: bool) -> ProcResult {
    let bytes: &[u8] = match (state, is_scholar()) {
        (true, true) | (true, false) => &[0x90; 6],
        (false, true) => &[0x88, 0x81, 0xAC, 0x01, 0x00, 0x00],
        (false, false) => &[0x88, 0x91, 0xA8, 0x01, 0x00, 0x00],
    };
    write_bytes(patches::no_hollowing(), bytes)
}

pub fn is_no_hollowing() -> ProcResult<bool> {
    if is_scholar() {
        read::<[u8; 6]> (patches::no_hollowing())
            .map(|val| val != [0x88, 0x81, 0xAC, 0x01, 0x00, 0x00])
    } else {
        read::<[u8; 6]> (patches::no_hollowing())
            .map(|val| val != [0x88, 0x91, 0xA8, 0x01, 0x00, 0x00])
    }
}

pub fn set_infinite_durability(state: bool) -> ProcResult {
    let bytes: &[u8] = match (state, is_scholar()) {
        (true, true) => &[0x90; 9],
        (true, false) => &[0x90; 5],
        (false, true) => &[0xF3, 0x0F, 0x11, 0xB4, 0xC3, 0x94, 0x00, 0x00, 0x00],
        (false, false) => &[0xF3, 0x0F, 0x11, 0x47, 0x6C],
    };
    write_bytes(patches::infinite_durability(), bytes)
}

pub fn is_infinite_durability() -> ProcResult<bool> {
    if is_scholar() {
        read::<[u8; 9]> (patches::infinite_durability())
            .map(|val| val != [0xF3, 0x0F, 0x11, 0xB4, 0xC3, 0x94, 0x00, 0x00, 0x00])
    } else {
        read::<[u8; 5]> (patches::infinite_durability())
            .map(|val| val != [0xF3, 0x0F, 0x11, 0x47, 0x6C])
    }
}

pub fn set_no_soul_gain(state: bool) -> ProcResult {
    let bytes: &[u8] = match (state, is_scholar()) {
        (true, true) | (true, false) => &[0x90; 5],
        (false, true) => &[0xE8, 0x71, 0x01, 0x00, 0x00],
        (false, false) => &[0xE8, 0xF7, 0xF5, 0xFF, 0xFF],
    };
    write_bytes(patches::no_soul_gain(), bytes)
}

pub fn is_no_soul_gain() -> ProcResult<bool> {
    read::<[u8; 5]> (patches::no_soul_gain())
        .map(|val| val == [0x90; 5])
}

pub fn set_no_soul_loss(state: bool) -> ProcResult {
    let bytes: &[u8] = match (state, is_scholar()) {
        (true, true) => &[0x90; 6],
        (true, false) => &[0x90; 10],
        (false, true) => &[0x89, 0x90, 0xEC, 0x00, 0x00, 0x00],
        (false, false) => &[0xC7, 0x80, 0xE8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    };
    write_bytes(patches::no_soul_loss(), bytes)
}

pub fn is_no_soul_loss() -> ProcResult<bool> {
    if is_scholar() {
        read::<[u8; 6]> (patches::no_soul_loss())
            .map(|val| val != [0x89, 0x90, 0xEC, 0x00, 0x00, 0x00])
    } else {
        read::<[u8; 10]> (patches::no_soul_loss())
            .map(|val| val != [0xC7, 0x80, 0xE8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
    }
}

pub fn set_infinite_stamina(state: bool) -> ProcResult {
    match state {
        true => write::<u8>(patches::infinite_stamina(), 0x82),
        false => write::<u8>(patches::infinite_stamina(), 0x83),
    }
}

pub fn is_infinite_stamina() -> ProcResult<bool> {
    read::<u8>(patches::infinite_stamina())
        .map(|val| val != 0x83)
}

pub fn set_hidden(state: bool) -> ProcResult {
    match state {
        true => write::<u8>(patches::player_hidden(), 0x85),
        false => write::<u8>(patches::player_hidden(), 0x84),
    }
}

pub fn is_hidden() -> ProcResult<bool> {
    read::<u8>(patches::player_hidden())
        .map(|val| val != 0x84)
}

pub fn set_silent(state: bool) -> ProcResult {
    match is_scholar() {
        true => {
            if state {
                write_bytes(patches::player_silent(), &[0x90; 5])
            } else {
                let mut bytes = vec![0xE8; 5];
                write_rel_i32(&mut bytes, patches::player_silent(), 1, functions::make_sound(), 4)?;
                write_bytes(patches::player_silent(), &bytes)
            }
        },
        false => {
            let push_op_neg_offset = match version() {
                Some(DarkSouls2Version::Vanilla1_0_12) => 4,
                _ => 1,
            };
            if state {
                write_bytes(patches::player_silent(), &[0x90; 15])?;
                write::<u8>(patches::player_silent() - push_op_neg_offset, 0x90)
            } else {
                let mut bytes = match version() {
                    Some(DarkSouls2Version::Vanilla1_0_12) => vec![
                        0xF3, 0x0F, 0x11, 0x04, 0x24, 0x51, 0x52, 0x53, 0x8B, 0xCF,
                        0xE8, 0x00, 0x00, 0x00, 0x00,
                    ],
                    _ => vec![
                        0xF3, 0x0F, 0x11, 0x04, 0x24, 0x52, 0x50, 0x53, 0x8B, 0xCF,
                        0xE8, 0x00, 0x00, 0x00, 0x00,
                    ],
                };
                write_rel_i32(&mut bytes, patches::player_silent(), 11, functions::make_sound(), 4)?;
                write_bytes(patches::player_silent(), &bytes)?;
                write::<u8>(patches::player_silent() - push_op_neg_offset, 0x51)
            }
        }
    }
}

pub fn is_silent() -> ProcResult<bool> {
    if is_scholar() {
        read::<[u8; 5]> (patches::player_silent())
            .map(|val| val == [0x90; 5])
    } else {
        read::<[u8; 15]>(patches::player_silent()).map(|val| {
            val == [0x90; 15]
        })
    }
}