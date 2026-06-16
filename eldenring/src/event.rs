use crate::{
    chr_ins::ChrInsExt, event, mem::*, offsets::{ChainReadExt, code_cave::CaveOffset, functions, virtual_memory_flag}, player, resources::{
        ASM,
        bosses::{Boss, bosses_array},
        talk_commands::TalkCommand,
    }, utils::{character_loaded_check, dlc_check}
};
use anyhow::{anyhow, ensure};
use gubtool_core::sys::error::ProcResult;
use shared::event_log::{EventLog, EventLogger};
use utils::slice_ops::*;

pub fn get_event(event_id: u32) -> anyhow::Result<bool> {
    let (data_ptr, block_offset) = event_flag_lookup(event_id)?;
    let mask = 1 << (7 - (block_offset & 7));
    Ok(is_bit_set(data_ptr + (block_offset >> 3) as u64, mask)?)
}

pub fn _set_event_direct(event_id: u32, state: bool) -> anyhow::Result<()> {
    let (data_ptr, block_offset) = event_flag_lookup(event_id)?;
    let mask = 1 << (7 - (block_offset & 7));
    Ok(set_bit(data_ptr + (block_offset >> 3) as u64, mask, state)?)
}

pub fn set_event(event_id: u32, state: bool) -> ProcResult {
    let virt_mem_flag = read::<u64>(virtual_memory_flag::base_ptr())?;

    let fun = ASM.get_function("set_event");
    let mut asm = fun.get_bytes();

    write_to_slice::<i64>(&mut asm, fun.reloc("virt_mem_flag"), virt_mem_flag)?;
    write_to_slice::<i64>(&mut asm, fun.reloc("event_id"), event_id)?;
    write_to_slice::<i64>(&mut asm, fun.reloc("state"), state)?;
    write_to_slice::<i64>(&mut asm, fun.reloc("fn_set_event"), functions::set_event())?;

    spawn_thread_join(CaveOffset::SetEventAsm.addr(), asm)
}

struct VirtMemInfo {
    block_size: u32,
    stride: u32,
    mem_base: u64,
    lookup_tree_root: u64,
}

impl VirtMemInfo {
    pub fn read() -> ProcResult<Self> {
        let bytes = read::<u64>(virtual_memory_flag::base_ptr())
            .read::<[u8; 0x40]>()?;
        Ok(Self {
            block_size: read_from_slice::<u32>(&bytes, 0x1C)?,
            stride: read_from_slice::<u32>(&bytes, 0x20)?,
            mem_base: read_from_slice::<u64>(&bytes, 0x28)?,
            lookup_tree_root: read_from_slice::<u64>(&bytes, 0x38)?,
        })
    }
}

#[derive(Clone, Copy)]
struct Node {
    left_child: u64,
    right_child: u64,
    is_leaf: bool,
    block_idx: u32,
    block_type: u32,
    data_idx: u32,
}

impl Node {
    fn read_at(address: u64) -> ProcResult<Self> {
        let bytes = read::<[u8; 0x34]>(address)?;
        Ok(Self {
            left_child: read_from_slice::<u64>(&bytes, 0x0)?,
            right_child: read_from_slice::<u64>(&bytes, 0x10)?,
            is_leaf: read_from_slice::<u8>(&bytes, 0x19)? != 0x0,
            block_idx: read_from_slice::<u32>(&bytes, 0x20)?,
            block_type: read_from_slice::<u32>(&bytes, 0x28)?,
            data_idx: read_from_slice::<u32>(&bytes, 0x30)?,
        })
    }
}

fn event_flag_lookup(event_id: u32) -> anyhow::Result<(u64, u32)> {
    let virt_mem_info = VirtMemInfo::read()?;
    let block_idx = event_id / virt_mem_info.block_size;
    let block_offset = event_id % virt_mem_info.block_size;

    let mut last_valid_node: Option<Node> = None;
    let mut current_node_ptr = read::<u64>(virt_mem_info.lookup_tree_root + 0x8)?;

    loop {
        let current_node = Node::read_at(current_node_ptr)?;

        if current_node.is_leaf {
            break;
        }

        if current_node.block_idx < block_idx {
            current_node_ptr = current_node.right_child;
        } else {
            last_valid_node = Some(current_node);
            current_node_ptr = current_node.left_child;
        };
    }
    if let Some(node) = last_valid_node && node.block_idx <= block_idx {
        let data_ptr = match node.block_type {
            1 => node.data_idx as u64 * virt_mem_info.stride as u64 + virt_mem_info.mem_base,
            2 => node.data_idx as u64,
            _ => anyhow::bail!("block type invalid")
        };
        ensure!(data_ptr != 0x0, "block pointer is null");
        return Ok((data_ptr, block_offset));
    }
    Err(anyhow!("flag not found"))
}

pub fn execute_talk_command(command_id: i32, params: &'static [i32], handle: u64) -> ProcResult {
    let location = CaveOffset::EzStateTalkAsm.addr();
    let params_location = CaveOffset::EzStateParams.addr();
    let params: Vec<u8> = params.iter().flat_map(|&x| x.to_le_bytes()).collect();

    let fun = ASM.get_function("execute_talk_command");
    let mut asm = fun.get_bytes();
    write_to_slice::<i32>(&mut asm, 18, command_id)?;
    write_rel_i32(&mut asm, location, 23, functions::external_event_temp_ctor(), 4)?;
    write_to_slice::<u64>(&mut asm, 65, handle)?;
    write_to_slice::<i32>(&mut asm, 78, params.len())?;
    write_rel_i32(&mut asm, location, 93, params_location, 4)?;
    write_rel_i32(&mut asm, location, 155, functions::execute_talk_command(), 4)?;

    write_bytes(params_location, &params)?;
    spawn_thread_join(location, asm)
}

