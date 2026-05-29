use crate::{
    chr_ins::ChrIns,
    mem::*,
    offsets::{code_cave::CaveOffset, field_area, hooks},
    resources::ASM,
};
use anyhow::{Result, anyhow, bail, ensure};
use engine::{
    attached::version,
    game_version::EldenRingVersion::*,
};
use shared::slice_ops::*;

pub fn target_ins() -> ChrIns {
    let addr = read::<u64>(CaveOffset::SavedTargetPointer.addr())
        .map_err(|_| anyhow!("Could not read target pointer"))?;
    ensure!(addr != 0x0, "Target not found");
    Ok(addr)
}

pub fn install_target_hook() -> Result<()> {
    let location = CaveOffset::SaveTargetHook.addr();
    let saved_pointer = CaveOffset::SavedTargetPointer.addr();

    let mut asm = ASM.get_function("save_target_hook").get_bytes();
    write_rel_i32(&mut asm, location, 10, saved_pointer, 4)?;
    write_rel_i32(&mut asm, location, 15, hooks::locked_target_pointer() + 7 , 4)?;

    install_hook(&asm, location, hooks::locked_target_pointer(), 7)
}

const TARGET_HOOK_BYTES_ORIGINAL: [u8; 7] = [0x48, 0x8B, 0x8F, 0x88, 0x00, 0x00, 0x00];
pub fn uninstall_target_hook() -> Result<()> {
    write_bytes(hooks::locked_target_pointer(), &TARGET_HOOK_BYTES_ORIGINAL)
}

pub fn is_target_hook_active() -> Result<bool> {
    read::<[u8; 7]>(hooks::locked_target_pointer())
        .map(|val| val != TARGET_HOOK_BYTES_ORIGINAL)
}

fn get_force_act_idx_original_bytes() -> [u8; 7] {
    match version() {
        Some(Version1_2_0) |
        Some(Version1_2_1) |
        Some(Version1_2_2) |
        Some(Version1_2_3) |
        Some(Version1_3_0) |
        Some(Version1_3_1) |
        Some(Version1_3_2) |
        Some(Version1_4_0) |
        Some(Version1_4_1) |
        Some(Version1_5_0) |
        Some(Version1_6_0) => [0x0F, 0xBE, 0x80, 0xB1, 0xE9, 0x00, 0x00],
        _ => [0x0F, 0xBE, 0x80, 0xC1, 0xE9, 0x00, 0x00],
    }
}

pub fn force_act_sequence(act_sequence: Vec<i32>, npc_think_param_id: i32) -> Result<()> {
    ensure!(act_sequence.len() <= 10, "Max number of acts is 10");

    let location = CaveOffset::ForceActSequenceHook.addr();
    let current_idx_location = CaveOffset::CurrentActIdx.addr();
    let should_run_flag_location = CaveOffset::ActSeqeunceShouldRun.addr();
    let act_array_location = CaveOffset::ActArray.addr();

    let mut act_array = act_sequence;
    act_array.resize(10, 0);
    let act_array: Vec<u8> = act_array.iter().flat_map(|&x| x.to_le_bytes()).collect();

    let mut asm = ASM.get_function("force_act_sequence_hook").get_bytes();
    write_rel_i32(&mut asm, location, 2, should_run_flag_location, 5)?;
    write_to_slice::<i32>(&mut asm, 12, npc_think_param_id)?;
    write_rel_i32(&mut asm, location, 23, current_idx_location, 4)?;
    write_rel_i32(&mut asm, location, 30, act_array_location, 4)?;
    write_rel_i32(&mut asm, location, 42, current_idx_location, 4)?;
    write_rel_i32(&mut asm, location, 53, should_run_flag_location, 5)?;
    write_rel_i32(&mut asm, location, 62, hooks::get_force_act_idx() + 7, 4)?;
    write_to_slice::<[u8; 7]>(&mut asm, 66, get_force_act_idx_original_bytes())?;
    write_rel_i32(&mut asm, location, 74, hooks::get_force_act_idx() + 7, 4)?;

    write::<i32>(current_idx_location, 0x0)?;
    write_bytes(act_array_location, &act_array)?;
    write::<u8>(should_run_flag_location, 0x1)?;
    install_hook(&asm, location, hooks::get_force_act_idx(), 7)
}

pub fn install_stagger_hook() -> Result<()> {
    let location = CaveOffset::TargetNoStaggerHook.addr();
    let target_ptr_location = CaveOffset::SavedTargetPointer.addr();

    let mut asm = ASM.get_function("target_stagger_hook").get_bytes();
    write_rel_i32(&mut asm, location, 8, target_ptr_location, 4)?;
    write_rel_i32(&mut asm, location, 24, hooks::target_no_stagger() + 8, 4)?;

    install_hook(&asm, location, hooks::target_no_stagger(), 8)
}

const TARGET_STAGGER_HOOK_BYTES_ORIGINAL: [u8; 8] = [0x48, 0x8B, 0x41, 0x08, 0x83, 0x48, 0x2C, 0x08];
pub fn uninstall_stagger_hook() -> Result<()> {
    write_bytes(hooks::target_no_stagger(), &TARGET_STAGGER_HOOK_BYTES_ORIGINAL)
}

pub fn is_stagger_hook_active() -> Result<bool> {
    read::<[u8; 8]>(hooks::target_no_stagger())
        .map(|val| val != TARGET_STAGGER_HOOK_BYTES_ORIGINAL)
}

pub fn toggle_stagger_hook() -> Result<()> {
    match is_stagger_hook_active()? {
        true => uninstall_stagger_hook(),
        false => install_stagger_hook(),
    }
}

pub fn world_block_info_from_block_id(block_id: u32) -> Result<u64> {
    let target_area = (block_id >> 24) & 0xFF;
    let world_info_owner = read::<u64>(field_area::base_ptr())
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
