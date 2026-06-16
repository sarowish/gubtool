use std::time::Duration;

use crate::{
    emevd,
    mem::*,
    offsets::{
        self, ChainReadExt, chr_ins::*, code_cave::CaveOffset, functions, world_chr_man
    },
    phase_transition,
    resources::{ASM, chr_names::CHR_NAMES},
    target,
};
use anyhow::ensure;
use gubtool_core::sys::error::ProcResult;
use utils::slice_ops::*;

pub type ChrIns = ProcResult<u64>;

pub trait ChrInsExt {
    fn get_current_hp(&self) -> ProcResult<i32>;
    fn get_max_hp(&self) -> ProcResult<i32>;
    fn set_hp(&self, val: i32) -> ProcResult;
    fn get_hp_pct(&self) -> anyhow::Result<f32>;
    fn set_hp_pct(&self, val: i32) -> anyhow::Result<()>;
    fn set_no_death(&self, val: bool) -> ProcResult;
    fn is_no_death(&self) -> ProcResult<bool>;
    fn set_no_damage(&self, val: bool) -> ProcResult;
    fn is_no_damage(&self) -> ProcResult<bool>;
    fn get_current_poise(&self) -> ProcResult<f32>;
    fn get_max_poise(&self) -> ProcResult<f32>;
    fn get_poise_timer(&self) -> ProcResult<f32>;
    fn get_current_animation(&self) -> ProcResult<i32>;
    fn get_last_act(&self) -> ProcResult<u8>;
    fn is_repeat_act(&self) -> ProcResult<bool>;
    fn repeat_act(&self, act: u8) -> ProcResult;
    fn force_act(&self, act: u8) -> ProcResult;
    fn set_repeat_last_act(&self, val: bool) -> ProcResult;
    fn set_disable_ai(&self, state: bool) -> ProcResult;
    fn is_disable_ai(&self) -> ProcResult<bool>;
    fn get_animation_speed(&self) -> ProcResult<f32>;
    fn set_animation_speed(&self, val: f32) -> ProcResult;
    fn local_coords(&self) -> ProcResult<[f32; 3]>;
    fn map_coords(&self) -> anyhow::Result<[f32; 3]>;
    fn hurtbox_radius(&self) -> ProcResult<f32>;
    fn get_distance(&self, other: &ChrIns) -> ProcResult<f32>;
    fn set_speffect(self, speffect_id: u32) -> ProcResult;
    fn remove_speffect(&self, speffect_id: u32) -> ProcResult;
    fn has_speffect(&self, speffect_id: u32) -> ProcResult<bool>;
    fn reset_position(&self) -> ProcResult;
    fn get_lua_timers(&self) -> ProcResult<[f32; 16]>;
    fn force_animation_playback(
        &self,
        animation_id: u32,
        should_loop: bool,
        should_wait_for_completion: bool,
        ignore_wait_for_transition: bool,
    ) -> ProcResult;

    fn set_as_target(&self) -> ProcResult;
    fn next_phase(&self) -> anyhow::Result<()>;

    fn chr_id(&self) -> ProcResult<i32>;
    fn handle(&self) -> ProcResult<u64>;
    fn entity_id(&self) -> ProcResult<u32>;
    fn block_id(&self) -> ProcResult<u32>;
    fn npc_think_param_id(&self) -> ProcResult<i32>;

    fn modules(&self) -> ProcResult<u64>;
    fn data_pointer(&self) -> ProcResult<u64>;
    fn super_armor_pointer(&self) -> ProcResult<u64>;
    fn time_act_pointer(&self) -> ProcResult<u64>;
    fn physics_pointer(&self) -> ProcResult<u64>;
    fn behaviour_pointer(&self) -> ProcResult<u64>;
    fn ai_think_pointer(&self) -> ProcResult<u64>;
    fn special_effect_pointer(&self) -> ProcResult<u64>;
    fn ctrl_flags_pointer(&self) -> ProcResult<u64>;
    fn ride_pointer(&self) -> ProcResult<u64>;

