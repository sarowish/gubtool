use crate::{
    mem::*,
    offsets::{
        ChainReadExt,
        code_cave::CaveOffset,
        game_manager_imp::{self, event_manager_offsets},
        module_offsets::{BasePointer, Function, Hook},
    },
    resources::{asm_function, bosses::Boss, event_flags::EventFlag, map_ids::MapId},
    utility,
    utils::player_loaded_check,
};
use anyhow::{anyhow, ensure};
use gubtool_core::slice_ops::*;
use gubtool_core::{address::Address, attached::is_32, sys::error::ProcResult};
use shared::event_log::{EventLog, EventLogger};

pub fn get_event_flag(flag_id: u32) -> anyhow::Result<bool> {
    let (byte_addr, bit_mask) = event_flag_lookup(flag_id)?;
    Ok(is_bit_set(byte_addr, bit_mask)?)
}

fn _set_event_flag_direct(flag_id: u32, state: bool) -> anyhow::Result<()> {
    player_loaded_check()?;
    let (byte_addr, bit_mask) = event_flag_lookup(flag_id)?;
    Ok(set_bit(byte_addr, bit_mask, state)?)
}

#[derive(Debug)]
struct Node {
    bitmap_ptr: u64,
    size: u32,
    key: u32,
    next_node: u64,
}

