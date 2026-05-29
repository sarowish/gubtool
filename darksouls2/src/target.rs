use crate::{
    chr_ctrl::{ChrCtrl, ChrCtrlExt},
    mem::*,
    offsets::{code_cave::CaveOffset, hooks},
    resources::{scholar, vanilla},
};
use anyhow::{Result, anyhow, ensure};
use shared::slice_ops::*;

pub fn target_ctrl() -> ChrCtrl {
    let target_ptr = read::<u64>(CaveOffset::SavedTargetPointer.addr())
        .map_err(|_| anyhow!("Could not read target pointer"))?;
    let target_ins = Ok(target_ptr);
    is_valid_target(&target_ins)
        .map_err(|_| anyhow!("Target not found"))?;
    target_ins
}

pub fn is_valid_target(target_ins: &ChrCtrl) -> Result<()> {
    ensure!(target_ins.chr_ctrl_pointer()? != 0x0, "");
    let health = target_ins.get_hp()?;
    let max_health = target_ins.max_hp()?;
    ensure!(health >= 0 && max_health > 0 && health < 10000000 && max_health < 10000000 && (health as f32) < (max_health as f32) * 1.5, "");
    Ok(())
}

pub fn install_target_hook() -> Result<()>{
    let location = CaveOffset::SaveTargetHook.addr();
    let saved_ptr_loc = CaveOffset::SavedTargetPointer.addr();
    match is_scholar() {
        true => install_target_hook_scholar(location, saved_ptr_loc),
        false => install_target_hook_vanilla(location, saved_ptr_loc),
    }
}

fn install_target_hook_scholar(location: u64, saved_ptr_loc: u64) -> Result<()> {
    let mut asm = scholar::ASM.get_function("save_target_hook").get_bytes();

    write_rel_i32(&mut asm, location, 3, saved_ptr_loc, 4)?;
    write_rel_i32(&mut asm, location, 15, hooks::locked_target_pointer() + 7, 4)?;

    install_hook(&asm, location, hooks::locked_target_pointer(), 7)
}

fn install_target_hook_vanilla(location: u64, saved_ptr_loc: u64) -> Result<()> {
    let mut asm = vanilla::ASM.get_function("save_target_hook").get_bytes();

    write_to_slice::<u32>(&mut asm, 8, saved_ptr_loc)?;
    write_rel_i32(&mut asm, location, 13, hooks::locked_target_pointer() + 6, 4)?;

    install_hook(&asm, location, hooks::locked_target_pointer(), 6)
}

pub fn is_target_hook_active() -> Result<bool> {
    if is_scholar() {
        read::<[u8; 7]>(hooks::locked_target_pointer())
            .map(|val| val != [0x48, 0x89, 0xBB, 0xC0, 0x00, 0x00, 0x00])
    } else {
        read::<[u8; 6]>(hooks::locked_target_pointer())
            .map(|val| val != [0x89, 0xB7, 0xB8, 0x00, 0x00, 0x00])
    }
}