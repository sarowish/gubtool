use crate::{
    chr_ctrl::{ChrCtrl, ChrCtrlExt},
    mem::*,
    offsets::{
        code_cave::CaveOffset, functions,
        game_manager_imp::{self, px_world_offsets},
        hooks, patches,
    },
    resources::{scholar, vanilla},
};
use anyhow::Result;
use shared::slice_ops::*;

pub fn player_ctrl() -> ChrCtrl {
    read_address(game_manager_imp::base_ptr())
        .and_then(|addr| read_address(addr + game_manager_imp::player_ctrl()))
}

pub fn give_souls(amount: i32) -> Result<()> {
    let location = CaveOffset::GiveSoulsAsm.addr();
    match is_scholar() {
        true => give_souls_scholar(location, amount),
        false => give_souls_vanilla(location, amount),
    }
}

fn give_souls_scholar(location: u64, amount: i32) -> Result<()> {
    let mut asm = scholar::ASM.get_function("give_souls").get_bytes();
    write_to_slice::<u64>(&mut asm, 2, player_ctrl().stats_pointer()?)?;
    write_to_slice::<i64>(&mut asm, 12, amount)?;
    write_to_slice::<u64>(&mut asm, 22, functions::give_souls())?;
    append_flag_setter(location, &mut asm)?;

    write_bytes(location, &asm)?;
    run_thread(location)
}

fn give_souls_vanilla(location: u64, amount: i32) -> Result<()> {
    let mut asm = vanilla::ASM.get_function("give_souls").get_bytes();
    write_to_slice::<i32>(&mut asm, 1, amount)?;
    write_to_slice::<u32>(&mut asm, 7, player_ctrl().stats_pointer()?)?;
    write_to_slice::<u32>(&mut asm, 12, functions::give_souls())?;
    append_flag_setter(location, &mut asm)?;

    write_bytes(location, &asm)?;
    run_thread(location)
}

pub fn player_position() -> Result<[f32; 16]> {
    let mut pointers = vec![
        game_manager_imp::base_ptr(),
        game_manager_imp::px_world(),
    ];
    pointers.extend_from_slice(&px_world_offsets::player_coords_chain());
    let pointer = follow_pointers(&pointers, false)?;
    read::<[f32; 16]>(pointer)
}

pub fn set_infinite_poise(state: bool) -> Result<()> {
    let location = CaveOffset::InfinitePoiseHook.addr();
    match (state, is_scholar()) {
        (true, true) => install_infinite_poise_scholar(location),
        (true, false) => install_infinite_poise_vanilla(location),
        (false, true) => write_bytes(hooks::infinite_poise(), &[0x39, 0x9D, 0xEC, 0x05, 0x00, 0x00]),
        (false, false) => write_bytes(hooks::infinite_poise(), &[0x83, 0xBB, 0xEC, 0x05, 0x00, 0x00, 0x00]),
    }
}

pub fn is_infinite_poise() -> Result<bool> {
    if is_scholar() {
        read::<[u8; 6]>(hooks::infinite_poise())
            .map(|val| val != [0x39, 0x9D, 0xEC, 0x05, 0x00, 0x00])
    } else {
        read::<[u8; 7]>(hooks::infinite_poise())
            .map(|val| val != [0x83, 0xBB, 0xEC, 0x05, 0x00, 0x00, 0x00])
    }
}

fn install_infinite_poise_scholar(location: u64) -> Result<()> {
    let fun = scholar::ASM.get_function("infinite_poise_hook");
    let mut asm = fun.get_bytes();

    write_to_slice::<u64>(&mut asm, fun.reloc("game_man"), game_manager_imp::base_ptr())?;
    write_rel_i32(&mut asm, location, fun.reloc("hook_loc"), hooks::infinite_poise() + 6, 4)?;

    install_hook(&asm, location, hooks::infinite_poise(), 6)
}

