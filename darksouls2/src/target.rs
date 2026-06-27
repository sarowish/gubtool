use crate::{
    chr_ctrl::{ChrCtrl, ChrCtrlExt},
    mem::*,
    offsets::{code_cave::CaveOffset, module_offsets::Hook},
    resources::asm_function,
};
use gubtool_core::slice_ops::*;
use gubtool_core::{
    address::Address,
    attached::is_32,
    sys::error::{PointerType, ProcResult, ProcessError},
};

pub fn target_ctrl() -> ChrCtrl {
    let target_ptr = read::<u64>(CaveOffset::SavedTargetPointer)?;
    let target_ctrl = Ok(target_ptr);
    if !target_ctrl.is_valid_chr()? {
        return Err(ProcessError::InvalidPointer { pointer_type: PointerType::TargetIns })
    }
    target_ctrl
}

pub fn install_target_hook() -> ProcResult {
    let orig_instr_len = if is_32() { 6 } else { 7 };

    let mut fun = asm_function("save_target_hook");
    let mut asm = fun.take_bytes();

    write_addr_to_slice(&mut asm, fun.reloc("saved_ptr_loc"), CaveOffset::SavedTargetPointer)?;
    write_rel_i32(
        &mut asm,
        CaveOffset::SaveTargetHook,
        fun.reloc("hook_loc"),
        Hook::LockedTargetPointer.add_offset(orig_instr_len),
        4
    )?;
    install_hook(&asm, CaveOffset::SaveTargetHook, Hook::LockedTargetPointer, orig_instr_len)
}

pub fn is_target_hook_active() -> bool {
    if is_32() {
        read::<[u8; 6]>(Hook::LockedTargetPointer)
            .map(|val| val != [0x89, 0xB7, 0xB8, 0x00, 0x00, 0x00])
    } else {
        read::<[u8; 7]>(Hook::LockedTargetPointer)
            .map(|val| val != [0x48, 0x89, 0xBB, 0xC0, 0x00, 0x00, 0x00])
    }
    .unwrap_or_default()
}