use crate::{
    mem::*,
    offsets::{code_cave::CaveAddress, game_manager_imp, module_offsets::Function},
    pointer_cache::ResolvedPtr,
    resources::{
        asm_function,
        menus::{MenuType, Shop, Trade},
    },
    utils::player_loaded_check,
};
use gubtool_core::{address::Address, attached::is_32, slice_ops::*, sys::error::ProcResult};
use std::{thread, time::Duration};

pub fn open_shop(shop: Shop) -> anyhow::Result<()> {
    write::<u32>(CaveAddress::NpcTalkArgs.add_offset(0x4), shop as u32)?;
    write::<u32>(CaveAddress::NpcTalkArgs.add_offset(0x8), shop as u32 + 999)?;
    open_menu(MenuType::Shop)
}

pub fn open_trade(trade: Trade) -> anyhow::Result<()> {
    write::<u32>(CaveAddress::NpcTalkArgs.add_offset(0x14), trade as u32)?;
    write::<u32>(CaveAddress::NpcTalkArgs.add_offset(0x2C), trade as u32 + 999)?;
    open_menu(MenuType::Trading)
}

pub fn open_menu(menu_type: MenuType) -> anyhow::Result<()> {
    player_loaded_check()?;

    let args_loc = CaveAddress::OpenMenuArgs.addr();
    let npc_args_loc = CaveAddress::NpcTalkArgs.addr();

    if is_32() {
        write::<u32>(args_loc, npc_args_loc as u32)?;
        write::<u8>(args_loc + 0x4, menu_type as u8)?;
        write::<u32>(args_loc + 0x20, 0x1)?;
    } else {
        write::<u64>(args_loc, npc_args_loc)?;
        write::<u8>(args_loc + 0x8, menu_type as u8)?;
        write::<u64>(args_loc + 0x28, 0x1)?;
    }

    let mut fun = asm_function("open_menu");
    let mut asm = fun.take_bytes();

    write_addr_to_slice(&mut asm, fun.reloc("npc_pos"), CaveAddress::NpcPos)?;
    write_addr_to_slice(&mut asm, fun.reloc("args"), args_loc)?;
    write_addr_to_slice(&mut asm, fun.reloc("window_manager"), ResolvedPtr::WindowManager.get()?)?;
    write_addr_to_slice(&mut asm, fun.reloc("fn_open_menu"), Function::OpenMenu)?;

    set_menu_open_chr_state(true)?;
    spawn_thread_join(CaveAddress::OpenMenuAsm, asm)?;
    tokio::spawn(async {
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
    let mut fun = asm_function("menu_chr_state");
    let mut asm = fun.take_bytes();

    write_addr_to_slice(&mut asm, fun.reloc("dl_back_allocator"), ResolvedPtr::DlBackAllocator.get()?)?;
    write_to_slice::<u32>(&mut asm, fun.reloc("state"), state)?;
    write_addr_to_slice(&mut asm, fun.reloc("fn_menu_chr_state"), Function::MenuChrState)?;

    spawn_thread_join(CaveAddress::MenuChrStateAsm.addr(), asm)
}