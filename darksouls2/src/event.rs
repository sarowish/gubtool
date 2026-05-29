use crate::{
    mem::*,
    offsets::{
        code_cave::CaveOffset, functions,
        game_manager_imp::{self, event_manager_offsets},
        hooks,
    },
    resources::{bosses::Boss, scholar, vanilla},
    utils::character_loaded_check,
};
use anyhow::{Ok, Result, bail};
use shared::{
    event_log::{EventLog, EventLogger},
    slice_ops::*,
};

pub fn get_event_flag(flag_id: u32) -> Result<bool> {
    character_loaded_check()?;
    let (byte_addr, bit_mask) = event_flag_lookup(flag_id)?;
    is_bit_set(byte_addr, bit_mask)
}

fn _set_event_flag_direct(flag_id: u32, state: bool) -> Result<()> {
    character_loaded_check()?;
    let (byte_addr, bit_mask) = event_flag_lookup(flag_id)?;
    set_bit(byte_addr, bit_mask, state)
}

#[derive(Debug)]
struct Node {
    bitmap_ptr: u64,
    size: u32,
    key: u32,
    next_node: u64,
}

impl Node {
    fn read_at(address: u64) -> Result<Self> {
        if is_scholar() {
            let bytes = read::<[u8; 0x18]>(address)?;
            Ok(Self {
                bitmap_ptr: read_from_slice::<u64>(&bytes, 0x0)?,
                size: read_from_slice::<u32>(&bytes, 0x8)?,
                key: read_from_slice::<u32>(&bytes, 0xC)?,
                next_node: read_from_slice::<u64>(&bytes, 0x10)?,
            })
        } else {
            let bytes = read::<[u8; 0x10]>(address)?;
            Ok(Self {
                bitmap_ptr: read_from_slice::<u32>(&bytes, 0x0)? as u64,
                size: read_from_slice::<u32>(&bytes, 0x4)?,
                key: read_from_slice::<u32>(&bytes, 0x8)?,
                next_node: read_from_slice::<u32>(&bytes, 0xC)? as u64,
            })
        }
    }
}

fn event_flag_lookup(flag_id: u32) -> Result<(u64, u8)> {
    let event_flag_man = read_address(game_manager_imp::base_ptr())
        .and_then(|addr| read_address(addr + game_manager_imp::event_manager()))
        .and_then(|addr| read_address(addr + event_manager_offsets::event_flag_manager()))?;

    let group = flag_id / 10000;
    let hash = group.wrapping_mul(0x89);
    let bit_index = flag_id % 10000;
    let bit_mask = 1u8 << (7 - (bit_index & 7));

    let first_node_offset = if is_scholar() {
        0x20 + (hash % 0x1F) as u64 * 8
    } else {
        0x10 + (hash % 0x1F) as u64 * 4
    };

    let mut node_ptr = read_address(event_flag_man + first_node_offset)?;

    while node_ptr != 0x0 {
        let node = Node::read_at(node_ptr)?;
        if node.key == group {
            let byte_index = bit_index >> 3;
            if byte_index < node.size {
                return Ok((node.bitmap_ptr + byte_index as u64, bit_mask));
            }
        }
        node_ptr = node.next_node;
    }
    bail!("Event flag not found")
}

pub fn set_event_flag(flag_id: u32, state: bool) -> Result<()> {
    character_loaded_check()?;
    let location = CaveOffset::SetEventAsm.addr();
    let event_flag_man = read_address(game_manager_imp::base_ptr())
        .and_then(|addr| read_address(addr + game_manager_imp::event_manager()))
        .and_then(|addr| read_address(addr + event_manager_offsets::event_flag_manager()))?;

    match is_scholar() {
        true => set_event_scholar(location, flag_id, state, event_flag_man),
        false => set_event_vanilla(location, flag_id, state, event_flag_man),
    }
}

fn set_event_scholar(location: u64, flag_id: u32, state: bool, event_flag_man: u64) -> Result<()> {
    let fun = scholar::ASM.get_function("set_event");
    let mut asm = fun.get_bytes();

    write_to_slice::<u64>(&mut asm, fun.reloc("event_flag_man"), event_flag_man)?;
    write_to_slice::<u32>(&mut asm, fun.reloc("state"), state)?;
    write_to_slice::<u64>(&mut asm, fun.reloc("event_id"), flag_id)?;
    write_to_slice::<u64>(&mut asm, fun.reloc("fn_set_event"), functions::set_event())?;
    append_flag_setter(location, &mut asm)?;

    write_bytes(location, &asm)?;
    run_thread(location)
}

