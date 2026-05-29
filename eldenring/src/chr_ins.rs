use crate::{
    emevd,
    mem::*,
    offsets::{
        chr_ins::{self, *},
        code_cave::CaveOffset, functions, world_chr_man,
    },
    resources::{ASM, chr_names::CHR_NAMES, entity_ids},
    target,
    utils::{wait_for_cutscence_completion, wait_for_event},
};
use anyhow::{Result, anyhow, bail, ensure};
use shared::slice_ops::*;

pub type ChrIns = Result<u64>;

pub trait ChrInsExt {
    fn get_current_hp(&self) -> Result<i32>;
    fn get_max_hp(&self) -> Result<i32>;
    fn set_hp(&self, val: i32) -> Result<()>;
    fn get_hp_pct(&self) -> Result<f32>;
    fn set_hp_pct(&self, val: i32) -> Result<()>;
    fn set_no_death(&self, val: bool) -> Result<()>;
    fn is_no_death(&self) -> Result<bool>;
    fn set_no_damage(&self, val: bool) -> Result<()>;
    fn is_no_damage(&self) -> Result<bool>;
    fn get_current_poise(&self) -> Result<f32>;
    fn get_max_poise(&self) -> Result<f32>;
    fn get_poise_timer(&self) -> Result<f32>;
    fn get_current_animation(&self) -> Result<i32>;
    fn get_last_act(&self) -> Result<u8>;
    fn set_repeat_act(&self, val: bool) -> Result<()>;
    fn is_repeat_act(&self) -> Result<bool>;
    fn force_act(&self, act: i32) -> Result<()>;
    fn set_disable_ai(&self, state: bool) -> Result<()>;
    fn is_disable_ai(&self) -> Result<bool>;
    fn get_animation_speed(&self) -> Result<f32>;
    fn set_animation_speed(&self, val: f32) -> Result<()>;
    fn local_coords(&self) -> Result<[f32; 3]>;
    fn map_coords(&self) -> Result<[f32; 3]>;
    fn hurtbox_radius(&self) -> Result<f32>;
    fn get_distance(&self, other: &ChrIns) -> Result<f32>;
    fn set_speffect(&self, speffect_id: u32) -> Result<()>;
    fn remove_speffect(&self, speffect_id: u32) -> Result<()>;
    fn has_speffect(&self, speffect_id: u32) -> Result<bool>;
    fn reset_position(&self) -> Result<()>;
    fn get_lua_timers(&self) -> Result<[f32; 16]>;
    fn force_animation_playback(
        &self,
        animation_id: u32,
        should_loop: bool,
        should_wait_for_completion: bool,
        ignore_wait_for_transition: bool,
    ) -> Result<()>;

    fn set_as_target(&self) -> Result<()>;

    fn chr_id(&self) -> Result<i32>;
    fn handle(&self) -> Result<u64>;
    fn entity_id(&self) -> Result<u32>;
    fn block_id(&self) -> Result<u32>;
    fn npc_think_param_id(&self) -> Result<i32>;

    fn chr_ins_pointer(&self) -> Result<u64>;
    fn modules(&self) -> Result<u64>;
    fn data_pointer(&self) -> Result<u64>;
    fn super_armor_pointer(&self) -> Result<u64>;
    fn time_act_pointer(&self) -> Result<u64>;
    fn physics_pointer(&self) -> Result<u64>;
    fn behaviour_pointer(&self) -> Result<u64>;
    fn ai_think_pointer(&self) -> Result<u64>;
    fn special_effect_pointer(&self) -> Result<u64>;
    fn ctrl_flags_pointer(&self) -> Result<u64>;
    fn ride_pointer(&self) -> Result<u64>;

    fn name_from_chr_id(&self) -> &'static str;

    fn next_phase(&self) -> Result<()>;
}

pub fn chr_ins_from_entity_id(entity_id: u32) -> ChrIns {
    let location = CaveOffset::ChrInsFromEntityIdAsm.addr();
    let looked_up_entity_id = CaveOffset::LookedUpEntityId.addr();
    let world_chr_man = read::<u64>(world_chr_man::base_ptr())?;

    let fun = ASM.get_function("chr_ins_from_entity_id");
    let mut asm = fun.get_bytes();

    write_to_slice::<u64>(&mut asm, fun.reloc("world_chr_man"), world_chr_man)?;
    write_to_slice::<u32>(&mut asm, fun.reloc("entity_id"), entity_id)?;
    write_to_slice::<u64>(&mut asm, fun.reloc("fn_chr_ins"), functions::get_chr_ins_by_entity_id())?;
    write_to_slice::<u64>(&mut asm, fun.reloc("looked_up"), looked_up_entity_id)?;
    append_flag_setter(location, &mut asm)?;

    write_bytes(location, &asm)?;
    run_thread(location)?;
    read::<u64>(looked_up_entity_id)
}