fn install_infinite_poise_vanilla(location: u64) -> Result<()> {
    let fun = vanilla::ASM.get_function("infinite_poise_hook");
    let mut asm = fun.get_bytes();

    write_to_slice::<u32>(&mut asm, fun.reloc("game_man"), game_manager_imp::base_ptr())?;
    write_rel_i32(&mut asm, location, fun.reloc("hook_loc"), hooks::infinite_poise() + 7, 4)?;

    install_hook(&asm, location, hooks::infinite_poise(), 7)
}

pub fn set_no_damage(state: bool) -> Result<()> {
    let location = CaveOffset::PlayerNoDamageHook.addr();
    match (state, is_scholar()) {
        (true, true) => install_no_damage_scholar(location),
        (true, false) => install_no_damage_vanilla(location),
        (false, true) => write_bytes(hooks::player_no_damage(), &[0x89, 0x83, 0x68, 0x01, 0x00, 0x00]),
        (false, false) => write_bytes(hooks::player_no_damage(), &[0x89, 0x8E, 0xFC, 0x00, 0x00, 0x00]),
    }
}

pub fn is_no_damage() -> Result<bool> {
    if is_scholar() {
        read::<[u8; 6]>(hooks::player_no_damage())
            .map(|val| val != [0x89, 0x83, 0x68, 0x01, 0x00, 0x00])
    } else {
        read::<[u8; 6]>(hooks::player_no_damage())
            .map(|val| val != [0x89, 0x8E, 0xFC, 0x00, 0x00, 0x00])
    }
}

fn install_no_damage_scholar(location: u64) -> Result<()> {
    let fun = scholar::ASM.get_function("player_no_damage");
    let mut asm = fun.get_bytes();

    write_to_slice::<u64>(&mut asm, fun.reloc("game_man"), game_manager_imp::base_ptr())?;
    write_rel_i32(&mut asm, location, fun.reloc("hook_loc"), hooks::player_no_damage() + 6, 4)?;

    install_hook(&asm, location, hooks::player_no_damage(), 6)
}

fn install_no_damage_vanilla(location: u64) -> Result<()> {
    let fun = vanilla::ASM.get_function("player_no_damage");
    let mut asm = fun.get_bytes();

    write_to_slice::<u32>(&mut asm, fun.reloc("game_man"), game_manager_imp::base_ptr())?;
    write_rel_i32(&mut asm, location, fun.reloc("hook_loc"), hooks::player_no_damage() + 6, 4)?;

    install_hook(&asm, location, hooks::player_no_damage(), 6)
}

pub fn set_infinite_consumables(state: bool) -> Result<()> {
    let bytes: &[u8] = match (state, is_scholar()) {
        (true, true) | (true, false) => &[0x90; 4],
        (false, true) => &[0x66, 0x29, 0x73, 0x20],
        (false, false) => &[0x66, 0x29, 0x5E, 0x18],
    };
    write_bytes(patches::infinite_consumables(), bytes)
}

pub fn is_infinite_consumables() -> Result<bool> {
    read::<[u8; 4]> (patches::infinite_consumables())
        .map(|val| val == [0x90; 4])
}

pub fn set_no_hollowing(state: bool) -> Result<()> {
    let bytes: &[u8] = match (state, is_scholar()) {
        (true, true) | (true, false) => &[0x90; 6],
        (false, true) => &[0x88, 0x81, 0xAC, 0x01, 0x00, 0x00],
        (false, false) => &[0x88, 0x91, 0xA8, 0x01, 0x00, 0x00],
    };
    write_bytes(patches::no_hollowing(), bytes)
}

pub fn is_no_hollowing() -> Result<bool> {
    if is_scholar() {
        read::<[u8; 6]> (patches::no_hollowing())
            .map(|val| val != [0x88, 0x81, 0xAC, 0x01, 0x00, 0x00])
    } else {
        read::<[u8; 6]> (patches::no_hollowing())
            .map(|val| val != [0x88, 0x91, 0xA8, 0x01, 0x00, 0x00])
    }
}

