use crate::{
    mem::{EXECUTE_EMEVD_COMMAND_MUTEX, spawn_thread_join, write_bytes},
    offsets::{
        code_cave::CaveOffset,
        module_offsets::{BasePointer, Function},
    },
    resources::ASM,
    utils::player_loaded_check,
};
use gubtool_core::{slice_ops::*, sys::error::ProcResult};

fn execute_emevd_command(group_id: i32, command_id: i32, args: &[u8]) -> ProcResult {
    let mut fun = ASM.get_function("execute_emevd_command");
    let mut asm = fun.take_bytes();

    write_addr_to_slice(&mut asm, fun.reloc("fn_emk_event_ins_ctor"), Function::EmkEventInsCtor)?;
    write_to_slice::<i32>(&mut asm, fun.reloc("group_id"), group_id)?;
    write_to_slice::<i32>(&mut asm, fun.reloc("command_id"), command_id)?;
    write_addr_to_slice(&mut asm, fun.reloc("args_location"), CaveOffset::EmevdArgs)?;
    write_addr_to_slice(&mut asm, fun.reloc("cs_emk_system_base"), BasePointer::CsEmkSystem)?;
    write_addr_to_slice(&mut asm, fun.reloc("fn_emevd_switch"), Function::EmevdSwitch)?;

    let _handle = EXECUTE_EMEVD_COMMAND_MUTEX.lock().unwrap();

    write_bytes(CaveOffset::EmevdArgs, args)?;
    spawn_thread_join(CaveOffset::EmevdAsm, asm)
}

pub fn set_night() -> ProcResult {
    let mut param_data: [u8; 20] = [0x0; 20];
    write_to_slice::<u8>(&mut param_data, 0, 20)?;
    write_to_slice::<u8>(&mut param_data, 5, 1)?;
    write_to_slice::<f32>(&mut param_data, 8, 0.75)?;
    write_to_slice::<f32>(&mut param_data, 12, 2.0)?;
    write_to_slice::<f32>(&mut param_data, 16, 0.0)?;
    execute_emevd_command(2001, 4, &param_data)
}

pub fn rest() -> ProcResult {
    player_loaded_check()?;
    execute_emevd_command(2004, 47, &[])
}


pub fn disable_title_card() -> ProcResult {
    execute_emevd_command(2012, 8, &[])
}

pub fn reset_character_position(entity_id: u32) -> ProcResult {
    player_loaded_check()?;
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
) -> ProcResult {
    let mut param_data: [u8; 20] = [0x0; 20];
    write_to_slice::<u32>(&mut param_data, 0, entity_id)?;
    write_to_slice::<u32>(&mut param_data, 4, animation_id)?;
    write_to_slice::<u8>(&mut param_data, 8, should_loop as u8)?;
    write_to_slice::<u8>(&mut param_data, 9, should_wait_for_completion as u8)?;
    write_to_slice::<u8>(&mut param_data, 10, ignore_wait_for_transition as u8)?;
    execute_emevd_command(2003, 18, &param_data)
}