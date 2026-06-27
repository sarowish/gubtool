use std::ptr;

pub use crate::offsets::{chr_dbg_flags::ChrDbgOffset, game_data_man::PlayerGameDataOffset};
use crate::{
    chr_ins::{self, ChrIns, ChrInsExt},
    mem::*,
    offsets::{
        ChainReadExt,
        code_cave::CaveOffset,
        game_data_man,
        module_offsets::{BasePointer, Data, Function, Hook, Patch},
        world_chr_man,
    },
    resources::ASM,
    utils::player_loaded_check,
};
use gubtool_core::{
    address::Address,
    attached::version,
    game_version::EldenRingVersion::*,
    slice_ops::*,
    sys::error::{PointerType, ProcResult, ProcessError},
};

pub fn player_ins() -> ChrIns {
    match read::<u64>(BasePointer::WorldChrMan).read_offset(world_chr_man::player_ins()) {
        Ok(ptr) if ptr != 0x0 => Ok(ptr),
        Ok(_) | Err(_) => Err(ProcessError::InvalidPointer {
            pointer_type: PointerType::PlayerIns,
        }),
    }
}

pub fn torrent_ins() -> ChrIns {
    let handle = player_game_data()
        .read_offset(game_data_man::torrent_handle())?;
    chr_ins::chr_ins_from_handle(handle)
}

pub fn set_chr_dbg_flag(offset: ChrDbgOffset, state: bool) -> ProcResult {
    write::<u8>(Data::ChrDbgFlags.add_offset(offset as u64), state as u8)
}

pub fn is_chr_dbg_flag(offset: ChrDbgOffset) -> ProcResult<bool> {
    read::<u8>(Data::ChrDbgFlags.add_offset(offset as u64)).map(|val| val == 1)
}

pub fn set_rune_arc(state: bool) -> ProcResult {
        player_game_data()
        .add_offset(PlayerGameDataOffset::RuneArc as u64)
        .write::<u8>(state as u8)
}

pub fn set_rfbs() -> ProcResult {
    let player_ins = player_ins();
    let max_hp = player_ins.get_max_hp()?;
    player_ins.set_hp((max_hp * 20) / 100 - 1)
}

pub fn set_runes(amount: u32) -> ProcResult {
    let current_amount = PlayerGameData::read().rune_count;
    let to_give = amount as i32 - current_amount as i32;
    give_runes(to_give as i64)
}

pub fn give_runes(amount: i64) -> ProcResult {
    player_loaded_check()?;

    let mut fun = ASM.get_function("give_runes");
    let mut asm = fun.take_bytes();

    write_addr_to_slice(&mut asm, fun.reloc("player_game_data"), player_game_data()?)?;
    write_to_slice::<i64>(&mut asm, fun.reloc("amount"), amount)?;
    write_addr_to_slice(&mut asm, fun.reloc("fn_give_runes"), Function::GiveRunes)?;

    spawn_thread_join(CaveOffset::GiveRunesAsm, asm)
}

pub fn map_coords() -> ProcResult<[f32; 3]> {
    read::<[f32; 3]>(
        player_ins()? + world_chr_man::player_ins_offsets::current_map_coords(),
    )
}

pub fn map_angle() -> ProcResult<f32> {
    read::<f32>(
        player_ins()? + world_chr_man::player_ins_offsets::current_map_angle(),
    )
}

fn install_grab_hook() -> ProcResult {
    let mut fun = ASM.get_function("grab_hook");
    let mut asm = fun.take_bytes();

    let location = CaveOffset::NoGrabHook;
    let skip_grab_jmp_location = Hook::PlayerNoGrab.add_offset(0x95);

    write_addr_to_slice(&mut asm, fun.reloc("world_chr_man"), BasePointer::WorldChrMan)?;
    write_to_slice::<i32>(&mut asm, fun.reloc("player_ins_off"), world_chr_man::player_ins())?;
    write_rel_i32(&mut asm, location, fun.reloc("skip_grab_jmp_location"), skip_grab_jmp_location, 4)?;
    write_rel_i32(&mut asm, location, fun.reloc("hook_loc"), Hook::PlayerNoGrab.add_offset(9), 4)?;

    install_hook(&asm, location, Hook::PlayerNoGrab, 9)
}

