use crate::{
    game_state::is_loading_screen,
    mem::*,
    offsets::{
        code_cave, functions,
        game_manager_imp::{self, event_manager_offsets, hk_hardware_info},
        hooks,
    },
    resources::{bonfires::Bonfire, bosses::Boss, scholar, vanilla},
    utils::character_loaded_check,
};
use anyhow::{Result, anyhow};
use shared::slice_ops::*;
use std::{thread, time::Duration};

impl Boss {
    pub fn warp(&self) -> Result<()> {
        let _handle = TRAVEL_MUTEX.try_lock()
            .map_err(|_| anyhow!("Is already travelling"))?;

        character_loaded_check()?;

        let event_warp_entity = read_address(game_manager_imp::base())
            .and_then(|addr| read_address(addr + game_manager_imp::event_manager()))
            .and_then(|addr| read_address(addr + event_manager_offsets::warp_event_entity()))?;

        if let Some(event_object_id) = self.event_object_id {
            event_warp(self.bonfire_id, event_object_id, event_warp_entity)?
        } else {
            bonfire_warp(self.bonfire_id, event_warp_entity)?
        }
        write_coords_hook(self.coordinates)?;
        wait_for_loaded(true)
    }
}

impl Bonfire {
    pub fn warp(&self) -> Result<()> {
        let _handle = TRAVEL_MUTEX.try_lock()
            .map_err(|_| anyhow!("Is already travelling"))?;

        character_loaded_check()?;

        let event_warp_entity = read_address(game_manager_imp::base())
            .and_then(|addr| read_address(addr + game_manager_imp::event_manager()))
            .and_then(|addr| read_address(addr + event_manager_offsets::warp_event_entity()))?;

        bonfire_warp(self.bonfire_id, event_warp_entity)?;
        wait_for_loaded(false)
    }
}

fn bonfire_warp(bonfire_id: i32, event_warp_entity: u64) -> Result<()> {
    let bonfire_id_location = code_cave::base() + code_cave::BONFIRE_ID;
    write::<i32>(bonfire_id_location, bonfire_id)?;
    if is_scholar() {
        bonfire_warp_scholar(event_warp_entity)
    } else {
        bonfire_warp_vanilla(event_warp_entity)
    }
}

fn bonfire_warp_scholar(event_warp_entity: u64) -> Result<()> {
    let empty_space = code_cave::base() + code_cave::BONFIRE_WARP_OUTPUT;
    let bonfire_id_location = code_cave::base() + code_cave::BONFIRE_ID;
    let location = code_cave::base() + code_cave::BONFIRE_WARP_ASM;

    let mut asm = scholar::ASM.get_function("bonfire_warp").get_bytes();

    write_rel_i32(&mut asm, location, 7, empty_space, 4)?;
    write_rel_i32(&mut asm, location, 14, bonfire_id_location, 4)?;
    write_rel_i32(&mut asm, location, 25, functions::warp_prep(), 4)?;
    write_to_slice::<u64>(&mut asm, 31, event_warp_entity)?;
    write_rel_i32(&mut asm, location, 42, empty_space, 4)?;
    write_rel_i32(&mut asm, location, 47, functions::warp(), 4)?;
    append_flag_setter(location, &mut asm)?;

    write_bytes(location, &asm)?;
    run_thread(location)
}

fn bonfire_warp_vanilla(event_warp_entity: u64) -> Result<()> {
    let empty_space = code_cave::base() + code_cave::BONFIRE_WARP_OUTPUT;
    let bonfire_id_location = code_cave::base() + code_cave::BONFIRE_ID;
    let location = code_cave::base() + code_cave::BONFIRE_WARP_ASM;

    let mut asm = vanilla::ASM.get_function("bonfire_warp").get_bytes();

    write_to_slice::<u32>(&mut asm, 7, bonfire_id_location)?;
    write_to_slice::<u32>(&mut asm, 13, empty_space)?;
    write_to_slice::<u32>(&mut asm, 19, functions::warp_prep())?;
    write_to_slice::<u32>(&mut asm, 30, empty_space)?;
    write_to_slice::<u32>(&mut asm, 36, event_warp_entity)?;
    write_to_slice::<u32>(&mut asm, 41, functions::warp())?;
    append_flag_setter(location, &mut asm)?;

    write_bytes(location, &asm)?;
    run_thread(location)
}

