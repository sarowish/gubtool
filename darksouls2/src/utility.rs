use crate::{
    mem::*,
    offsets::{
        ChainReadExt,
        code_cave::CaveOffset,
        game_manager_imp::{
            self,
            game_data_manager_offsets::{self, clearcount_ptr_offsets},
        },
        module_offsets::{BasePointer, Data, Hook, Patch},
    },
    resources::asm_function,
};
use gubtool_core::slice_ops::*;
use gubtool_core::{address::Address, attached::is_32, sys::error::ProcResult};

pub fn quitout() -> ProcResult {
    read_address(BasePointer::GameManagerImp)
        .add_offset(game_manager_imp::QUITOUT)
        .write::<u8>(0x6)
}

pub fn get_area_id() -> ProcResult<u32> {
    read::<u32>(Data::MapId)
}

pub fn get_ng() -> ProcResult<u8> {
    read_address(BasePointer::GameManagerImp)
        .read_offset(game_manager_imp::GAME_DATA_MANAGER)
        .read_offset(game_data_manager_offsets::CLEARCOUNT_PTR)
        .add_offset(clearcount_ptr_offsets::CLEARCOUNT)
        .read::<u8>()
}

pub fn set_ng(count: u8) -> ProcResult {
    read_address(BasePointer::GameManagerImp)
        .read_offset(game_manager_imp::GAME_DATA_MANAGER)
        .read_offset(game_data_manager_offsets::CLEARCOUNT_PTR)
        .add_offset(clearcount_ptr_offsets::CLEARCOUNT)
        .write::<u8>(count)
}

const VANILLA_MENU_PATCH_ORIGINAL: [u8; 2] = [0x0F, 0x85];
const SCHOLAR_MENU_PATCH_ORIGINAL: [u8; 2] = [0x75, 0xEA];
pub fn set_faster_menu(state: bool) -> ProcResult {
    if state {
        let orig_instr_len = if is_32() { 5 } else { 8 };
        let patch_bytes = if is_32() { [0x0F, 0x84] } else { [0x74, 0xEA] };
        write_bytes(Patch::MenuTransition, &patch_bytes)?;

        let mut fun = asm_function("faster_menu");
        let mut asm = fun.take_bytes();
        write_rel_i32(
            &mut asm,
            CaveOffset::FasterMenuHook,
            fun.reloc("hook_loc"),
            Hook::FasterMenu.add_offset(orig_instr_len),
            4
        )?;
        install_hook(&asm, CaveOffset::FasterMenuHook, Hook::FasterMenu, orig_instr_len)
    } else {
        let patch_orig = if is_32() {
            &VANILLA_MENU_PATCH_ORIGINAL
        } else {
            &SCHOLAR_MENU_PATCH_ORIGINAL
        };
        let hook_orig: &[u8] = if is_32() {
            &[0x33, 0xC5, 0x89, 0x45, 0xFC]
        } else {
            &[0x48, 0x89, 0x84, 0x24, 0x50, 0x01, 0x00, 0x0]
        };
        write_bytes(Patch::MenuTransition, patch_orig)?;
        write_bytes(Hook::FasterMenu, hook_orig)
    }
}

pub fn is_faster_menu() -> bool {
    let patch_orig = if is_32() {
        VANILLA_MENU_PATCH_ORIGINAL
    } else {
        SCHOLAR_MENU_PATCH_ORIGINAL
    };
    read::<[u8; 2]>(Patch::MenuTransition)
        .map(|val| val != patch_orig)
        .unwrap_or_default()
}

const VANILLA_CREDITS_ORIGINAL: [u8; 6] = [0x81, 0xEC, 0xFC, 0x01, 0x00, 0x00];
const SCHOLAR_CREDITS_ORIGINAL: [u8; 7] = [0x48, 0x81, 0xEC, 0x20, 0x02, 0x00, 0x00];
pub fn set_credits_skip(state: bool) -> ProcResult {
    if state {
    let orig_instr_len = if is_32() { 6 } else { 7 };
    let modify_once = CaveOffset::CreditsModifyOnceFlag;
    let mut fun = asm_function("credits_skip");
    let mut asm = fun.take_bytes();

    write_addr_to_slice(&mut asm, fun.reloc("modify_once_flag"), modify_once)?;
    write_rel_i32(
        &mut asm,
        CaveOffset::CreditsSkipHook,
        fun.reloc("hook_loc"),
        Hook::CreditsSkip.add_offset(orig_instr_len),
        4
    )?;

    write::<u8>(modify_once, 0x0)?;
    install_hook(&asm, CaveOffset::CreditsSkipHook, Hook::CreditsSkip, orig_instr_len)

    } else {
        let bytes: &[u8] = if is_32() {
            &VANILLA_CREDITS_ORIGINAL
        } else {
            &SCHOLAR_CREDITS_ORIGINAL
        };
        write_bytes(Hook::CreditsSkip, bytes)
    }
}

pub fn is_credits_skip() -> bool {
    if is_scholar() {
        read::<[u8; 7]>(Hook::CreditsSkip)
            .map(|val| val != [0x48, 0x81, 0xEC, 0x20, 0x02, 0x00, 0x00])
    } else {
        read::<[u8; 6]>(Hook::CreditsSkip)
            .map(|val| val != [0x81, 0xEC, 0xFC, 0x01, 0x00, 0x00])
    }
    .unwrap_or_default()
}

const DISABLE_ROLL_ORIGINAL: [u8; 2] = [0xB0, 0x01];
pub fn set_disable_roll(state: bool) -> ProcResult {
    let bytes = if state {
        [0x30, 0xC0]
    } else {
        DISABLE_ROLL_ORIGINAL
    };
    write_bytes(Patch::NoRoll, &bytes)
}

pub fn is_disable_roll() -> bool {
    read::<[u8; 2]>(Patch::NoRoll)
        .map(|val| val != DISABLE_ROLL_ORIGINAL)
        .unwrap_or_default()
}

const DISABLE_BACKSTEP_ORIGINAL: [u8; 3] = [0x0F, 0x95, 0xC0];
pub fn set_disable_backstep(state: bool) -> ProcResult {
    let bytes = if state {
        [0x30, 0xC0, 0x90]
    } else {
        DISABLE_BACKSTEP_ORIGINAL
    };
    write_bytes(Patch::NoBackstep, &bytes)
}

pub fn is_disable_backstep() -> bool {
    read::<[u8; 3]>(Patch::NoBackstep)
        .map(|val| val != DISABLE_BACKSTEP_ORIGINAL)
        .unwrap_or_default()
}