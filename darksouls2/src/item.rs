use crate::{
    mem::*,
    offsets::{
        self,
        code_cave::{
            CaveOffset, item_args_offsets::{self, *}, item_struct_offsets
        },
        functions, game_manager_imp,
    },
    resources::{
        items::{
            Categories, Item,
            armor::ARMOR,
            arrows::ARROWS,
            consumables::CONSUMABLES,
            gestures::GESTURES,
            infusions::{INFUSION_IDS, INFUSIONS, Infusions},
            key_items::KEY_ITEMS,
            rings::RINGS,
            spells::SPELLS,
            upgrade_materials::UPGRADE_MATERIALS,
            weapons::WEAPONS,
        },
        scholar, vanilla,
    },
    utils::{ScholarError, character_loaded_check},
};
use anyhow::{Result, bail};
use shared::slice_ops::*;
use std::{thread, time::Duration};


fn itemspawn(
    item_id: i32,
    stack_size: i32,
    durability: i32,
    quantity: i32,
    upgrade: i32,
    infusion: i32,
) -> Result<()> {
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

    let _handle = ITEM_SPAWN_MUTEX.lock().unwrap();

    let location = CaveOffset::ItemSpawnAsm.addr();
    let stack_loc = CaveOffset::ItemSpawnStack.addr();
    let args_loc = CaveOffset::ItemArgs.addr();
    write_bytes(args_loc, &args)?;

    if read::<u8>(location)? == 0x0 {
        match is_scholar() {
            true => write_item_code_scholar(location, stack_loc, args_loc)?,
            false => write_item_code_vanilla(location, stack_loc, args_loc)?,
        }
        run_thread_release(location)?
    }
    Ok(())
}

fn write_item_code_scholar(location: u64, stack_loc: u64, args_loc: u64) -> Result<()> {
    let item_struct = args_loc + ITEM_STRUCT;
    use item_struct_offsets as off;

    let mut asm = scholar::ASM.get_function("item_spawn").get_bytes();

    write_to_slice::<u64>(&mut asm, 15, game_manager_imp::base_ptr())?;
    write_to_slice::<u64>(&mut asm, 87, functions::current_item_quantity_check())?;
    write_to_slice::<u64>(&mut asm, 175, functions::item_spawn())?;
    write_to_slice::<u64>(&mut asm, 215, functions::build_item_dialogue())?;
    write_to_slice::<u64>(&mut asm, 242, functions::show_item_dialogue())?;
    write_to_slice::<u64>(&mut asm, 262, read::<u64>(offsets::kernel32_sleep())?)?;
    write_rel_i32(&mut asm, location, 2, args_loc + SHOULD_PROCESS_FLAG, 5)?;
    write_rel_i32(&mut asm, location, 46, args_loc + SHOULD_PROCESS_FLAG, 5)?;
    write_rel_i32(&mut asm, location, 53, args_loc + ADJUST_QUANTITY_FLAG, 5)?;
    write_rel_i32(&mut asm, location, 67, args_loc + CURRENT_QUANTITY, 4)?;
    write_rel_i32(&mut asm, location, 74, args_loc + STACK_COUNT, 4)?;
    write_rel_i32(&mut asm, location, 81, item_struct + off::ITEM_ID, 4)?;
    write_rel_i32(&mut asm, location, 109, item_struct + off::QUANTITY, 4)?;
    write_rel_i32(&mut asm, location, 115, args_loc + CURRENT_QUANTITY, 4)?;
    write_rel_i32(&mut asm, location, 121, args_loc + MAX_QUANTITY, 4)?;
    write_rel_i32(&mut asm, location, 129, args_loc + MAX_QUANTITY, 4)?;
    write_rel_i32(&mut asm, location, 135, args_loc + CURRENT_QUANTITY, 4)?;
    write_rel_i32(&mut asm, location, 142, item_struct + off::QUANTITY, 4)?;
    write_rel_i32(&mut asm, location, 159, item_struct, 4)?;
    write_rel_i32(&mut asm, location, 166, args_loc + ITEM_COUNT, 4)?;
    write_rel_i32(&mut asm, location, 189, stack_loc, 4)?;
    write_rel_i32(&mut asm, location, 196, item_struct, 4)?;
    write_rel_i32(&mut asm, location, 203, args_loc + ITEM_COUNT, 4)?;
    write_rel_i32(&mut asm, location, 236, stack_loc, 4)?;
    write_rel_i32(&mut asm, location, 281, args_loc + SHOULD_EXIT_FLAG, 5)?;

    write_bytes(location, &asm)
}