    fn name_from_chr_id(&self) -> &'static str;
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

    spawn_thread_join(location, asm)?;
    read::<u64>(looked_up_entity_id)
}

pub fn chr_ins_from_handle(handle: u64) -> ChrIns {
    let pool_index = (handle >> 20) & 0xFF;
    let slot_index = handle & 0xFFFFF;
    read::<u64>(world_chr_man::base_ptr())
        .read_offset(world_chr_man::chr_set_pool() + pool_index * 8)
        .read_offset(world_chr_man::chr_set_offsets::CHR_SET_ENTRIES)
        .read_offset(slot_index * 16)
}

impl ChrInsExt for ChrIns {
    fn get_current_hp(&self) -> ProcResult<i32> {
        self.data_pointer()
            .add_offset(data_offsets::HEALTH)
            .read::<i32>()
    }

    fn get_max_hp(&self) -> ProcResult<i32> {
        self.data_pointer()
            .add_offset(data_offsets::MAX_HEALTH)
            .read::<i32>()
    }

    fn set_hp(&self, val: i32) -> ProcResult {
        self.data_pointer()
            .add_offset(data_offsets::HEALTH)
            .write::<i32>(val)
    }

    fn get_hp_pct(&self) -> anyhow::Result<f32> {
        let current = self.get_current_hp()?;
        let max = self.get_max_hp()?;
        ensure!(max != 0, "Could not get hp percentage: Tried to divide by zero");
        Ok((current as f32 / max as f32) * 100.0)
    }

    fn set_hp_pct(&self, pct: i32) -> anyhow::Result<()> {
        let max = self.get_max_hp()?;
        ensure!(max != 0, "Could not set hp percentage: Tried to divide by zero");
        let val = (pct * max) / 100;
        Ok(write::<i32>(self.data_pointer()?.saturating_add(data_offsets::HEALTH), val)?)
    }

    fn set_no_death(&self, state: bool) -> ProcResult {
        self.data_pointer()
            .add_offset(offsets::chr_ins::data_flags())
            .set_bit(bit_flags::NO_DEATH, state)
    }

    fn is_no_death(&self) -> ProcResult<bool> {
        self.data_pointer()
            .add_offset(offsets::chr_ins::data_flags())
            .is_bit_set(bit_flags::NO_DEATH)
    }

    fn set_no_damage(&self, state: bool) -> ProcResult {
        self.data_pointer()
            .add_offset(offsets::chr_ins::data_flags())
            .set_bit(bit_flags::NO_DAMAGE, state)
    }

    fn is_no_damage(&self) -> ProcResult<bool> {
        self.data_pointer()
            .add_offset(offsets::chr_ins::data_flags())
            .is_bit_set(bit_flags::NO_DAMAGE)
    }

    fn get_max_poise(&self) -> ProcResult<f32> {
        self.super_armor_pointer()
            .add_offset(super_armor_offsets::MAX_POISE)
            .read::<f32>()
    }

    fn get_current_poise(&self) -> ProcResult<f32> {
        self.super_armor_pointer()
            .add_offset(super_armor_offsets::CURRENT_POISE)
            .read::<f32>()
    }

    fn get_poise_timer(&self) -> ProcResult<f32> {
        self.super_armor_pointer()
            .add_offset(super_armor_offsets::POISE_TIMER)
            .read::<f32>()
    }

    fn get_current_animation(&self) -> ProcResult<i32> {
        self.time_act_pointer()
            .add_offset(time_act_offsets::ANIMATION_ID)
            .read::<i32>()
    }

    fn get_last_act(&self) -> ProcResult<u8> {
        self.ai_think_pointer()
            .add_offset(ai_think_offsets::last_act())
            .read::<u8>()
    }

    fn set_repeat_last_act(&self, state: bool) -> ProcResult {
        let val = if state { self.get_last_act()? } else { 0x0 };
        self.ai_think_pointer()
            .add_offset(ai_think_offsets::force_act())
            .write::<u8>(val)
    }