pub fn chr_ins_from_handle(handle: u64) -> ChrIns {
    let pool_index = (handle >> 20) & 0xFF;
    let slot_index = handle & 0xFFFFF;
    read::<u64>(world_chr_man::base_ptr())
        .and_then(|addr| read::<u64>(addr + world_chr_man::chr_set_pool() + pool_index * 8))
        .and_then(|addr| read::<u64>(addr + world_chr_man::chr_set_offsets::CHR_SET_ENTRIES))
        .and_then(|addr| read::<u64>(addr + slot_index * 16))
}

impl ChrInsExt for ChrIns {
    fn get_current_hp(&self) -> Result<i32> {
        read::<i32>(self.data_pointer()?.saturating_add(data_offsets::HEALTH))
    }

    fn get_max_hp(&self) -> Result<i32> {
        read::<i32>(self.data_pointer()?.saturating_add(data_offsets::MAX_HEALTH))
    }

    fn set_hp(&self, val: i32) -> Result<()> {
        write::<i32>(self.data_pointer()?.saturating_add(data_offsets::HEALTH), val)
    }

    fn get_hp_pct(&self) -> Result<f32> {
        let current = self.get_current_hp()?;
        let max = self.get_max_hp()?;
        ensure!(max != 0, "Could not get hp percentage: Tried to divide by zero");
        Ok((current as f32 / max as f32) * 100.0)
    }

    fn set_hp_pct(&self, pct: i32) -> Result<()> {
        let max = self.get_max_hp()?;
        ensure!(max != 0, "Could not set hp percentage: Tried to divide by zero");
        let val = (pct * max) / 100;
        write::<i32>(self.data_pointer()?.saturating_add(data_offsets::HEALTH), val)
    }

    fn set_no_death(&self, state: bool) -> Result<()> {
        set_bit(self.data_pointer()?.saturating_add(data_flags()), bit_flags::NO_DEATH, state)
    }

    fn is_no_death(&self) -> Result<bool> {
        is_bit_set(self.data_pointer()?.saturating_add(data_flags()), bit_flags::NO_DEATH)
    }

    fn set_no_damage(&self, state: bool) -> Result<()> {
        set_bit(self.data_pointer()?.saturating_add(data_flags()), bit_flags::NO_DAMAGE, state)
    }

    fn is_no_damage(&self) -> Result<bool> {
        is_bit_set(self.data_pointer()?.saturating_add(data_flags()), bit_flags::NO_DAMAGE)
    }

    fn get_max_poise(&self) -> Result<f32> {
        read::<f32>(self.super_armor_pointer()?.saturating_add(super_armor_offsets::MAX_POISE))
    }

    fn get_current_poise(&self) -> Result<f32> {
        read::<f32>(self.super_armor_pointer()?.saturating_add(super_armor_offsets::CURRENT_POISE))
    }

    fn get_poise_timer(&self) -> Result<f32> {
        read::<f32>(self.super_armor_pointer()?.saturating_add(super_armor_offsets::POISE_TIMER))
    }

    fn get_current_animation(&self) -> Result<i32> {
        read::<i32>(self.time_act_pointer()?.saturating_add(time_act_offsets::ANIMATION_ID))
    }

    fn get_last_act(&self) -> Result<u8> {
        read::<u8>(self.ai_think_pointer()?.saturating_add(ai_think_offsets::last_act()))
    }

    fn set_repeat_act(&self, state: bool) -> Result<()> {
        write::<u8>(self.ai_think_pointer()?.saturating_add(ai_think_offsets::force_act()), state as u8)
    }

    fn is_repeat_act(&self) -> Result<bool> {
        read::<u8>(self.ai_think_pointer()?.saturating_add(ai_think_offsets::force_act()))
            .map(|val| val != 0x0)
    }

    fn force_act(&self, act: i32) -> Result<()> {
        write::<i32>(self.ai_think_pointer()?.saturating_add(ai_think_offsets::force_act()), act)
    }

    fn set_disable_ai(&self, state: bool) -> Result<()> {
        set_bit(self.ctrl_flags_pointer()?, bit_flags::DISABLE_AI, state)
    }

    fn is_disable_ai(&self) -> Result<bool> {
        is_bit_set(self.ctrl_flags_pointer()?, bit_flags::DISABLE_AI)
    }

    fn get_animation_speed(&self) -> Result<f32> {
        read::<f32>(self.behaviour_pointer()?.saturating_add(behavior_offsets::ANIMATION_SPEED))
    }

    fn set_animation_speed(&self, val: f32) -> Result<()> {
        write::<f32>(self.behaviour_pointer()?.saturating_add(behavior_offsets::ANIMATION_SPEED), val)
    }