fn write_item_code_vanilla(location: u64, stack_loc: u64, args_loc: u64) -> Result<()> {
    let item_struct = args_loc + ITEM_STRUCT;
    use item_struct_offsets as off;

    let mut asm = vanilla::ASM.get_function("item_spawn").get_bytes();

    write_to_slice::<u32>(&mut asm, 2, args_loc + SHOULD_PROCESS_FLAG)?;
    write_to_slice::<u32>(&mut asm, 15, game_manager_imp::base_ptr())?;
    write_to_slice::<u32>(&mut asm, 32, args_loc + SHOULD_PROCESS_FLAG)?;
    write_to_slice::<u32>(&mut asm, 39, args_loc + ADJUST_QUANTITY_FLAG)?;
    write_to_slice::<u32>(&mut asm, 51, item_struct + off::ITEM_ID)?;
    write_to_slice::<u32>(&mut asm, 58, args_loc + STACK_COUNT)?;
    write_to_slice::<u32>(&mut asm, 65, args_loc + CURRENT_QUANTITY)?;
    write_to_slice::<u32>(&mut asm, 78, item_struct + off::QUANTITY)?;
    write_to_slice::<u32>(&mut asm, 84, args_loc + CURRENT_QUANTITY)?;
    write_to_slice::<u32>(&mut asm, 90, args_loc + MAX_QUANTITY)?;
    write_to_slice::<u32>(&mut asm, 97, args_loc + MAX_QUANTITY)?;
    write_to_slice::<u32>(&mut asm, 103, args_loc + CURRENT_QUANTITY)?;
    write_to_slice::<u32>(&mut asm, 109, item_struct + off::QUANTITY)?;
    write_to_slice::<u32>(&mut asm, 119, args_loc + ITEM_COUNT)?;
    write_to_slice::<u32>(&mut asm, 125, item_struct)?;
    write_to_slice::<u32>(&mut asm, 139, args_loc + ITEM_COUNT)?;
    write_to_slice::<u32>(&mut asm, 145, item_struct)?;
    write_to_slice::<u32>(&mut asm, 152, stack_loc)?;
    write_to_slice::<u32>(&mut asm, 173, stack_loc)?;
    write_to_slice::<u32>(&mut asm, 184, offsets::kernel32_sleep())?;
    write_to_slice::<u32>(&mut asm, 194, args_loc + SHOULD_EXIT_FLAG)?;
    write_rel_i32(&mut asm, location, 71, functions::current_item_quantity_check(), 4)?;
    write_rel_i32(&mut asm, location, 131, functions::item_spawn(), 4)?;
    write_rel_i32(&mut asm, location, 158, functions::build_item_dialogue(), 4)?;
    write_rel_i32(&mut asm, location, 179, functions::show_item_dialogue(), 4)?;

    write_bytes(location, &asm)
}

pub fn mass_spawn(category: Categories) -> Result<()> {
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
    pub fn spawn(&self, quantity: i32, upgrade: i32, infusion: i32) -> Result<()> {
        character_loaded_check()?;
        if !is_scholar() && self.scholar_only {
            bail!(ScholarError)
        }
        itemspawn(
            self.id,
            self.stack_size,
            self.durability.unwrap_or(0),
            quantity,
            upgrade,
            infusion,
        )
    }
    pub fn available_infusions(&self) -> Vec<Infusions> {
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