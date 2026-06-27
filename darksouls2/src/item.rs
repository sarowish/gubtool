use crate::{
    mem::*,
    offsets::{
        code_cave::{
            CaveOffset,
            item_args_offsets::{self, *},
            item_struct_offsets,
        },
        module_offsets::{BasePointer, ExternalFunctionPointer, Function},
    },
    resources::{
        asm_function,
        items::{
            Categories, Item,
            armor::ARMOR,
            arrows::ARROWS,
            consumables::CONSUMABLES,
            gestures::GESTURES,
            infusions::{INFUSION_IDS, INFUSIONS, Infusion},
            key_items::KEY_ITEMS,
            rings::RINGS,
            spells::SPELLS,
            upgrade_materials::UPGRADE_MATERIALS,
            weapons::WEAPONS,
        },
    },
    utils::{ScholarError, player_loaded_check},
};
use gubtool_core::slice_ops::*;
use gubtool_core::{address::Address, sys::error::ProcResult};
use std::{thread, time::Duration};


pub fn itemspawn(
    item_id: i32,
    stack_size: i32,
    durability: i32,
    quantity: i32,
    upgrade: i32,
    infusion: i32,
) -> ProcResult {
    let mut args: [u8; 35] = [0x0; 35];
    write_to_slice::<i32>(&mut args, item_args_offsets::CURRENT_QUANTITY, 0)?;
    write_to_slice::<i32>(&mut args, item_args_offsets::STACK_COUNT, 0)?;
    write_to_slice::<i32>(&mut args, item_args_offsets::MAX_QUANTITY, stack_size)?;
    write_to_slice::<i32>(&mut args, item_args_offsets::ITEM_COUNT, 1)?;
    write_to_slice::<u8>(&mut args, item_args_offsets::ADJUST_QUANTITY_FLAG, stack_size > 1)?;
    write_to_slice::<u8>(&mut args, item_args_offsets::SHOULD_PROCESS_FLAG, 1)?;

    let item_struct = item_args_offsets::ITEM_STRUCT;
    write_to_slice::<i32>(&mut args, item_struct + item_struct_offsets::ITEM_ID, item_id)?;
    write_to_slice::<f32>(&mut args, item_struct + item_struct_offsets::DURABILITY, durability as f32)?;
    write_to_slice::<i16>(&mut args, item_struct + item_struct_offsets::QUANTITY, quantity)?;
    write_to_slice::<u8>(&mut args, item_struct + item_struct_offsets::UPGRADE, upgrade)?;
    write_to_slice::<u8>(&mut args, item_struct + item_struct_offsets::INFUSION, infusion)?;

    let args_loc = CaveOffset::ItemArgs.addr();
    write_bytes(args_loc, &args)?;

    if read::<u8>(CaveOffset::ItemSpawnAsm)? == 0x0 {
        let item_struct = args_loc + ITEM_STRUCT;
        use item_struct_offsets as off;

        let mut fun = asm_function("item_spawn");
        let mut asm = fun.take_bytes();

        write_addr_to_slice(&mut asm, fun.reloc("should_process_flag"), args_loc + SHOULD_PROCESS_FLAG)?;
        write_addr_to_slice(&mut asm, fun.reloc("game_man_imp"), BasePointer::GameManagerImp)?;
        write_addr_to_slice(&mut asm, fun.reloc("adjust_quantity_flag"), args_loc + ADJUST_QUANTITY_FLAG)?;
        write_addr_to_slice(&mut asm, fun.reloc("item_id"), item_struct + off::ITEM_ID)?;
        write_addr_to_slice(&mut asm, fun.reloc("stack_count"), args_loc + STACK_COUNT)?;
        write_addr_to_slice(&mut asm, fun.reloc("current_quantity"), args_loc + CURRENT_QUANTITY)?;
        write_addr_to_slice(&mut asm, fun.reloc("fn_current_item_quantity_check"), Function::CurrentItemQuantityCheck)?;
        write_addr_to_slice(&mut asm, fun.reloc("quantity"), item_struct + off::QUANTITY)?;
        write_addr_to_slice(&mut asm, fun.reloc("current_quantity"), args_loc + CURRENT_QUANTITY)?;
        write_addr_to_slice(&mut asm, fun.reloc("max_quantity"), args_loc + MAX_QUANTITY)?;
        write_addr_to_slice(&mut asm, fun.reloc("item_count"), args_loc + ITEM_COUNT)?;
        write_addr_to_slice(&mut asm, fun.reloc("item_struct"), item_struct)?;
        write_addr_to_slice(&mut asm, fun.reloc("fn_item_spawn"), Function::ItemSpawn)?;
        write_addr_to_slice(&mut asm, fun.reloc("stack_loc"), CaveOffset::ItemSpawnStack)?;
        write_addr_to_slice(&mut asm, fun.reloc("fn_build_item_dialogue"), Function::BuildItemDialogue)?;
        write_addr_to_slice(&mut asm, fun.reloc("fn_show_item_dialogue"), Function::ShowItemDialogue)?;
        write_addr_to_slice(&mut asm, fun.reloc("fn_sleep"), ExternalFunctionPointer::Kernel32Sleep)?;
        write_addr_to_slice(&mut asm, fun.reloc("should_exit_flag"), args_loc + SHOULD_EXIT_FLAG)?;

        spawn_thread_release(CaveOffset::ItemSpawnAsm, asm)?
    }
    Ok(())
}

pub fn mass_spawn(category: Categories) -> anyhow::Result<()> {
    let _handle = MASS_SPAWN_MUTEX.lock().unwrap();

    let items: &'static [Item] = match category {
            Categories::Armor => ARMOR,
            Categories::Arrows => ARROWS,
            Categories::Consumables => CONSUMABLES,
            Categories::Gestures => GESTURES,
            Categories::KeyItems => KEY_ITEMS,
            Categories::Rings => RINGS,
            Categories::Spells => SPELLS,
            Categories::UpgradeMaterials => UPGRADE_MATERIALS,
            Categories::Weapons => WEAPONS,
    };
    for item in items {
        if let Err(err) = item.spawn(1, 0, 0) &&
            !err.is::<ScholarError>() {
                return Err(err);
        }
        thread::sleep(Duration::from_millis(8));
    }
    Ok(())
}

impl Item {
    pub fn spawn(&self, quantity: i32, upgrade: i32, infusion: i32) -> anyhow::Result<()> {
        player_loaded_check()?;
        if !is_scholar() && self.scholar_only {
            Err(ScholarError)?
        }
        itemspawn(
            self.id,
            self.stack_size,
            self.durability.unwrap_or(0),
            quantity,
            upgrade,
            infusion,
        )?;
        Ok(())
    }
    pub fn available_infusions(&self) -> Vec<Infusion> {
        let mut infusions = Vec::new();
        if let Some(infusion_id) = self.infuse_id &&
        let Some(flags) = INFUSION_IDS.get(&infusion_id) {
            flags.iter().enumerate().for_each(|(idx, val)| {
                if *val == 1 {
                    infusions.push(INFUSIONS[idx]);
                }
            })
        }
        infusions
    }
}