    fn local_coords(&self) -> Result<[f32; 3]> {
        read::<[f32; 3]>(self.physics_pointer()?.saturating_add(chr_ins::physics_offsets::COORDS))
    }

    fn hurtbox_radius(&self) -> Result<f32> {
        read::<f32>(self.physics_pointer()?.saturating_add(chr_ins::physics_offsets::HURT_CAPSULE_RADIUS))
    }

    fn get_distance(&self, other: &ChrIns) -> Result<f32> {
        let self_pos = self.local_coords()?;
        let other_pos = other.local_coords()?;
        let distance = (
            (other_pos[0] - self_pos[0]).powi(2) +
            (other_pos[1] - self_pos[1]).powi(2) +
            (other_pos[2] - self_pos[2]).powi(2))
            .sqrt();
        Ok(distance - self.hurtbox_radius()? - other.hurtbox_radius()?)
    }

    fn block_id(&self) -> Result<u32> {
        read::<u32>(self.chr_ins_pointer()?.saturating_add(chr_ins::BLOCK_ID))
    }

    fn map_coords(&self) -> Result<[f32; 3]> {
        let block_pos = target::world_block_info_from_block_id(self.block_id()?)
            .and_then(|addr| read::<[f32; 3]>(addr.saturating_add(0x70)))?;
        let local_coords = self.local_coords()?;
        Ok([
            local_coords[0] - block_pos[0],
            local_coords[1] - block_pos[2],
            local_coords[1] - block_pos[2],
        ])
    }

    fn set_speffect(&self, speffect_id: u32) -> Result<()> {
        let location = CaveOffset::SetSpeffectAsm.addr();

        let mut asm = ASM.get_function("set_speffect").get_bytes();
        write_to_slice::<u64>(&mut asm, 2, self.chr_ins_pointer()?)?;
        write_to_slice::<i64>(&mut asm, 12, speffect_id)?;
        write_to_slice::<u64>(&mut asm, 22, functions::set_speffect())?;
        append_flag_setter(location, &mut asm)?;

        write_bytes(location, &asm)?;
        run_thread(location)
    }

    fn remove_speffect(&self, speffect_id: u32) -> Result<()> {
        let location = CaveOffset::RemoveSpeffectAsm.addr();

        let mut asm = ASM.get_function("remove_speffect").get_bytes();
        write_to_slice::<u64>(&mut asm, 2, self.special_effect_pointer()?)?;
        write_to_slice::<i64>(&mut asm, 12, speffect_id)?;
        write_to_slice::<u64>(&mut asm, 22, functions::remove_speffect())?;
        append_flag_setter(location, &mut asm)?;

        write_bytes(location, &asm)?;
        run_thread(location)
    }

    fn has_speffect(&self, speffect_id: u32) -> Result<bool> {
        let mut current = read::<u64>(
            self.special_effect_pointer()?.saturating_add(speffect_offsets::HEAD),
        )?;
        while current != 0x0 {
            if read::<u32>(current.saturating_add(speffect_entry::ID))? == speffect_id {
                return Ok(true);
            }
            current = read::<u64>(current.saturating_add(speffect_entry::NEXT))?;
        }
        Ok(false)
    }

    fn reset_position(&self) -> Result<()> {
        emevd::reset_character_position(self.entity_id()?)
    }

    fn force_animation_playback(
        &self,
        animation_id: u32,
        should_loop: bool,
        should_wait_for_completion: bool,
        ignore_wait_for_transition: bool,
    ) -> Result<()> {
        emevd::force_animation_playback(
            self.entity_id()?,
            animation_id,
            should_loop,
            should_wait_for_completion,
            ignore_wait_for_transition,
        )
    }

    fn get_lua_timers(&self) -> Result<[f32; 16]> {
        read::<[f32; 16]>(self.ai_think_pointer()?.saturating_add(ai_think_offsets::LUA_TIMERS_ARRAY))
    }

    fn set_as_target(&self) -> Result<()> {
        write::<u64>(CaveOffset::SavedTargetPointer.addr(), self.chr_ins_pointer()?)
    }

    fn chr_id(&self) -> Result<i32> {
        read::<i32>(self.chr_ins_pointer()?.saturating_add(chr_ins::CHR_ID))
    }

    fn handle(&self) -> Result<u64> {
        read::<u64>(self.chr_ins_pointer()?.saturating_add(chr_ins::HANDLE))
    }

    fn entity_id(&self) -> Result<u32> {
        read::<u32>(self.chr_ins_pointer()?.saturating_add(chr_ins::entity_id()))
    }

    fn npc_think_param_id(&self) -> Result<i32> {
        read::<i32>(self.ai_think_pointer()?.saturating_add(ai_think_offsets::NPC_THINK_PARAM_ID))
    }