    fn is_repeat_act(&self) -> ProcResult<bool> {
        self.ai_think_pointer()
            .add_offset(ai_think_offsets::force_act())
            .read::<u8>()
            .map(|val| val != 0x0)
    }

    fn repeat_act(&self, act: u8) -> ProcResult {
        self.ai_think_pointer()
            .add_offset(ai_think_offsets::force_act())
            .write::<u8>(act)
    }

    fn force_act(&self, act: u8) -> ProcResult {
        self.repeat_act(act)?;
        while self.get_last_act()? != act {
            std::thread::sleep(Duration::from_millis(50));
        }
        self.set_repeat_last_act(false)
    }

    fn set_disable_ai(&self, state: bool) -> ProcResult {
        self.ctrl_flags_pointer()
            .set_bit(bit_flags::DISABLE_AI, state)
    }

    fn is_disable_ai(&self) -> ProcResult<bool> {
        self.ctrl_flags_pointer()
            .is_bit_set(bit_flags::DISABLE_AI)
    }

    fn get_animation_speed(&self) -> ProcResult<f32> {
        self.behaviour_pointer()
            .add_offset(behavior_offsets::ANIMATION_SPEED)
            .read::<f32>()
    }

    fn set_animation_speed(&self, val: f32) -> ProcResult {
        self.behaviour_pointer()
            .add_offset(behavior_offsets::ANIMATION_SPEED)
            .write::<f32>(val)
    }

    fn local_coords(&self) -> ProcResult<[f32; 3]> {
        self.physics_pointer()
            .add_offset(offsets::chr_ins::physics_offsets::COORDS)
            .read::<[f32; 3]>()
    }

    fn hurtbox_radius(&self) -> ProcResult<f32> {
        self.physics_pointer()
            .add_offset(offsets::chr_ins::physics_offsets::HURT_CAPSULE_RADIUS)
            .read::<f32>()
    }

    fn get_distance(&self, other: &ChrIns) -> ProcResult<f32> {
        let self_pos = self.local_coords()?;
        let other_pos = other.local_coords()?;
        let distance = (
            (other_pos[0] - self_pos[0]).powi(2) +
            (other_pos[1] - self_pos[1]).powi(2) +
            (other_pos[2] - self_pos[2]).powi(2))
            .sqrt();
        Ok(distance - self.hurtbox_radius()? - other.hurtbox_radius()?)
    }

    fn block_id(&self) -> ProcResult<u32> {
        self.add_offset(offsets::chr_ins::BLOCK_ID).read::<u32>()
    }

    fn map_coords(&self) -> anyhow::Result<[f32; 3]> {
        let block_pos = target::world_block_info_from_block_id(self.block_id()?)
            .and_then(|addr| Ok(read::<[f32; 3]>(addr.saturating_add(0x70))?))?;
        let local_coords = self.local_coords()?;
        Ok([
            local_coords[0] - block_pos[0],
            local_coords[1] - block_pos[2],
            local_coords[1] - block_pos[2],
        ])
    }

    fn set_speffect(self, speffect_id: u32) -> ProcResult {
        let location = CaveOffset::SetSpeffectAsm.addr();

        let mut asm = ASM.get_function("set_speffect").get_bytes();
        write_to_slice::<u64>(&mut asm, 2, self?)?;
        write_to_slice::<i64>(&mut asm, 12, speffect_id)?;
        write_to_slice::<u64>(&mut asm, 22, functions::set_speffect())?;

        spawn_thread_join(location, asm)
    }

    fn remove_speffect(&self, speffect_id: u32) -> ProcResult {
        let location = CaveOffset::RemoveSpeffectAsm.addr();

        let mut asm = ASM.get_function("remove_speffect").get_bytes();
        write_to_slice::<u64>(&mut asm, 2, self.special_effect_pointer()?)?;
        write_to_slice::<i64>(&mut asm, 12, speffect_id)?;
        write_to_slice::<u64>(&mut asm, 22, functions::remove_speffect())?;

        spawn_thread_join(location, asm)
    }