fn set_event_vanilla(location: u64, flag_id: u32, state: bool, event_flag_man: u64) -> Result<()> {
    let fun = vanilla::ASM.get_function("set_event");
    let mut asm = fun.get_bytes();

    write_to_slice::<u32>(&mut asm, fun.reloc("event_flag_man"), event_flag_man)?;
    write_to_slice::<u32>(&mut asm, fun.reloc("state"), state)?;
    write_to_slice::<u32>(&mut asm, fun.reloc("event_id"), flag_id)?;
    write_to_slice::<u32>(&mut asm, fun.reloc("fn_set_event"), functions::set_event())?;
    append_flag_setter(location, &mut asm)?;

    write_bytes(location, &asm)?;
    run_thread(location)
}

impl Boss {
    pub fn revive(&self) -> Result<()> {
        set_event_flag(self.death_flag, false)
    }
    pub fn revive_status(&self) -> &str {
        if !get_event_flag(self.death_flag).unwrap_or_default() {
            "Alive"
        } else {
            "Dead"
        }
    }
}

#[derive(Default)]
pub struct Ds2EventLogger {
    event_log: EventLog,
}

impl EventLogger for Ds2EventLogger {
    fn event_log(&self) -> &EventLog {
        &self.event_log
    }
    fn event_log_mut(&mut self) -> &mut EventLog {
        &mut self.event_log
    }
    fn file_prefix(&self) -> &'static str {
        "darksouls2"
    }
    fn write_idx(&self) -> Result<i32> {
        read::<i32>(CaveOffset::EventLogWriteIdx.addr())
    }
    fn read_buffer(&self) -> Result<[u8; 0x1000]> {
        read::<[u8; 0x1000]>(CaveOffset::EventLogBuffer.addr())
    }
    fn clear_cave(&self) -> Result<()> {
        write::<i32>(CaveOffset::EventLogWriteIdx.addr(), 0x0)?;
        write_bytes(CaveOffset::EventLogBuffer.addr(), &[0x0; 0x1000])
    }
}

const EVENT_LOG_HOOK_ORIGINAL: [u8; 5] = [0xB8, 0x59, 0x17, 0xB7, 0xD1];
pub fn is_event_log_hook() -> Result<bool> {
    read::<[u8; 5]>(hooks::event_log()).map(|bytes| bytes != EVENT_LOG_HOOK_ORIGINAL)
}

pub fn set_event_log_hook(state: bool) -> Result<()> {
    match state {
        true => {
            let location = CaveOffset::EventLogHook.addr();
            let write_index = CaveOffset::EventLogWriteIdx.addr();
            let buffer = CaveOffset::EventLogBuffer.addr();
            if is_scholar() {
                install_event_log_hook_scholar(location, write_index, buffer)
            } else {
                install_event_log_hook_vanilla(location, write_index, buffer)
            }
        }
        false => write_bytes(hooks::event_log(), &EVENT_LOG_HOOK_ORIGINAL),
    }
}

fn install_event_log_hook_scholar(location: u64, write_index: u64, buffer: u64) -> Result<()> {
    let fun = scholar::ASM.get_function("event_log");
    let mut asm = fun.get_bytes();

    write_rel_i32(&mut asm, location, fun.reloc("write_index_1"), write_index, 4)?;
    write_rel_i32(&mut asm, location, fun.reloc("buffer"), buffer, 4)?;
    write_rel_i32(&mut asm, location, fun.reloc("write_index_2"), write_index, 4)?;
    write_rel_i32(&mut asm, location, fun.reloc("set_event"), hooks::event_log() + 5, 4)?;

    install_hook(&asm, location, hooks::event_log(), 5)
}

fn install_event_log_hook_vanilla(location: u64, write_index: u64, buffer: u64) -> Result<()> {
    let fun = vanilla::ASM.get_function("event_log");
    let mut asm = fun.get_bytes();

    write_to_slice::<u32>(&mut asm, fun.reloc("write_index_1"), write_index)?;
    write_to_slice::<u32>(&mut asm, fun.reloc("buffer"), buffer)?;
    write_to_slice::<u32>(&mut asm, fun.reloc("write_index_2"), write_index)?;
    write_rel_i32(&mut asm, location, fun.reloc("set_event"), hooks::event_log() + 5, 4)?;

    install_hook(&asm, location, hooks::event_log(), 5)
}

