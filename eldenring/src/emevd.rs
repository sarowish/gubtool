use anyhow::Result;
use shared::slice_ops::write_to_slice;

use crate::{
    mem::{EXECUTE_EMEVD_COMMAND_MUTEX, append_flag_setter, run_thread, write_bytes},
    offsets::{code_cave::CaveOffset, cs_emk_system, functions},
    resources::ASM,
};

fn execute_emevd_command(group_id: i32, command_id: i32, args: &[u8]) -> Result<()> {
    let location = CaveOffset::EmevdAsm.addr();
    let args_location = CaveOffset::EmevdArgs.addr();

    let fun = ASM.get_function("execute_emevd_command");
    let mut asm = fun.get_bytes();
    write_to_slice::<u64>(&mut asm, fun.reloc("fn_emk_event_ins_ctor"), functions::emk_event_ins_ctor())?;
    write_to_slice::<i32>(&mut asm, fun.reloc("group_id"), group_id)?;
    write_to_slice::<i32>(&mut asm, fun.reloc("command_id"), command_id)?;
    write_to_slice::<u64>(&mut asm, fun.reloc("args_location"), args_location)?;
    write_to_slice::<u64>(&mut asm, fun.reloc("cs_emk_system_base"), cs_emk_system::base_ptr())?;
    write_to_slice::<u64>(&mut asm, fun.reloc("fn_emevd_switch"), functions::emevd_switch())?;
    append_flag_setter(location, &mut asm)?;

    let _handle = EXECUTE_EMEVD_COMMAND_MUTEX.lock().unwrap();

    write_bytes(args_location, args)?;
    write_bytes(location, &asm)?;
    run_thread(location)
}

pub fn set_night() -> Result<()> {
    let mut param_data: [u8; 20] = [0x0; 20];
    write_to_slice::<u8>(&mut param_data, 0, 20)?;
    write_to_slice::<u8>(&mut param_data, 5, 1)?;
    write_to_slice::<f32>(&mut param_data, 8, 0.75)?;
    write_to_slice::<f32>(&mut param_data, 12, 2.0)?;
    write_to_slice::<f32>(&mut param_data, 16, 0.0)?;
    execute_emevd_command(2001, 4, &param_data)
}

pub fn rest() -> Result<()> {
    execute_emevd_command(2004, 47, &[])
}


pub fn disable_title_card() -> Result<()> {
    execute_emevd_command(2012, 8, &[])
}

pub fn reset_character_position(entity_id: u32) -> Result<()> {
    let mut param_data: [u8; 20] = [0x0; 20];
    write_to_slice::<u32>(&mut param_data, 0, entity_id)?;
    execute_emevd_command(2004, 81, &param_data)
}

pub fn force_animation_playback(
    entity_id: u32,
    animation_id: u32,
    should_loop: bool,
    should_wait_for_completion: bool,
    ignore_wait_for_transition: bool,
) -> Result<()> {
    let mut param_data: [u8; 20] = [0x0; 20];
    write_to_slice::<u32>(&mut param_data, 0, entity_id)?;
    write_to_slice::<u32>(&mut param_data, 4, animation_id)?;
    write_to_slice::<u8>(&mut param_data, 8, should_loop as u8)?;
    write_to_slice::<u8>(&mut param_data, 9, should_wait_for_completion as u8)?;
    write_to_slice::<u8>(&mut param_data, 10, ignore_wait_for_transition as u8)?;
    execute_emevd_command(2003, 18, &param_data)
}