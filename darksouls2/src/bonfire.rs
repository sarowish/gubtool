use crate::{
    mem::*,
    offsets::{
        ChainReadExt, code_cave::CaveOffset, functions, game_manager_imp::{
            self,
            event_manager_offsets::{self, bonfire_manager_offsets},
        }
    },
    resources::{
        bonfires::Bonfire,
        scholar, vanilla,
    },
};
use gubtool_core::sys::error::ProcResult;
use utils::slice_ops::write_to_slice;

impl Bonfire {
    pub fn unlock(&self) -> ProcResult {
        light_bonfire(self.bonfire_id)
    }
    pub fn light(&self) -> ProcResult {
        light_bonfire(self.bonfire_id)
    }
    pub fn rest(&self) -> ProcResult {
        rest_at_bonfire(self)
    }
    pub fn is_lit(&self) -> ProcResult<bool> {
        is_bonfire_lit(self.bonfire_id)
    }
}

pub fn get_last_bonfire_id() -> ProcResult<u32> {
    read_address(game_manager_imp::base_ptr())
        .read_offset(game_manager_imp::EVENT_MANAGER)
        .add_offset(event_manager_offsets::RESPAWN_BONFIRE)
        .read::<u32>()
}

pub fn light_all_bonfires() -> ProcResult {
    let bonfire_manager = get_bonfire_manager()?;
    let asm = if is_scholar() {
        let fun = scholar::ASM.get_function("bonfire_unlock_all");
        let mut asm = fun.get_bytes();
        write_to_slice::<u64>(&mut asm, fun.reloc("bonfire_manager"), bonfire_manager)?;
        write_to_slice::<u64>(&mut asm, fun.reloc("fn_bonfire_unlock"), functions::bonfire_unlock())?;
        asm
    } else {
        let fun = vanilla::ASM.get_function("bonfire_unlock_all");
        let mut asm = fun.get_bytes();
        write_to_slice::<u32>(&mut asm, fun.reloc("bonfire_manager"), bonfire_manager)?;
        write_to_slice::<u32>(&mut asm, fun.reloc("fn_bonfire_unlock"), functions::bonfire_unlock())?;
        asm
    };
    spawn_thread_join(CaveOffset::BonfireUnlockAllAsm.addr(), asm)
}


fn light_bonfire(bonfire_id: u32) -> ProcResult {
    let bonfire_manager = get_bonfire_manager()?;
    let asm = if is_scholar() {
        let fun = scholar::ASM.get_function("bonfire_unlock");
        let mut asm = fun.get_bytes();
        write_to_slice::<u64>(&mut asm, fun.reloc("bonfire_manager"), bonfire_manager)?;
        write_to_slice::<u32>(&mut asm, fun.reloc("bonfire_id"), bonfire_id)?;
        write_to_slice::<u64>(&mut asm, fun.reloc("fn_bonfire_unlock"), functions::bonfire_unlock())?;
        asm
    } else {
        let fun = vanilla::ASM.get_function("bonfire_unlock");
        let mut asm = fun.get_bytes();
        write_to_slice::<u32>(&mut asm, fun.reloc("bonfire_manager"), bonfire_manager)?;
        write_to_slice::<u32>(&mut asm, fun.reloc("bonfire_id"), bonfire_id)?;
        write_to_slice::<u32>(&mut asm, fun.reloc("fn_bonfire_unlock"), functions::bonfire_unlock())?;
        asm
    };
    spawn_thread_join(CaveOffset::BonfireUnlockAsm.addr(), asm)
}

fn is_bonfire_lit(bonfire_id: u32) -> ProcResult<bool> {
    let Some(addr) = bonfire_handle_from_id(bonfire_id)? else {
        return Ok(false)
    };
    read::<u8>(addr + 0x2)
        .map(|val| val != 0)
}

fn bonfire_handle_from_id(bonfire_id: u32) -> ProcResult<Option<u64>> {
    let bonfire_manager = get_bonfire_manager()?;
    let size = if is_scholar() { 0x18 } else { 0x10 };

    let array_ptr = read_address(bonfire_manager + bonfire_manager_offsets::ARRAY_BASE.resolve())?;
    let mut high = read::<i32>(bonfire_manager + bonfire_manager_offsets::COUNT.resolve())? - 1;
    let mut low = 0;

    while low <= high {
        let mid = low + ((high - low) >> 1);
        let entry_id = read::<u16>(array_ptr + (mid as u64) * size)? as u32;

        if bonfire_id < entry_id {
            if mid == 0 { break; }
            high = mid - 1;
        } else if bonfire_id > entry_id {
            low = mid + 1;
        } else {
            return Ok(Some(array_ptr + (mid as u64) * size))
        }
    }
    Ok(None)
}

fn rest_at_bonfire(bonfire: &Bonfire) -> ProcResult {
    let bonfire_manager = get_bonfire_manager()?;
    let respawn_map_loc = read_address(game_manager_imp::base_ptr())
        .read_offset(game_manager_imp::EVENT_MANAGER)
        .add_offset(event_manager_offsets::RESPAWN_MAP)?;

    let asm = if is_scholar() {
        let fun = scholar::ASM.get_function("bonfire_rest");
        let mut asm = fun.get_bytes();
        write_to_slice::<u32>(&mut asm, fun.reloc("bonfire_id"), bonfire.bonfire_id)?;
        write_to_slice::<u64>(&mut asm, fun.reloc("bonfire_manager"), bonfire_manager)?;
        write_to_slice::<u64>(&mut asm, fun.reloc("fn_bonfire_rest"), functions::bonfire_rest())?;
        asm
    } else {
        let fun = vanilla::ASM.get_function("bonfire_rest");
        let mut asm = fun.get_bytes();
        write_to_slice::<u32>(&mut asm, fun.reloc("bonfire_id"), bonfire.bonfire_id)?;
        write_to_slice::<u32>(&mut asm, fun.reloc("bonfire_manager"), bonfire_manager)?;
        write_to_slice::<u32>(&mut asm, fun.reloc("fn_bonfire_rest"), functions::bonfire_rest())?;
        asm
    };
    let has_rested = 0x0;
    write::<[u32; 3]>(respawn_map_loc, [bonfire.map_id as u32, has_rested, bonfire.bonfire_id])?;
    spawn_thread_join(CaveOffset::BonfireRestAsm.addr(), asm)
}

fn get_bonfire_manager() -> ProcResult<u64> {
    read_address(game_manager_imp::base_ptr())
        .read_offset(game_manager_imp::EVENT_MANAGER)
        .read_offset(event_manager_offsets::EVENT_BONFIRE_MANAGER)
}