    fn chr_ins_pointer(&self) -> Result<u64> {
        Ok(*self.as_ref().map_err(|e| anyhow!("{e}"))?)
    }

    fn modules(&self) -> Result<u64> {
        read::<u64>(self.chr_ins_pointer()?.saturating_add(chr_ins::MODULES))
    }

    fn data_pointer(&self) -> Result<u64> {
        read::<u64>(self.modules()?.saturating_add(chr_ins::CHR_DATA_MODULE))
    }

    fn super_armor_pointer(&self) -> Result<u64> {
        read::<u64>(self.modules()?.saturating_add(chr_ins::CHR_SUPER_ARMOR_MODULE))
    }

    fn time_act_pointer(&self) -> Result<u64> {
        read::<u64>(self.modules()?.saturating_add(chr_ins::CHR_TIME_ACT_MODULE))
    }

    fn behaviour_pointer(&self) -> Result<u64> {
        read::<u64>(self.modules()?.saturating_add(chr_ins::CHR_BEHAVIOR_MODULE))
    }

    fn physics_pointer(&self) -> Result<u64> {
        read::<u64>(self.modules()?.saturating_add(chr_ins::CHR_PHYSICS_MODULE))
    }

    fn ai_think_pointer(&self) -> Result<u64> {
        read::<u64>(self.chr_ins_pointer()?.saturating_add(chr_ins::manipulator()))
            .and_then(|addr| read::<u64>(addr.saturating_add(0xC0)))
    }

    fn special_effect_pointer(&self) -> Result<u64> {
        read::<u64>(self.chr_ins_pointer()?.saturating_add(chr_ins::SPECIAL_EFFECT))
    }

    fn ctrl_flags_pointer(&self) -> Result<u64> {
        read::<u64>(self.chr_ins_pointer()?.saturating_add(chr_ins::CHR_CTRL))
            .and_then(|addr| read::<u64>(addr.saturating_add(0xC8)))
            .map(|addr| addr + 0x24)
    }

    fn ride_pointer(&self) -> Result<u64> {
        read::<u64>(self.modules()?.saturating_add(chr_ins::CHR_RIDE_MODULE))
    }

    fn name_from_chr_id(&self) -> &'static str {
        CHR_NAMES
            .get(&self.chr_id().unwrap_or_default())
            .map_or("", |v| *v)
    }

    fn next_phase(&self) -> Result<()> {
        match self.entity_id()? {
            entity_ids::MARGIT_BOSS => {
                if !self.has_speffect(16200)? {
                    self.set_hp_pct(65)?
                }
            }
            entity_ids::MORGOTT => {
                if !self.has_speffect(16200)? {
                    self.set_hp_pct(60)?;
                    self.force_animation_playback(3024, false, false, false)?
                }
            }
            // entity_ids::STARSCOURGE_RADAHN => {
                // if true {
                    // self.set_hp_pct(50)?;
                    // self.force_animation_playback(3035, false, false, false)?
                // }
            // }
            entity_ids::DTS_BOSS => {
                if !self.has_speffect(13708)? {
                    self.set_hp_pct(60)?;
                    self.force_animation_playback(3027, false, false, false)?
                }
            }
            entity_ids::CLERGYMAN => {
                self.set_hp_pct(55)?;
                let maliketh_ins = chr_ins_from_entity_id(entity_ids::MALIKETH);
                maliketh_ins.set_hp_pct(55)?;
                wait_for_cutscence_completion()?;
                wait_for_event(13002802, true, 5)?;
                maliketh_ins.set_as_target()?
            }
            entity_ids::MOHG_LOB => {
                if !self.has_speffect(10643)? {
                    self.set_hp_pct(50)?;
                    self.set_speffect(10641)?;
                    self.set_speffect(10642)?;
                    self.set_speffect(10643)?;
                    self.force_animation_playback(3004, false, false, false)?
                }
            }
            entity_ids::FORTISSAX => {
                if self.get_hp_pct()? > 60.0 {
                    self.set_hp_pct(60)?;
                }
            }
            entity_ids::NOBLE_MANOR | entity_ids::NOBLE_DUO => {
                if self.has_speffect(15500)? {
                    self.set_hp_pct(60)?;
                    self.force_animation_playback(3029, false, false, false)?
                }
            }
            // entity_ids::FIRE_GIANT_P1 => {
                // let p2_ins = chr_ins_from_entity_id(entity_ids::FIRE_GIANT_P2);
                // p2_ins.set_hp_pct(53)?;
                // self.set_hp(0)?;
                // wait_for_cutscence_completion()?;
                // p2_ins.set_as_target()?
            // }
            _ => bail!("Not implemented for current target")
        }
        Ok(())
    }
}