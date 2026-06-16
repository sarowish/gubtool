use crate::{
    chr_ctrl::{ChrCtrl, ChrCtrlExt},
    mem::*,
    offsets::{code_cave::CaveOffset, hooks},
    resources::{scholar, vanilla},
};
use gubtool_core::sys::error::{PointerType, ProcResult, ProcessError};
use utils::slice_ops::*;

pub fn target_ctrl() -> ChrCtrl {
    let target_ptr = read::<u64>(CaveOffset::SavedTargetPointer.addr())?;
    let target_ins = Ok(target_ptr);
    if !target_ins.is_valid_chr()? {
        return Err(ProcessError::InvalidPointer { pointer_type: PointerType::TargetIns })
    }
    target_ins
}

pub fn install_target_hook() -> ProcResult {
    let location = CaveOffset::SaveTargetHook.addr();
    let saved_ptr_loc = CaveOffset::SavedTargetPointer.addr();
    match is_scholar() {
        true => install_target_hook_scholar(location, saved_ptr_loc),
        false => install_target_hook_vanilla(location, saved_ptr_loc),
    }
}

fn install_target_hook_scholar(location: u64, saved_ptr_loc: u64) -> ProcResult {
    let mut asm = scholar::ASM.get_function("save_target_hook").get_bytes();

    write_rel_i32(&mut asm, location, 3, saved_ptr_loc, 4)?;
    write_rel_i32(&mut asm, location, 15, hooks::locked_target_pointer() + 7, 4)?;

    install_hook(&asm, location, hooks::locked_target_pointer(), 7)
}

fn install_target_hook_vanilla(location: u64, saved_ptr_loc: u64) -> ProcResult {
    let mut asm = vanilla::ASM.get_function("save_target_hook").get_bytes();

    write_to_slice::<u32>(&mut asm, 8, saved_ptr_loc)?;
    write_rel_i32(&mut asm, location, 13, hooks::locked_target_pointer() + 6, 4)?;

    install_hook(&asm, location, hooks::locked_target_pointer(), 6)
}

pub fn is_target_hook_active() -> ProcResult<bool> {
    if is_scholar() {
        read::<[u8; 7]>(hooks::locked_target_pointer())
            .map(|val| val != [0x48, 0x89, 0xBB, 0xC0, 0x00, 0x00, 0x00])
    } else {
        read::<[u8; 6]>(hooks::locked_target_pointer())
            .map(|val| val != [0x89, 0xB7, 0xB8, 0x00, 0x00, 0x00])
    }
}