const GRAB_HOOK_BYTES_ORIGINAL: [u8; 9] = [0x41, 0x8B, 0x56, 0x44, 0x48, 0x8D, 0x4C, 0x24, 0x40];
fn uninstall_grab_hook() -> ProcResult {
    write_bytes(Hook::PlayerNoGrab, &GRAB_HOOK_BYTES_ORIGINAL)
}

fn install_infinite_poise_hook() -> ProcResult {
    let mut fun = ASM.get_function("infinite_poise_hook");
    let mut asm = fun.take_bytes();

    write_addr_to_slice(&mut asm, fun.reloc("world_chr_man"), BasePointer::WorldChrMan)?;
    write_to_slice::<i32>(&mut asm, fun.reloc("player_ins_off"), world_chr_man::player_ins())?;
    write_addr_to_slice(&mut asm, fun.reloc("fn_get_chr_ins"), Function::GetChrInsByEntityId)?;
    write_rel_i32(
        &mut asm,
        CaveOffset::InfinitePoiseHook,
        fun.reloc("hook_loc"),
        Hook::PlayerInfinitePoise.add_offset(7),
        4,
    )?;
    install_hook(&asm, CaveOffset::InfinitePoiseHook, Hook::PlayerInfinitePoise, 7)
}

fn infinite_poise_bytes_original() -> [u8; 7] {
    match version() {
        Some(Version1_2_0) | Some(Version1_2_1) | Some(Version1_2_2) | Some(Version1_2_3) => {
            [0x4C, 0x8B, 0xC7, 0x41, 0x0F, 0xB6, 0xD6]
        }
        _ => [0x4C, 0x8B, 0xC7, 0x40, 0x0F, 0xB6, 0xD5],
    }
}
fn uninstall_infinite_poise_hook() -> ProcResult {
    write_bytes(Hook::PlayerInfinitePoise, &infinite_poise_bytes_original())
}

pub fn is_infinite_poise() -> ProcResult<bool> {
    read::<[u8; 7]>(Hook::PlayerInfinitePoise)
        .map(|val| val != infinite_poise_bytes_original())
}

pub fn set_infinite_poise(val: bool) -> ProcResult {
    match val {
        true => {
            install_infinite_poise_hook()?;
            install_grab_hook()
        }
        false => {
            uninstall_infinite_poise_hook()?;
            uninstall_grab_hook()
        }
    }
}

pub fn set_torrent_anywhere(state: bool) -> ProcResult {
    match state {
        true => {
            write_bytes(Patch::TorrentDisabledUnderworld, &[0x30, 0xC0, 0x90])?;
            write_bytes(Patch::WhistleDisabled, &[0x30, 0xC0, 0x90])
        }
        false => {
            write_bytes(Patch::TorrentDisabledUnderworld, &[0x0F, 0x95, 0xC0])?;
            write_bytes(Patch::WhistleDisabled, &[0x0F, 0x95, 0xC0])
        }
    }
}

pub fn is_torrent_anywhere() -> ProcResult<bool> {
    read::<[u8; 3]>(Patch::WhistleDisabled)
        .map(|val| val != [0x0F, 0x95, 0xC0])
}

