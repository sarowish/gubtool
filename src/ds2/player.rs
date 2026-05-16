use anyhow::Result;

use crate::{
    core::common::write_to_slice,
    ds2::{
        chr_ctrl::{ChrCtrl, ChrCtrlExt},
        mem::*,
        offsets::{
            code_cave, functions,
            game_manager_imp::{self, px_world_offsets},
        },
        resources::{scholar, vanilla},
        utils::is_scholar,
    },
};

pub fn player_ctrl() -> ChrCtrl {
    read_address(game_manager_imp::base())
        .and_then(|addr| read_address(addr + game_manager_imp::player_ctrl()))
}

pub fn give_souls(amount: i32) -> Result<()> {
    if is_scholar() {
        give_souls_scholar(amount)
    } else {
        give_souls_vanilla(amount)
    }
}

fn give_souls_scholar(amount: i32) -> Result<()> {
    let location = code_cave::base() + code_cave::SOULS_GIVE_ASM;

    let mut asm = scholar::ASM.get_function("give_souls").bytes.clone();
    write_to_slice::<u64>(&mut asm, 2, player_ctrl().stats_pointer()?)?;
    write_to_slice::<i64>(&mut asm, 12, amount)?;
    write_to_slice::<u64>(&mut asm, 22, functions::give_souls())?;
    append_flag_setter(location, &mut asm)?;

    write_bytes(location, &asm)?;
    run_thread(location)
}

fn give_souls_vanilla(amount: i32) -> Result<()> {
    let location = code_cave::base() + code_cave::SOULS_GIVE_ASM;

    let mut asm = vanilla::ASM.get_function("give_souls").bytes.clone();
    write_to_slice::<i32>(&mut asm, 1, amount)?;
    write_to_slice::<u32>(&mut asm, 7, player_ctrl().stats_pointer()?)?;
    write_to_slice::<u32>(&mut asm, 12, functions::give_souls())?;
    append_flag_setter(location, &mut asm)?;

    write_bytes(location, &asm)?;
    run_thread(location)
}

pub fn player_position() -> Result<[f32; 16]> {
    let mut pointers = vec![
        game_manager_imp::base(),
        game_manager_imp::px_world(),
    ];
    pointers.extend_from_slice(&px_world_offsets::player_coords_chain());
    let pointer = follow_pointers(&pointers, false)?;
    read::<[f32; 16]>(pointer)
}