impl TalkCommand {
    pub fn execute(&self) -> ProcResult {
        let handle = match self.handle {
            Some(function) => function()?,
            None => 0,
        };
        if self.command_id == 24 {
            execute_talk_command(49, &[6001, 232], 0)?;
            execute_talk_command(49, &[6001, 233], 0)?;
            execute_talk_command(49, &[6001, 234], 0)?;
            execute_talk_command(49, &[6001, 235], 0)?;
        }
        execute_talk_command(self.command_id, self.params, handle)
    }
}

#[derive(Default)]
pub struct ErEventLogger {
    event_log: EventLog,
}

impl EventLogger for ErEventLogger {
    fn event_log(&self) -> &EventLog {
        &self.event_log
    }
    fn event_log_mut(&mut self) -> &mut EventLog {
        &mut self.event_log
    }
    fn file_prefix(&self) -> &'static str {
        "eldenring"
    }
    fn write_idx(&self) -> ProcResult<i32> {
        read::<i32>(CaveOffset::EventLogWriteIdx.addr())
    }
    fn read_buffer(&self) -> ProcResult<[u8; 0x1000]> {
        read::<[u8; 0x1000]>(CaveOffset::EventLogBuffer.addr())
    }
    fn clear_cave(&self) -> ProcResult {
        write::<i32>(CaveOffset::EventLogWriteIdx.addr(), 0x0)?;
        write_bytes(CaveOffset::EventLogBuffer.addr(), &[0x0; 0x1000])
    }
}

const EVENT_LOG_HOOK_ORIGINAL: [u8; 5] = [0x48, 0x89, 0x5C, 0x24, 0x08];

pub fn set_event_log_hook(state: bool) -> ProcResult {
    match state {
        true => install_event_log_hook(),
        false => write_bytes(functions::set_event(), &EVENT_LOG_HOOK_ORIGINAL),
    }
}

pub fn is_event_log_hook() -> ProcResult<bool> {
    read::<[u8; 5]>(functions::set_event())
        .map(|bytes| bytes != EVENT_LOG_HOOK_ORIGINAL)
}

fn install_event_log_hook() -> ProcResult {
    let location = CaveOffset::EventLogHook.addr();
    let write_index = CaveOffset::EventLogWriteIdx.addr();
    let buffer = CaveOffset::EventLogBuffer.addr();

    let fun = ASM.get_function("event_log");
    let mut asm = fun.get_bytes();

    write_rel_i32(&mut asm, location, fun.reloc("write_index_1"), write_index, 4)?;
    write_rel_i32(&mut asm, location, fun.reloc("buffer"), buffer, 4)?;
    write_rel_i32(&mut asm, location, fun.reloc("write_index_2"), write_index, 4)?;
    write_rel_i32(&mut asm, location, fun.reloc("hook_loc"), functions::set_event() + 5, 4)?;

    install_hook(&asm, location, functions::set_event(), 5)
}

pub fn fight_fortissax() -> anyhow::Result<()> {
    ensure!(
        player::player_ins().block_id()? == 201523200,
        "Must be in general area"
    );
    set_event(12032859, true)?;
    Ok(())
}

pub fn fight_elden_beast() -> anyhow::Result<()> {
    ensure!(
        player::player_ins().block_id()? == 318767104,
        "Must be in general area"
    );
    set_event(19002802, true)?;
    set_event(19002805, true)?;
    Ok(())
}

pub fn set_dlc_clear(state: bool) -> anyhow::Result<()> {
    character_loaded_check()?;
    dlc_check()?;
    set_event(70, state)?;
    Ok(())
}

pub fn get_dlc_clear() -> anyhow::Result<bool> {
    dlc_check()?;
    Ok(get_event(70)?)
}

pub fn unlock_metyr() -> anyhow::Result<()> {
    character_loaded_check()?;
    dlc_check()?;
    let events = [
        2050400600, 2053460600, 2051459226, 2051459228, 2051459229, 2051459230, 2051455023,
        2051459249, 2051452717, 2050407000, 400662, 4856, 4855, 4854, 4849, 2051452718, 2051459213,
        2051450715, 9440, 2051450180,
    ];
    events.iter().try_for_each(|&i| set_event(i, true))?;
    Ok(())
}

pub const DEAD: &str = "Dead";
pub const ALIVE: &str = "Alive";
pub const ALIVE_SE: &str = "Alive (Second Encounter)";

impl Boss {
    pub fn revive(&self, first_encounter: bool, warp: bool) -> anyhow::Result<()> {
        character_loaded_check()?;
        if self.dlc {
            dlc_check()?;
        }
        if first_encounter {
            self.fe_flags
                .iter()
                .try_for_each(|(id, state)| set_event(*id, *state))?;
        }
        self.flags
            .iter()
            .try_for_each(|(id, state)| set_event(*id, *state))?;
        if warp {
            self.warp()?
        }
        Ok(())
    }
    pub fn revive_status(&self) -> &str {
        if event::get_event(self.flags[0].0).unwrap_or_default() {
            return DEAD;
        }
        if self
            .fe_flags
            .iter()
            .all(|x| event::get_event(x.0).unwrap_or_default() == x.1)
        {
            ALIVE
        } else {
            ALIVE_SE
        }
    }
}

pub fn mass_revive(dlc: bool, first_encounter: bool) -> anyhow::Result<()> {
    bosses_array(dlc)
        .iter()
        .try_for_each(|boss| boss.revive(first_encounter, false))
}