use crate::{
    mem::*,
    offsets::{
        ChainReadExt,
        code_cave::CaveOffset,
        functions,
        game_manager_imp::{
            self, event_manager_offsets,
        },
    },
    resources::{
        menus::{MenuType, Shop, Trade},
        scholar, vanilla,
    },
};
use gubtool_core::sys::error::ProcResult;
use utils::slice_ops::write_to_slice;
use std::{thread, time::Duration};

pub fn open_shop(shop: Shop) -> ProcResult {
    let npc_args_loc = CaveOffset::NpcTalkArgs.addr();
    write::<u32>(npc_args_loc + 0x4, shop as u32)?;
    write::<u32>(npc_args_loc + 0x8, shop as u32 + 999)?;
    open_menu(MenuType::Shop)
}

pub fn open_trade(trade: Trade) -> ProcResult {
    let npc_args_loc = CaveOffset::NpcTalkArgs.addr();
    write::<u32>(npc_args_loc + 0x14, trade as u32)?;
    write::<u32>(npc_args_loc + 0x2C, trade as u32 + 999)?;
    open_menu(MenuType::Trading)
}

pub fn open_menu(menu_type: MenuType) -> ProcResult {
    let args_loc = CaveOffset::OpenMenuArgs.addr();
    let npc_args_loc = CaveOffset::NpcTalkArgs.addr();
    let window_manager = read_address(game_manager_imp::base_ptr())
        .read_offset(game_manager_imp::EVENT_MANAGER)
        .read_offset(event_manager_offsets::EVENT_WINDOW_MANAGER)?;

    let asm = if is_scholar() {
        write::<u64>(args_loc, npc_args_loc)?;
        write::<u8>(args_loc + 0x8, menu_type as u8)?;
        write::<u64>(args_loc + 0x28, 0x1)?;

        let fun = scholar::ASM.get_function("open_menu");
        let mut asm = fun.get_bytes();
        write_to_slice::<u64>(&mut asm, fun.reloc("window_manager"), window_manager)?;
        write_to_slice::<u64>(&mut asm, fun.reloc("args"), args_loc)?;
        write_to_slice::<u64>(&mut asm, fun.reloc("npc_pos"), CaveOffset::NpcPos.addr())?;
        write_to_slice::<u64>(&mut asm, fun.reloc("fn_open_menu"), functions::open_menu())?;
        asm
    } else {
        write::<u32>(args_loc, npc_args_loc as u32)?;
        write::<u8>(args_loc + 0x4, menu_type as u8)?;
        write::<u32>(args_loc + 0x20, 0x1)?;

        let fun = vanilla::ASM.get_function("open_menu");
        let mut asm = fun.get_bytes();
        write_to_slice::<u32>(&mut asm, fun.reloc("window_manager"), window_manager)?;
        write_to_slice::<u32>(&mut asm, fun.reloc("args"), args_loc)?;
        write_to_slice::<u32>(&mut asm, fun.reloc("npc_pos"), CaveOffset::NpcPos.addr())?;
        write_to_slice::<u32>(&mut asm, fun.reloc("fn_open_menu"), functions::open_menu())?;
        asm
    };
    set_menu_open_chr_state(true)?;
    spawn_thread_join(CaveOffset::OpenMenuAsm.addr(), asm)?;
    thread::spawn(|| {
        while is_menu_open() {
            thread::sleep(Duration::from_millis(50));
        }
        set_menu_open_chr_state(false)
    });
    Ok(())
}

fn is_menu_open() -> bool {
    follow_pointers(&game_manager_imp::fe_item_select_menu_chain(), true).is_ok()
}

fn set_menu_open_chr_state(state: bool) -> ProcResult {
    let dl_back_allocator = get_dl_back_allocator()?;
    let asm = if is_scholar() {
        let fun = scholar::ASM.get_function("menu_chr_state");
        let mut asm = fun.get_bytes();
        write_to_slice::<u64>(&mut asm, fun.reloc("dl_back_allocator"), dl_back_allocator)?;
        write_to_slice::<u32>(&mut asm, fun.reloc("state"), state)?;
        write_to_slice::<u64>(&mut asm, fun.reloc("fn_menu_chr_state"), functions::menu_chr_state())?;
        asm
    } else {
        let fun = vanilla::ASM.get_function("menu_chr_state");
        let mut asm = fun.get_bytes();
        write_to_slice::<u32>(&mut asm, fun.reloc("dl_back_allocator"), dl_back_allocator)?;
        write_to_slice::<u32>(&mut asm, fun.reloc("state"), state)?;
        write_to_slice::<u32>(&mut asm, fun.reloc("fn_menu_chr_state"), functions::menu_chr_state())?;
        asm
    };
    spawn_thread_join(CaveOffset::MenuChrStateAsm.addr(), asm)
}

fn get_dl_back_allocator() -> ProcResult<u64> {
    read_address(game_manager_imp::base_ptr())
        .read_offset(game_manager_imp::DL_BACK_ALLOCATOR)
}