pub fn set_infinite_durability(state: bool) -> Result<()> {
    let bytes: &[u8] = match (state, is_scholar()) {
        (true, true) => &[0x90; 9],
        (true, false) => &[0x90; 5],
        (false, true) => &[0xF3, 0x0F, 0x11, 0xB4, 0xC3, 0x94, 0x00, 0x00, 0x00],
        (false, false) => &[0xF3, 0x0F, 0x11, 0x47, 0x6C],
    };
    write_bytes(patches::infinite_durability(), bytes)
}

pub fn is_infinite_durability() -> Result<bool> {
    if is_scholar() {
        read::<[u8; 9]> (patches::infinite_durability())
            .map(|val| val != [0xF3, 0x0F, 0x11, 0xB4, 0xC3, 0x94, 0x00, 0x00, 0x00])
    } else {
        read::<[u8; 5]> (patches::infinite_durability())
            .map(|val| val != [0xF3, 0x0F, 0x11, 0x47, 0x6C])
    }
}

pub fn set_no_soul_gain(state: bool) -> Result<()> {
    let bytes: &[u8] = match (state, is_scholar()) {
        (true, true) | (true, false) => &[0x90; 5],
        (false, true) => &[0xE8, 0x71, 0x01, 0x00, 0x00],
        (false, false) => &[0xE8, 0xF7, 0xF5, 0xFF, 0xFF],
    };
    write_bytes(patches::no_soul_gain(), bytes)
}

pub fn is_no_soul_gain() -> Result<bool> {
    read::<[u8; 5]> (patches::no_soul_gain())
        .map(|val| val == [0x90; 5])
}

pub fn set_no_soul_loss(state: bool) -> Result<()> {
    let bytes: &[u8] = match (state, is_scholar()) {
        (true, true) => &[0x90; 6],
        (true, false) => &[0x90; 10],
        (false, true) => &[0x89, 0x90, 0xEC, 0x00, 0x00, 0x00],
        (false, false) => &[0xC7, 0x80, 0xE8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    };
    write_bytes(patches::no_soul_loss(), bytes)
}

pub fn is_no_soul_loss() -> Result<bool> {
    if is_scholar() {
        read::<[u8; 6]> (patches::no_soul_loss())
            .map(|val| val != [0x89, 0x90, 0xEC, 0x00, 0x00, 0x00])
    } else {
        read::<[u8; 10]> (patches::no_soul_loss())
            .map(|val| val != [0xC7, 0x80, 0xE8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
    }
}

pub fn set_infinite_stamina(state: bool) -> Result<()> {
    match state {
        true => write::<u8>(patches::infinite_stamina(), 0x82),
        false => write::<u8>(patches::infinite_stamina(), 0x83),
    }
}

pub fn is_infinite_stamina() -> Result<bool> {
    read::<u8>(patches::infinite_stamina())
        .map(|val| val != 0x83)
}

pub fn set_hidden(state: bool) -> Result<()> {
    match state {
        true => write::<u8>(patches::player_hidden(), 0x85),
        false => write::<u8>(patches::player_hidden(), 0x84),
    }
}

pub fn is_hidden() -> Result<bool> {
    read::<u8>(patches::player_hidden())
        .map(|val| val != 0x84)
}

pub fn set_silent(state: bool) -> Result<()> {
    let bytes: Vec<u8> = match (state, is_scholar()) {
        (true, true) => vec![0x90; 5],
        (true, false) => vec![0x90, 0xB0, 0x01, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90],
        (false, true) => {
            let mut bytes = vec![0xE8; 5];
            write_rel_i32(&mut bytes, patches::player_silent(), 1, functions::make_sound(), 4)?;
            bytes
        }
        (false, false) => {
            let mut bytes = vec![0xE8; 5];
            write_rel_i32(&mut bytes, patches::player_silent(), 1, functions::make_sound(), 4)?;
            bytes
        }
    };
    write_bytes(patches::player_silent() + 1, &bytes)
}

pub fn is_silent() -> Result<bool> {
    if is_scholar() {
        read::<[u8; 5]> (patches::player_silent())
            .map(|val| val == [0x90; 5])
    } else {
        read::<[u8; 5]>(patches::player_silent() + 1).map(|val| {
            val == [0x83, 0xC4, 0x0C, 0xB0, 0x01]
        })
    }
}