fn event_warp(bonfire_id: i32, event_object_id: i32, event_warp_entity: u64) -> Result<()> {
    let params_location = code_cave::base() + code_cave::BONFIRE_ID;
    let mut params: [u8; 28] = [0; 28];
    write_to_slice::<i32>(&mut params, 0, 4)?;
    write_to_slice::<i32>(&mut params, 4, 5)?;
    write_to_slice::<i32>(&mut params, 8, bonfire_id)?;
    write_to_slice::<i32>(&mut params, 12, -1)?;
    write_to_slice::<i32>(&mut params, 24, event_object_id)?;
    write_bytes(params_location, &params)?;

    if is_scholar() {
        event_warp_scholar(event_warp_entity, params_location)
    } else {
        event_warp_vanilla(event_warp_entity, params_location)
    }
}

fn event_warp_scholar(event_warp_entity: u64, params_location: u64) -> Result<()> {
    let location = code_cave::base() + code_cave::EVENT_WARP_ASM;

    let mut asm = scholar::ASM.get_function("event_warp").get_bytes();

    write_to_slice::<u64>(&mut asm, 9, event_warp_entity)?;
    write_rel_i32(&mut asm, location, 20, params_location, 4)?;
    write_rel_i32(&mut asm, location, 25, functions::warp(), 4)?;
    append_flag_setter(location, &mut asm)?;

    write_bytes(location, &asm)?;
    run_thread(location)
}

fn event_warp_vanilla(event_warp_entity: u64, params_location: u64) -> Result<()> {
    let location = code_cave::base() + code_cave::EVENT_WARP_ASM;

    let mut asm = vanilla::ASM.get_function("event_warp").get_bytes();

    write_to_slice::<u32>(&mut asm, 1, event_warp_entity)?;
    write_to_slice::<u32>(&mut asm, 7, params_location)?;
    write_to_slice::<u32>(&mut asm, 13, functions::warp())?;
    append_flag_setter(location, &mut asm)?;

    write_bytes(location, &asm)?;
    run_thread(location)
}

fn write_coords_hook(coords: &[f32; 16]) -> Result<()> {
    let coords_location = code_cave::base() + code_cave::WARP_COORDS;

    write::<[f32; 16]>(coords_location, *coords)?;

    for (idx, coord) in coords.iter().enumerate() {
        write::<f32>(coords_location + idx as u64 * 4, *coord)?;
    }

    if is_scholar() {
        write_coords_hook_scholar(coords_location)
    } else {
        write_coords_hook_vanilla(coords_location)
    }
}

fn write_coords_hook_scholar(coords_location: u64) -> Result<()> {
    let location = code_cave::base() + code_cave::WARP_COORDS_HOOK;

    let mut asm = scholar::ASM.get_function("warp_coord_hook").get_bytes();

    write_to_slice::<u64>(&mut asm, 10, hk_hardware_info::base())?;
    write_rel_i32(&mut asm, location, 55, coords_location, 4)?;
    write_rel_i32(&mut asm, location, 68, coords_location + 0x10, 4)?;
    write_rel_i32(&mut asm, location, 81, coords_location + 0x20, 4)?;
    write_rel_i32(&mut asm, location, 94, coords_location + 0x30, 4)?;
    write_rel_i32(&mut asm, location, 120, hooks::warp_coord_write() + 7, 4)?;

    write_bytes(location, &asm)
}

fn write_coords_hook_vanilla(coords_location: u64) -> Result<()> {
    let location = code_cave::base() + code_cave::WARP_COORDS_HOOK;

    let mut asm = vanilla::ASM.get_function("warp_coord_hook").get_bytes();

    write_to_slice::<u32>(&mut asm, 9, game_manager_imp::base())?;
    write_to_slice::<u32>(&mut asm, 53, coords_location)?;
    write_to_slice::<u32>(&mut asm, 64, coords_location + 0x10)?;
    write_to_slice::<u32>(&mut asm, 75, coords_location + 0x20)?;
    write_to_slice::<u32>(&mut asm, 86, coords_location + 0x30)?;
    write_rel_i32(&mut asm, location, 103, hooks::warp_coord_write() + 7, 4)?;

    write_bytes(location, &asm)
}

fn wait_for_loaded(do_coords_hook: bool) -> Result<()> {
    while !is_loading_screen()? {
        thread::sleep(Duration::from_millis(50));
    }

    if do_coords_hook {
        let location = code_cave::base() + code_cave::WARP_COORDS_HOOK;
        install_hook_without_code(location, hooks::warp_coord_write(), 7)?;
    }

    while is_loading_screen()? {
        thread::sleep(Duration::from_millis(50));
    }
    thread::sleep(Duration::from_millis(200));

    if do_coords_hook {
        if is_scholar() {
            write_bytes(hooks::warp_coord_write(), &[0x0F, 0x5C, 0xC2, 0x0F, 0x29, 0x47, 0x50])?
        } else {
            write_bytes(hooks::warp_coord_write(), &[0x0F, 0x5C, 0xC1, 0x0F, 0x29, 0x46, 0x40])?
        }
    }
    Ok(())
}