use crate::{
    chr_ins::ChrIns,
    mem::*,
    offsets::{
        code_cave::CaveOffset,
        field_area,
        module_offsets::{BasePointer, Hook},
    },
    resources::ASM,
};
use anyhow::bail;
use gubtool_core::{
    address::Address,
    attached::version,
    game_version::EldenRingVersion::*,
    slice_ops::*,
    sys::error::{PointerType, ProcResult, ProcessError},
};
use shared::act_array::ActArray;

pub fn target_ins() -> ChrIns {
    match read::<u64>(CaveOffset::SavedTargetPointer) {
        Ok(ptr) if ptr != 0x0 => Ok(ptr),
        Ok(_) | Err(_) => Err(ProcessError::InvalidPointer {
            pointer_type: PointerType::TargetIns,
        }),
    }
}

pub fn install_target_hook() -> ProcResult {
    let mut fun = ASM.get_function("save_target_hook");
    let mut asm = fun.take_bytes();

    write_addr_to_slice(&mut asm, fun.reloc("saved_pointer_loc"), CaveOffset::SavedTargetPointer)?;
    write_rel_i32(
        &mut asm,
        CaveOffset::SaveTargetHook,
        fun.reloc("hook_loc"),
        Hook::LockedTargetPointer.add_offset(7),
        4,
    )?;
    install_hook(&asm, CaveOffset::SaveTargetHook, Hook::LockedTargetPointer, 7)
}

const TARGET_HOOK_BYTES_ORIGINAL: [u8; 7] = [0x48, 0x8B, 0x8F, 0x88, 0x00, 0x00, 0x00];
pub fn uninstall_target_hook() -> ProcResult {
    write_bytes(Hook::LockedTargetPointer, &TARGET_HOOK_BYTES_ORIGINAL)
}

pub fn is_target_hook_active() -> ProcResult<bool> {
    read::<[u8; 7]>(Hook::LockedTargetPointer)
        .map(|val| val != TARGET_HOOK_BYTES_ORIGINAL)
}

fn force_act_orig_instr_off() -> i32 {
    match version() {
        Some(Version1_2_0) | Some(Version1_2_1) | Some(Version1_2_2) |
        Some(Version1_2_3) | Some(Version1_3_0) | Some(Version1_3_1) |
        Some(Version1_3_2) | Some(Version1_4_0) | Some(Version1_4_1) |
        Some(Version1_5_0) | Some(Version1_6_0) => 0xE9B1,
        _ => 0xE9C1,
    }
}

pub fn force_act_sequence(mut act_sequence: ActArray, npc_think_param_id: i32) -> ProcResult {
    let location = CaveOffset::ForceActSequenceHook;

    let mut fun = ASM.get_function("force_act_sequence_hook");
    let mut asm = fun.take_bytes();

    write_addr_to_slice(&mut asm, fun.reloc("should_run_flag"), CaveOffset::ActSeqeunceShouldRun)?;
    write_to_slice::<i32>(&mut asm, fun.reloc("npc_think_param_id"), npc_think_param_id)?;
    write_addr_to_slice(&mut asm, fun.reloc("current_idx"), CaveOffset::CurrentActIdx)?;
    write_addr_to_slice(&mut asm, fun.reloc("act_array"), CaveOffset::ActArray)?;
    write_to_slice::<i32>(&mut asm, fun.reloc("orig_instr_off"), force_act_orig_instr_off())?;
    write_rel_i32(&mut asm, location, fun.reloc("hook_loc"), Hook::GetForceActIdx.add_offset(7), 4)?;

    act_sequence.zero_fill();
    write_bytes(CaveOffset::ActArray, &act_sequence.as_qword_le_bytes())?;
    write::<i32>(CaveOffset::CurrentActIdx, 0x0)?;
    write::<u8>(CaveOffset::ActSeqeunceShouldRun, 0x1)?;
    install_hook(&asm, location, Hook::GetForceActIdx, 7)
}

pub fn install_stagger_hook() -> ProcResult {
    let mut fun = ASM.get_function("target_stagger_hook");
    let mut asm = fun.take_bytes();

    write_addr_to_slice(&mut asm, fun.reloc("target_ptr_loc"), CaveOffset::SavedTargetPointer)?;
    write_rel_i32(
        &mut asm,
        CaveOffset::TargetNoStaggerHook,
        fun.reloc("hook_loc"),
        Hook::TargetNoStagger.add_offset(8),
        4,
    )?;
    install_hook(&asm, CaveOffset::TargetNoStaggerHook, Hook::TargetNoStagger, 8)
}

const TARGET_STAGGER_HOOK_BYTES_ORIGINAL: [u8; 8] = [0x48, 0x8B, 0x41, 0x08, 0x83, 0x48, 0x2C, 0x08];
pub fn uninstall_stagger_hook() -> ProcResult {
    write_bytes(Hook::TargetNoStagger, &TARGET_STAGGER_HOOK_BYTES_ORIGINAL)
}

pub fn is_stagger_hook_active() -> ProcResult<bool> {
    read::<[u8; 8]>(Hook::TargetNoStagger)
        .map(|val| val != TARGET_STAGGER_HOOK_BYTES_ORIGINAL)
}

pub fn toggle_stagger_hook() -> ProcResult {
    match is_stagger_hook_active()? {
        true => uninstall_stagger_hook(),
        false => install_stagger_hook(),
    }
}

pub fn world_block_info_from_block_id(block_id: u32) -> anyhow::Result<u64> {
    let target_area = (block_id >> 24) & 0xFF;
    let world_info_owner = read::<u64>(BasePointer::FieldArea)
        .and_then(|addr| read::<u64>(addr + field_area::WORLD_INFO_OWNER))?;
    let area_count = read::<i32>(world_info_owner + field_area::world_info_owner_offsets::AREA_COUNT)?;

    for i in 0..area_count as u64 {
        let area_ptr = read::<u64>(world_info_owner + field_area::world_info_owner_offsets::AREA_ARRAY_BASE + (i * 8))?;
        let area_id = read::<u32>(area_ptr + 0xC)?;

        if area_id == target_area {
            let block_count = read::<i32>(area_ptr + 0x40)?;
            let blocks_ptr = read::<u64>(area_ptr + 0x48)?;

            for j in 0..block_count as u64 {
                let block_info_ptr = blocks_ptr + (j * 0xE0);
                let stored_block_id = read::<u32>(block_info_ptr + 0x8)?;

                if stored_block_id == block_id {
                    return Ok(block_info_ptr);
                }
            }
        }
    }
    bail!("Could not find world block info")
}