pub fn set_ivory_gauntlet_skip(state: bool) -> Result<()> {
    let location = CaveOffset::IvorySkipHook.addr();
    match (state, is_scholar()) {
        (true, true) => install_ivory_gauntlet_skip_scholar(location),
        (true, false) => install_ivory_gauntlet_skip_vanilla(location),
        (false, true) => write_bytes(functions::set_event(), &[0x48, 0x89, 0x74, 0x24, 0x10]),
        (false, false) => write_bytes(functions::set_event(), &[0x55, 0x8B, 0xEC, 0x83, 0xEC, 0x08]),
    }
}

pub fn is_ivory_gauntlet_skip() -> Result<bool> {
    match is_scholar() {
        true => read::<[u8; 5]>(functions::set_event()).map(|val| val != [0x48, 0x89, 0x74, 0x24, 0x10]),
        false => read::<[u8; 6]>(functions::set_event()) .map(|val| val != [0x55, 0x8B, 0xEC, 0x83, 0xEC, 0x08]),
    }
}

fn install_ivory_gauntlet_skip_scholar(location: u64) -> Result<()> {
    let fun = scholar::ASM.get_function("ivory_skip");
    let mut asm = fun.get_bytes();

    write_to_slice::<u64>(&mut asm, fun.reloc("fn_get_map_entity"), functions::get_map_entity_with_area_id_and_obj_id())?;
    write_to_slice::<u64>(&mut asm, fun.reloc("fn_get_map_object"), functions::get_state_act_component())?;
    write_to_slice::<u64>(&mut asm, fun.reloc("fn_set_event_1"), functions::set_event())?;
    write_rel_i32(&mut asm, location, fun.reloc("fn_set_event_2"), functions::set_event() + 5, 4)?;

    install_hook(&asm, location, functions::set_event(), 5)
}

fn install_ivory_gauntlet_skip_vanilla(location: u64) -> Result<()> {
    let fun = vanilla::ASM.get_function("ivory_skip");
    let mut asm = fun.get_bytes();

    write_to_slice::<u32>(&mut asm, 38, functions::set_event())?;
    write_to_slice::<u32>(&mut asm, 73, functions::get_map_entity_with_area_id_and_obj_id())?;
    write_to_slice::<u32>(&mut asm, 80, functions::get_state_act_component())?;
    write_rel_i32(&mut asm, location, 162, functions::set_event() + 6, 5)?;

    install_hook(&asm, location, functions::set_event(), 6)
}

pub fn set_ivory_no_knights(state: bool) -> Result<()> {
    let location = CaveOffset::IvoryKnightsHook.addr();
    match (state, is_scholar()) {
        (true, true) => install_ivory_no_knights_scholar(location),
        (true, false) => install_ivory_no_knights_vanilla(location),
        (false, true) => write_bytes(hooks::set_shared_flag(), &[0x44, 0x88, 0x84, 0x08, 0xA1, 0x03, 0x00, 0x00]),
        (false, false) => write_bytes(hooks::set_shared_flag(), &[0x88, 0x94, 0x08, 0xA1, 0x02, 0x00, 0x00]),
    }
}

pub fn is_ivory_no_knights() -> Result<bool> {
    match is_scholar() {
        true => {
            read::<[u8; 8]>(hooks::set_shared_flag())
                .map(|val| val != [0x44, 0x88, 0x84, 0x08, 0xA1, 0x03, 0x00, 0x00])
        }
        false => {
            read::<[u8; 7]>(hooks::set_shared_flag())
                .map(|val| val != [0x88, 0x94, 0x08, 0xA1, 0x02, 0x00, 0x00])
        }
    }
}

fn install_ivory_no_knights_scholar(location: u64) -> Result<()> {
    let mut asm = scholar::ASM.get_function("ivory_knights").get_bytes();
    write_rel_i32(&mut asm, location, 32, hooks::set_shared_flag() + 8, 4)?;

    install_hook(&asm, location, hooks::set_shared_flag(), 8)
}

fn install_ivory_no_knights_vanilla(location: u64) -> Result<()> {
    let mut asm = vanilla::ASM.get_function("ivory_knights").get_bytes();
    write_rel_i32(&mut asm, location, 28, hooks::set_shared_flag() + 7, 5)?;

    install_hook(&asm, location, hooks::set_shared_flag(), 7)
}