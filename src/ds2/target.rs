use crate::{
    core::common::{rel_i32, write_to_slice},
    ds2::{
        chr_ctrl::ChrCtrl,
        mem::*,
        offsets::{code_cave, hooks},
        resources::{scholar, vanilla},
        utils::is_scholar,
    },
};
use anyhow::Result;

pub fn target_ctrl() -> ChrCtrl {
    read::<u64>(code_cave::base() + code_cave::SAVED_TARGET_POINTER)
}

pub fn install_target_hook() -> Result<()>{
    if is_scholar() {
        install_target_hook_scholar()
    } else {
        install_target_hook_vanilla()
    }
}

fn install_target_hook_scholar() -> Result<()> {
    let location = code_cave::base() + code_cave::TARGET_POINTER_HOOK;
    let pointer_location = code_cave::base() + code_cave::SAVED_TARGET_POINTER;
    let mut asm = scholar::ASM.get_function("save_target_hook").bytes.clone();
    write_to_slice::<i32>(&mut asm, 3, rel_i32(pointer_location, location + 7)?)?;
    write_to_slice::<i32>(&mut asm, 15, rel_i32(hooks::locked_target() + 7, location + 19)?)?;

    let mut hookbytes: [u8; 7] = [0xE9, 0x00, 0x00, 0x00, 0x00, 0x90, 0x90];
    write_to_slice::<i32>(&mut hookbytes, 1, rel_i32(location, hooks::locked_target() + 5)?)?;

    write_bytes(location, &asm)?;
    write_bytes(hooks::locked_target(), &hookbytes)
}

fn install_target_hook_vanilla() -> Result<()> {
    let location = code_cave::base() + code_cave::TARGET_POINTER_HOOK;
    let pointer_location = code_cave::base() + code_cave::SAVED_TARGET_POINTER;
    let mut asm = vanilla::ASM.get_function("save_target_hook").bytes.clone();
    write_to_slice::<u32>(&mut asm, 8, pointer_location)?;
    write_to_slice::<i32>(&mut asm, 13, rel_i32(hooks::locked_target() + 6, location + 17)?)?;

    let mut hookbytes: [u8; 6] = [0xE9, 0x00, 0x00, 0x00, 0x00, 0x90];
    write_to_slice::<i32>(&mut hookbytes, 1, rel_i32(location, hooks::locked_target() + 5)?)?;

    write_bytes(location, &asm)?;
    write_bytes(hooks::locked_target(), &hookbytes)
}

pub fn is_target_hook_active() -> Result<bool> {
    if is_scholar() {
        read::<[u8; 7]>(hooks::locked_target())
            .map(|val| val != [0x48, 0x89, 0xBB, 0xC0, 0x00, 0x00, 0x00])
    } else {
        read::<[u8; 6]>(hooks::locked_target())
            .map(|val| val != [0x89, 0xB7, 0xB8, 0x00, 0x00, 0x00])
    }
}