fn player_game_data() -> ProcResult<u64> {
    read::<u64>(BasePointer::GameDataMan)
        .read_offset(game_data_man::PLAYER_GAME_DATA)
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct PlayerGameData {
    vftable: usize,
    pub character_event_id: u32,
    pub player_id: u32,
    pub current_hp: u32,
    pub current_max_hp: u32,
    pub base_max_hp: u32,
    pub current_fp: u32,
    pub current_max_fp: u32,
    pub base_max_fp: u32,
    unk28: f32,
    pub current_stamina: u32,
    pub current_max_stamina: u32,
    pub base_max_stamina: u32,
    unk38: f32,
    pub vigor: u32,
    pub mind: u32,
    pub endurance: u32,
    pub strength: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub faith: u32,
    pub arcane: u32,
    pub base_hero_point: f32,
    pub base_hero_point_2: f32,
    pub base_durability: f32,
    pub level: u32,
    pub rune_count: u32,
    pub rune_memory: u32,
    unk74: u32,
    pub poison_resist: u32,
    pub rot_resist: u32,
    pub bleed_resist: u32,
    pub death_resist: u32,
    pub frost_resist: u32,
    pub sleep_resist: u32,
    pub madness_resist: u32,
    pub pending_block_clear_bonus: f32,
    pub chr_type: i32,
    character_name: [u16; 17],
    pub gender: u8,
    pub archetype: u8,
    pub vow_type: u8,
    unkc1: u8,
    pub voice_type: u8,
    pub starting_gift: u8,
    unkc4: u8,
    pub unlocked_magic_slots: u8,
    pub unlocked_talisman_slots: u8,
    pub matchmaking_spirit_ashes_level: u8,
    pub total_summon_count: u32,
    pub coop_success_count: u32,
    pub game_data_man_index: u32,
    unkd4: [u8; 0xb],
    pub furlcalling_finger_remedy_active: bool,
    unke0: u8,
    unke1: u8,
    pub matching_weapon_level: u8,
    pub white_ring_active: u8,
    pub blue_ring_active: u8,
    pub multiplay_role: u8,
    unke6: u8,
    pub is_my_world: bool,
    unke8: [u8; 0x3],
    unke9: bool,
    pub character_id: u32,
    pub invasions_success_count: u32,
    pub solo_breakin_point: u32,
    pub invaders_killed: u32,
    pub scadutree_blessing: u8,
    pub reversed_spirit_ash: u8,
    pub resist_curse_item_count: u8,
    pub rune_arc_active: bool,
    unk100: bool,
    pub max_hp_flask: u8,
    pub max_fp_flask: u8,
}

impl PlayerGameData {
    pub fn read() -> Self {
        if player_loaded_check().is_err() {
            return Self::default()
        }
        let bytes = read::<u64>(BasePointer::GameDataMan)
            .read_offset(game_data_man::PLAYER_GAME_DATA)
            .read::<[u8; std::mem::size_of::<Self>()]>()
            .unwrap_or([0x0; std::mem::size_of::<Self>()]);
        unsafe {
            ptr::read_unaligned(bytes.as_ptr() as *const Self)
        }
    }
}

pub fn set_stat(player_game_data_offset: PlayerGameDataOffset, val: i32) -> anyhow::Result<()> {
    player_loaded_check()?;

    let val = val.clamp(0, 99);

    let game_data = player_game_data()?;
    let current_val = read::<i32>(game_data + player_game_data_offset as u64)?;

    let diff = val - current_val;
    let current_level = read::<i32>(game_data + PlayerGameDataOffset::RuneLevel as u64)?;

    if val > current_val {
        let mut rune_cost = 0;
        for i in 1..=diff {
            rune_cost += level_up_cost(current_level + i);
        }
        let current_rune_mem = read::<u32>(game_data + PlayerGameDataOffset::RuneMemory as u64)?;
        let new_rune_mem = std::cmp::min(current_rune_mem as u64 + rune_cost as u64, 0xFFFFFFFF);
        write::<u32>(
            game_data + PlayerGameDataOffset::RuneMemory as u64,
            new_rune_mem as u32,
        )?;
    }
    write::<i32>(
        game_data + PlayerGameDataOffset::RuneLevel as u64,
        current_level + diff,
    )?;
    write::<i32>(game_data + player_game_data_offset as u64, val)?;
    Ok(())
}

pub fn set_dlc_stat(player_game_data_offset: PlayerGameDataOffset, val: u8) -> anyhow::Result<()> {
    player_loaded_check()?;
    write::<u8>(player_game_data()? + player_game_data_offset as u64, val.clamp(0, 20))?;
    Ok(())
}

fn level_up_cost(next_level: i32) -> i32 {
    let base_level_offset = 80_f32;
    let initial_level_up_cost = 0.1_f32;
    let initial_level_up_offset = 1_f32;
    let level_up_cost_increase = 0.02_f32;
    let level_up_increase_interval = 92_f32;

    let base_level = next_level as f32 + base_level_offset;
    let adjusted_level = 0.0_f32.max(base_level - level_up_increase_interval);
    let cost =
        base_level * base_level * (level_up_cost_increase * adjusted_level + initial_level_up_cost)
            + initial_level_up_offset;
    cost as i32
}
