use crate::{
    core::common::{write_rel_i32, write_to_slice},
    er::{
        event,
        mem::*,
        offsets::{code_cave, functions, hooks, menu_man, world_chr_man},
        resources::{ASM, bosses::Boss, graces::Grace},
        utils::{character_loaded_check, dlc_check},
    },
};
use anyhow::Result;
use std::{thread, time::Duration};

pub fn warp_to_grace(grace_id: i64) -> Result<()> {
    let location = code_cave::base() + code_cave::GRACE_WARP_ASM;

    let mut asm = ASM.get_function("warp_to_grace").bytes.clone();
    write_to_slice::<u64>(&mut asm, 2, world_chr_man::base())?;
    write_to_slice::<i64>(&mut asm, 20, grace_id)?;
    write_to_slice::<u64>(&mut asm, 30, functions::grace_warp())?;
    append_flag_setter(location, &mut asm)?;

    write_bytes(location, &asm)?;
    run_thread(location)
}

pub fn warp_to_block_id(block_id: i32, coords: [f32; 3], angle: f32, is_night: bool) -> Result<()> {
    let location = code_cave::base() + code_cave::BLOCK_WARP_ASM;
    let area: i32 = (block_id >> 24) & 0xFF;
    let block: i32 = (block_id >> 16) & 0xFF;
    let map: i32 = (block_id >> 8) & 0xFF;
    let alt_no: i32 = block_id & 0xFF;

    let mut asm = ASM.get_function("warp_to_block_id").bytes.clone();
    write_to_slice::<i32>(&mut asm, 1, area)?;
    write_to_slice::<i32>(&mut asm, 6, block)?;
    write_to_slice::<i32>(&mut asm, 12, map)?;
    write_to_slice::<i32>(&mut asm, 18, alt_no)?;
    write_to_slice::<u64>(&mut asm, 24, functions::block_warp())?;
    append_flag_setter(location, &mut asm)?;

    write_bytes(location, &asm)?;
    run_thread(location)?;
    hook_warp_coord_writes(coords, angle, is_night)
}

fn hook_warp_coord_writes(coords: [f32; 3], angle: f32, is_night: bool) -> Result<()> {
    let target_coords_location = code_cave::base() + code_cave::WARP_COORDS;
    let target_angle_location = code_cave::base() + code_cave::WARP_ANGLE;
    let angle_offset_in_struct: i32 = 0xAB0;

    let mut target_coords: [u8; 16] = [0; 16];
    write_to_slice::<f32>(&mut target_coords, 0, coords[0])?;
    write_to_slice::<f32>(&mut target_coords, 4, coords[1])?;
    write_to_slice::<f32>(&mut target_coords, 8, coords[2])?;
    write_to_slice::<f32>(&mut target_coords, 12, 1.0)?;

    write_bytes(target_coords_location, &target_coords)?;
    write::<f32>(target_angle_location + 4, angle)?;

    let mut asm = ASM.get_function("warp_coord_angle_hook").bytes.clone();

    let warp_code_location = code_cave::base() + code_cave::WARP_COORDS_HOOK;

    write_rel_i32(&mut asm, warp_code_location, 3, target_coords_location, 4)?;
    write_rel_i32(&mut asm, warp_code_location, 15, hooks::warp_coord_write() + 7, 4)?;

    write_bytes(warp_code_location, &asm)?;

    let angle_code_location = code_cave::base() + code_cave::WARP_ANGLE_HOOK;
    write_rel_i32(&mut asm, angle_code_location, 3, target_angle_location, 4)?;
    write_to_slice::<i32>(&mut asm, 10, angle_offset_in_struct)?;
    write_rel_i32(&mut asm, angle_code_location, 15, hooks::warp_angle_write() + 7, 4)?;

    write_bytes(angle_code_location, &asm)?;

    install_hook(warp_code_location, hooks::warp_coord_write(), 6)?;
    install_hook(angle_code_location, hooks::warp_angle_write(), 6)?;

    wait_to_unhook_warp(is_night)
}

const COORD_HOOK_ORIGINAL: [u8; 7] = [0x0F, 0x11, 0x80, 0xA0, 0x0A, 0x00, 0x00];
const ANGLE_HOOK_ORIGINAL: [u8; 7] = [0x0F, 0x11, 0x80, 0xB0, 0x0A, 0x00, 0x00];
fn wait_to_unhook_warp(is_night: bool) -> Result<()> {
    let is_faded_ptr = read::<u64>(menu_man::base())? + menu_man::is_fading();

    while !is_bit_set(is_faded_ptr, menu_man::fade_bit_flags::IS_FADE_SCREEN)? {
        thread::sleep(Duration::from_millis(20));
    }

    while is_bit_set(is_faded_ptr, menu_man::fade_bit_flags::IS_FADE_SCREEN)? {
        thread::sleep(Duration::from_millis(20));
    }
    if is_night {
        event::set_night()?;
    }
    write_bytes(hooks::warp_coord_write(), &COORD_HOOK_ORIGINAL)?;
    write_bytes(hooks::warp_angle_write(), &ANGLE_HOOK_ORIGINAL)
}

impl Boss {
    pub fn warp(&self) -> Result<()> {
        character_loaded_check()?;
        if self.dlc {
            dlc_check()?;
        }
        if self.name == "Grafted Scion" && !event::get_event(10010801)? {
            warp_to_block_id(self.block_id, [-33.27, 21.37, -87.86], 2.92, self.is_night)
        } else {
            warp_to_block_id(self.block_id, self.coords, self.angle, self.is_night)
        }
    }
}

impl Grace {
    pub fn warp(&self) -> Result<()> {
        character_loaded_check()?;
        if self.dlc {
            dlc_check()?;
        }
        warp_to_grace(self.grace_entity_id)
    }
}
