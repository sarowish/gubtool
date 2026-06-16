pub mod chr_ctrl;
pub mod code_cave;
pub mod functions;
pub mod game_manager_imp;
pub mod hooks;
pub mod module_offsets;
pub mod patches;

use crate::{
    mem::{is_scholar, read, read_address, write},
    offsets::module_offsets::module_offsets,
};
use gubtool_core::{attached::module_base, sys::error::ProcResult};
use pelite::Pod;

pub struct Offset {
    vanilla: u64,
    scholar: u64,
}

impl Offset {
    #[inline(always)]
    pub fn resolve(&self) -> u64 {
        if is_scholar() { self.scholar } else { self.vanilla }
    }
}

pub trait ChainReadExt {
    fn read_offset(self, offset: Offset) -> ProcResult<u64>;
    fn add_offset(self, offset: Offset) -> ProcResult<u64>;
    fn read<T: Pod>(self) -> ProcResult<T>;
    fn write<T: Pod>(self, val: T) -> ProcResult;
}

impl ChainReadExt for ProcResult<u64> {
    fn read_offset(self, offset: Offset) -> ProcResult<u64> {
        let base = self?;
        read_address(base.saturating_add(offset.resolve()))
    }
    fn add_offset(self, offset: Offset) -> ProcResult<u64> {
        let base = self?;
        Ok(base.saturating_add(offset.resolve()))
    }
    fn read<T: Pod>(self) -> ProcResult<T> {
        let addr = self?;
        read::<T>(addr)
    }
    fn write<T: Pod>(self, val: T) -> ProcResult {
        let addr = self?;
        write::<T>(addr, val)
    }
}

pub fn kernel32_create_thread() -> u64 {
    module_base() + module_offsets().external_fn_ptrs.kernel32_create_thread
}

pub fn kernel32_close_handle() -> u64 {
    module_base() + module_offsets().external_fn_ptrs.kernel32_close_handle
}

pub fn kernel32_sleep() -> u64 {
    module_base() + module_offsets().external_fn_ptrs.kernel32_sleep
}

pub fn map_id() -> u64 {
    module_base() + module_offsets().data.map_id
}