impl Node {
    fn read_at(address: u64) -> ProcResult<Self> {
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

fn event_flag_lookup(flag_id: u32) -> anyhow::Result<(u64, u8)> {
    let event_flag_man = read_address(BasePointer::GameManagerImp)
        .read_offset(game_manager_imp::EVENT_MANAGER)
        .read_offset(event_manager_offsets::EVENT_FLAG_MANAGER)?;

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
    Err(anyhow!("Event flag not found"))
}

pub fn set_event_flag(flag_id: u32, state: bool) -> anyhow::Result<()> {
    player_loaded_check()?;

    let event_flag_man = read_address(BasePointer::GameManagerImp)
        .read_offset(game_manager_imp::EVENT_MANAGER)
        .read_offset(event_manager_offsets::EVENT_FLAG_MANAGER)?;

        let mut fun = asm_function("set_event");
        let mut asm = fun.take_bytes();

        write_addr_to_slice(&mut asm, fun.reloc("event_flag_man"), event_flag_man)?;
        write_to_slice::<u32>(&mut asm, fun.reloc("state"), state)?;
        write_to_slice::<u32>(&mut asm, fun.reloc("event_id"), flag_id)?;
        write_addr_to_slice(&mut asm, fun.reloc("fn_set_event"), Function::SetEvent)?;

    Ok(spawn_thread_join(CaveOffset::SetEventAsm, asm)?)
}

impl Boss {
    pub fn revive(&self) -> anyhow::Result<()> {
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
    fn write_idx(&self) -> ProcResult<i32> {
        read::<i32>(CaveOffset::EventLogWriteIdx)
    }
    fn read_buffer(&self) -> ProcResult<[u8; 0x1000]> {
        read::<[u8; 0x1000]>(CaveOffset::EventLogBuffer)
    }
    fn clear_cave(&self) -> ProcResult {
        write::<i32>(CaveOffset::EventLogWriteIdx, 0x0)?;
        write_bytes(CaveOffset::EventLogBuffer, &[0x0; 0x1000])
    }
}

const EVENT_LOG_HOOK_ORIGINAL: [u8; 5] = [0xB8, 0x59, 0x17, 0xB7, 0xD1];
pub fn set_event_log_hook(state: bool) -> ProcResult {
    if state {
        let mut fun = asm_function("event_log");
        let mut asm = fun.take_bytes();

        write_addr_to_slice(&mut asm, fun.reloc("write_index"), CaveOffset::EventLogWriteIdx)?;
        write_addr_to_slice(&mut asm, fun.reloc("buffer"), CaveOffset::EventLogBuffer)?;
        write_rel_i32(
            &mut asm,
            CaveOffset::EventLogHook,
            fun.reloc("hook_loc"),
            Hook::EventLog.add_offset(5),
            4
        )?;
        install_hook(&asm, CaveOffset::EventLogHook, Hook::EventLog, 5)
    } else {
        write_bytes(Hook::EventLog, &EVENT_LOG_HOOK_ORIGINAL)
    }
}

pub fn is_event_log_hook() -> bool {
    read::<[u8; 5]>(Hook::EventLog)
        .map(|bytes| bytes != EVENT_LOG_HOOK_ORIGINAL)
        .unwrap_or_default()
}

const VANILLA_IVORY_SKIP_ORIGINAL: [u8; 6] = [0x55, 0x8B, 0xEC, 0x83, 0xEC, 0x08];
const SCHOLAR_IVORY_SKIP_ORIGINAL: [u8; 5] = [0x48, 0x89, 0x74, 0x24, 0x10];
pub fn set_ivory_gauntlet_skip(state: bool) -> ProcResult {
    if state {
        let orig_instr_len = if is_32() { 6 } else { 5 };
        let mut fun = asm_function("ivory_skip");
        let mut asm = fun.take_bytes();

        write_addr_to_slice(&mut asm, fun.reloc("fn_get_map_entity"), Function::GetMapEntityWithAreaIdAndObjId)?;
        write_addr_to_slice(&mut asm, fun.reloc("fn_get_map_object"), Function::GetStateActComponent)?;
        write_addr_to_slice(&mut asm, fun.reloc("fn_set_event"), Function::SetEvent)?;
        write_rel_i32(
            &mut asm,
            CaveOffset::IvorySkipHook,
            fun.reloc("hook_loc"),
            Function::SetEvent.add_offset(orig_instr_len),
            4
        )?;
        install_hook(&asm, CaveOffset::IvorySkipHook, Function::SetEvent, orig_instr_len)
    } else {
        let bytes: &[u8] = if is_32() {
            &VANILLA_IVORY_SKIP_ORIGINAL
        } else {
            &SCHOLAR_IVORY_SKIP_ORIGINAL
        };
        write_bytes(Function::SetEvent, bytes)
    }
}

pub fn is_ivory_gauntlet_skip() -> bool {
    if is_32() {
        read::<[u8; 6]>(Function::SetEvent)
            .map(|val| val != VANILLA_IVORY_SKIP_ORIGINAL)
    } else {
        read::<[u8; 5]>(Function::SetEvent)
            .map(|val| val != SCHOLAR_IVORY_SKIP_ORIGINAL)
    }
    .unwrap_or_default()
}

const VANILLA_LOYCE_SKIP_ORIGINAL: [u8; 7] = [0x88, 0x94, 0x08, 0xA1, 0x02, 0x00, 0x00];
const SCHOLAR_LOYCE_SKIP_ORIGINAL: [u8; 8] = [0x44, 0x88, 0x84, 0x08, 0xA1, 0x03, 0x00, 0x00];
pub fn set_ivory_no_knights(state: bool) -> ProcResult {
    if state {
        let orig_instr_len = if is_32() { 7 } else { 8 };
        let mut fun = asm_function("ivory_knights");
        let mut asm = fun.take_bytes();

        write_rel_i32(
            &mut asm,
            CaveOffset::IvoryKnightsHook,
            fun.reloc("hook_loc"),
            Hook::SetSharedFlag.add_offset(orig_instr_len),
            4
        )?;
        install_hook(&asm, CaveOffset::IvoryKnightsHook, Hook::SetSharedFlag, orig_instr_len)
    } else {
        let bytes: &[u8] = if is_32() {
            &VANILLA_LOYCE_SKIP_ORIGINAL
        } else {
            &SCHOLAR_LOYCE_SKIP_ORIGINAL
        };
        write_bytes(Hook::SetSharedFlag, bytes)
    }
}

pub fn is_ivory_no_knights() -> bool {
    match is_scholar() {
        true => {
            read::<[u8; 8]>(Hook::SetSharedFlag)
                .map(|val| val != [0x44, 0x88, 0x84, 0x08, 0xA1, 0x03, 0x00, 0x00])
        }
        false => {
            read::<[u8; 7]>(Hook::SetSharedFlag)
                .map(|val| val != [0x88, 0x94, 0x08, 0xA1, 0x02, 0x00, 0x00])
        }
    }
    .unwrap_or_default()
}

impl EventFlag {
    pub fn get(&self) -> anyhow::Result<bool> {
        get_event_flag(*self as u32)
    }

    pub fn set(&self, state: bool) -> anyhow::Result<()> {
        set_event_flag(*self as u32, state)
    }

    pub fn get_flags(flags: &[Self]) -> anyhow::Result<bool> {
        for flag in flags {
            if !get_event_flag(*flag as u32)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub fn set_flags(flags: &[(Self, bool)]) -> anyhow::Result<()> {
        flags
            .iter()
            .try_for_each(|(flag, state)| set_event_flag(*flag as u32, *state))
    }

    pub fn set_area_conditional_event(&self, state: bool, area_id: MapId) -> anyhow::Result<()> {
        player_loaded_check()?;
        ensure!(
            utility::get_area_id()? == area_id as u32,
            "Must be in general area"
        );
        set_event_flag(*self as u32, state)
    }
}