    fn has_speffect(&self, speffect_id: u32) -> ProcResult<bool> {
        let mut current = self.special_effect_pointer()
            .read_offset(speffect_offsets::HEAD)?;
        while current != 0x0 {
            if read::<u32>(current.saturating_add(speffect_entry::ID))? == speffect_id {
                return Ok(true);
            }
            current = read::<u64>(current.saturating_add(speffect_entry::NEXT))?;
        }
        Ok(false)
    }

    fn reset_position(&self) -> ProcResult {
        emevd::reset_character_position(self.entity_id()?)
    }

    fn force_animation_playback(
        &self,
        animation_id: u32,
        should_loop: bool,
        should_wait_for_completion: bool,
        ignore_wait_for_transition: bool,
    ) -> ProcResult {
        emevd::force_animation_playback(
            self.entity_id()?,
            animation_id,
            should_loop,
            should_wait_for_completion,
            ignore_wait_for_transition,
        )
    }

    fn next_phase(&self) -> anyhow::Result<()> {
        phase_transition::next_phase(self)
    }

    fn get_lua_timers(&self) -> ProcResult<[f32; 16]> {
        read::<[f32; 16]>(self.ai_think_pointer()?.saturating_add(ai_think_offsets::LUA_TIMERS_ARRAY))
    }

    fn set_as_target(&self) -> ProcResult {
        write::<u64>(CaveOffset::SavedTargetPointer.addr(), self.clone()?)
    }

    fn chr_id(&self) -> ProcResult<i32> {
        self.add_offset(offsets::chr_ins::CHR_ID).read::<i32>()
    }

    fn handle(&self) -> ProcResult<u64> {
        self.read_offset(offsets::chr_ins::HANDLE)
    }

    fn entity_id(&self) -> ProcResult<u32> {
        self.add_offset(offsets::chr_ins::entity_id()).read::<u32>()
    }

    fn npc_think_param_id(&self) -> ProcResult<i32> {
        self.ai_think_pointer()
            .add_offset(ai_think_offsets::NPC_THINK_PARAM_ID)
            .read::<i32>()
    }

    fn modules(&self) -> ProcResult<u64> {
        self.read_offset(offsets::chr_ins::MODULES)
    }

    fn data_pointer(&self) -> ProcResult<u64> {
        self.modules().read_offset(offsets::chr_ins::CHR_DATA_MODULE)
    }

    fn super_armor_pointer(&self) -> ProcResult<u64> {
        self.modules().read_offset(offsets::chr_ins::CHR_SUPER_ARMOR_MODULE)
    }

    fn time_act_pointer(&self) -> ProcResult<u64> {
        self.modules().read_offset(offsets::chr_ins::CHR_TIME_ACT_MODULE)
    }

    fn behaviour_pointer(&self) -> ProcResult<u64> {
        self.modules().read_offset(offsets::chr_ins::CHR_BEHAVIOR_MODULE)
    }

    fn physics_pointer(&self) -> ProcResult<u64> {
        self.modules().read_offset(offsets::chr_ins::CHR_PHYSICS_MODULE)
    }

    fn ai_think_pointer(&self) -> ProcResult<u64> {
        self.read_offset(offsets::chr_ins::manipulator()).read_offset(0xC0)
    }

    fn special_effect_pointer(&self) -> ProcResult<u64> {
        self.read_offset(offsets::chr_ins::SPECIAL_EFFECT)
    }

    fn ctrl_flags_pointer(&self) -> ProcResult<u64> {
        self.read_offset(offsets::chr_ins::CHR_CTRL)
            .read_offset(0xC8)
            .add_offset(0x24)
    }

    fn ride_pointer(&self) -> ProcResult<u64> {
        self.modules().read_offset(offsets::chr_ins::CHR_RIDE_MODULE)
    }

    fn name_from_chr_id(&self) -> &'static str {
        CHR_NAMES
            .get(&self.chr_id().unwrap_or_default())
            .map_or("", |v| *v)
    }
}