use crate::{
    chr_ctrl::{ChrCtrl, ChrCtrlExt},
    mem::*,
    offsets::{
        self, ChainReadExt, Offset,
        chr_ctrl::stats_offsets::{self},
        code_cave::CaveOffset,
        game_manager_imp,
        module_offsets::{BasePointer, Function, Hook, Patch},
    },
    resources::asm_function,
};
use gubtool_core::{
    address::Address,
    attached::{is_32, version},
    game_version::DarkSouls2Version,
    slice_ops::*,
    sys::error::ProcResult,
};

pub fn player_ctrl() -> ChrCtrl {
    read_address(BasePointer::GameManagerImp)
        .read_offset(game_manager_imp::PLAYER_CTRL)
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Stats {
    pub vigor: u16,
    pub endurance: u16,
    pub vitality: u16,
    pub attunement: u16,
    pub strength: u16,
    pub dexterity: u16,
    pub intelligence: u16,
    pub faith: u16,
    pub adaptability: u16,
    // unk1: u16,
    // unk2: u16,
    // effective_vigor: u16,
    // effective_endurance: u16,
    // effective_vitality: u16,
    // effective_attunement: u16,
    // effective_strength: u16,
    // effective_dexterity: u16,
    // effective_intelligence: u16,
    // effective_faith: u16,
    // effective_adaptability: u16,
    // unk3: u16,
    // unk4: u16,
}

impl Stats {
    pub fn read() -> Self {
        let bytes = read_address(BasePointer::GameManagerImp)
            .read_offset(game_manager_imp::PLAYER_CTRL)
            .read_offset(offsets::chr_ctrl::STATS_PTR)
            .add_offset(stats_offsets::STATS)
            .read::<[u8; std::mem::size_of::<Self>()]>()
            .unwrap_or([0x0; std::mem::size_of::<Self>()]);

        unsafe { *(bytes.as_ptr() as *const Self) }
    }
}

#[repr(u64)]
#[derive(Debug, Clone, Copy)]
pub enum StatOffset {
    Vigor = 0x0,
    Endurance = 0x2,
    Vitality = 0x4,
    Attunement = 0x6,
    Strength = 0x8,
    Dexterity = 0xA,
    Intelligence = 0xC,
    Faith = 0xE,
    Adaptability = 0x10,
}

const NEGATIVE_LEVEL_PATCH: Offset = Offset {
    vanilla: 0x32,
    scholar: 0x39,
};
pub fn set_stat(offset: StatOffset, val: u16) -> ProcResult {
    let player_stats_entity = read_address(BasePointer::GameManagerImp)
        .read_offset(game_manager_imp::PLAYER_CTRL)
        .read_offset(offsets::chr_ctrl::STATS_PTR);

    let stats_base = player_stats_entity
        .add_offset(stats_offsets::STATS);

    let stat_loc = stats_base
        .map(|addr| addr + offset as u64);

    let new_stat = val.clamp(0, 99);
    let current_stat = stat_loc.read::<u16>()?;
    let num_levels = new_stat as i32 - current_stat as i32;

    stat_loc.write::<u16>(new_stat)?;

    let is_negative = num_levels <= 0;

    const NUM_LEVELS_SHORT: u64 = 0xE2;
    const NUM_LEVELS_INT: u64 = 0xE8;
    const CURRENT_LEVEL: u64 = 0xEC;
    const NEW_LEVEL: u64 = 0xF0;
    const CURRENT_SOULS: u64 = 0xF4;
    const REQUIRED_SOULS: u64 = 0xFC;
    const SOULS_AFTER: u64 = 0xF8;

    let current_level = player_stats_entity
        .add_offset(stats_offsets::SOUL_LEVEL)
        .read::<i32>()?;

    let current_souls = player_stats_entity
        .add_offset(stats_offsets::SOULS)
        .read::<i32>()?;

    let stat_bytes = stats_base.read::<[u8; 22]>()?;

    let location = CaveOffset::LevelUpAsm.addr();
    let buffer_loc = CaveOffset::LevelUpBuffer.addr();
    let negative_flag_loc = CaveOffset::NegativeFlag.addr();

    let mut buffer = [0x0; 0x100];

    write_to_slice::<[u8; 22]>(&mut buffer, 0, stat_bytes)?;
    write_to_slice::<i32>(&mut buffer, CURRENT_LEVEL, current_level)?;
    write_to_slice::<u16>(&mut buffer, NUM_LEVELS_SHORT, num_levels as u16)?;
    write_to_slice::<i32>(&mut buffer, NUM_LEVELS_INT, num_levels)?;
    write_to_slice::<i32>(&mut buffer, NEW_LEVEL, current_level + num_levels)?;
    write_to_slice::<i32>(&mut buffer, CURRENT_SOULS, current_souls)?;

    write_bytes(buffer_loc, &buffer)?;
    write::<u8>(negative_flag_loc, is_negative as u8)?;

    let negative_patch_loc = Function::LevelUp.add_offset(NEGATIVE_LEVEL_PATCH.resolve());
    if is_negative {
        write::<u8>(negative_patch_loc, 0x85)?;
    }

    let mut fun = asm_function("level_up");
    let mut asm = fun.take_bytes();

    write_addr_to_slice(&mut asm, fun.reloc("current_level"), buffer_loc + CURRENT_LEVEL)?;
    write_addr_to_slice(&mut asm, fun.reloc("negative_flag"), negative_flag_loc)?;
    write_addr_to_slice(&mut asm, fun.reloc("fn_level_lookup"), Function::LevelLookup)?;
    write_addr_to_slice(&mut asm, fun.reloc("new_level"), buffer_loc + NEW_LEVEL)?;
    write_addr_to_slice(&mut asm, fun.reloc("required_souls"), buffer_loc + REQUIRED_SOULS)?;
    write_addr_to_slice(&mut asm, fun.reloc("current_souls"), buffer_loc + CURRENT_SOULS)?;
    write_addr_to_slice(&mut asm, fun.reloc("stats_entity"), player_stats_entity?)?;
    write_addr_to_slice(&mut asm, fun.reloc("fn_give_souls"), Function::GiveSouls)?;
    write_addr_to_slice(&mut asm, fun.reloc("stats_entity"), player_stats_entity?)?;
    write_addr_to_slice(&mut asm, fun.reloc("current_souls"), buffer_loc + CURRENT_SOULS)?;
    write_addr_to_slice(&mut asm, fun.reloc("required_souls"), buffer_loc + REQUIRED_SOULS)?;
    write_addr_to_slice(&mut asm, fun.reloc("souls_after"), buffer_loc + SOULS_AFTER)?;
    write_addr_to_slice(&mut asm, fun.reloc("stats_entity"), player_stats_entity?)?;
    write_addr_to_slice(&mut asm, fun.reloc("buffer"), buffer_loc)?;
    write_addr_to_slice(&mut asm, fun.reloc("fn_level_up"), Function::LevelUp)?;

    spawn_thread_join(location, asm)?;

    if is_negative {
        write::<u8>(negative_patch_loc, 0x84)
    } else {
        let new_souls = player_stats_entity
            .add_offset(stats_offsets::SOULS)
            .read::<i32>()?;
        give_souls(current_souls - new_souls)
    }
}

pub fn get_souls() -> ProcResult<i32> {
    read_address(BasePointer::GameManagerImp)
        .read_offset(game_manager_imp::PLAYER_CTRL)
        .read_offset(offsets::chr_ctrl::STATS_PTR)
        .add_offset(stats_offsets::SOULS)
        .read::<i32>()
}

pub fn set_souls(amount: u32) -> ProcResult {
    let souls_loc = read_address(BasePointer::GameManagerImp)
        .read_offset(game_manager_imp::PLAYER_CTRL)
        .read_offset(offsets::chr_ctrl::STATS_PTR)
        .add_offset(stats_offsets::SOULS);
    let current = souls_loc.read::<i32>()?;
    let diff = amount.min(999999999) as i32 - current;
    if diff < 0 {
        souls_loc.write::<i32>(current + diff)
    } else {
        give_souls(diff)
    }
}

fn give_souls(amount: i32) -> ProcResult {
    let mut fun = asm_function("give_souls");
    let mut asm = fun.take_bytes();

    write_to_slice::<i32>(&mut asm, fun.reloc("amount"), amount)?;
    write_addr_to_slice(&mut asm, fun.reloc("stats_entity"), player_ctrl().stats_pointer()?)?;
    write_addr_to_slice(&mut asm, fun.reloc("fn_give_souls"), Function::GiveSouls)?;

    spawn_thread_join(CaveOffset::GiveSoulsAsm, asm)
}

pub fn player_position() -> ProcResult<[f32; 16]> {
    let pointer = follow_pointers(&game_manager_imp::player_coords_chain(), false)?;
    read::<[f32; 16]>(pointer)
}

const VANILLA_INFINITE_POISE_ORIGINAL: [u8; 7] = [0x83, 0xBB, 0xEC, 0x05, 0x00, 0x00, 0x00];
const SCHOLAR_INFINITE_POISE_ORIGINAL: [u8; 6] = [0x39, 0x9D, 0xEC, 0x05, 0x00, 0x00];
pub fn set_infinite_poise(state: bool) -> ProcResult {
    if state {
        let orig_instr_len = if is_32() { 7 } else { 6 };
        let mut fun = asm_function("infinite_poise_hook");
        let mut asm = fun.take_bytes();
        write_addr_to_slice(&mut asm, fun.reloc("game_man_imp"), BasePointer::GameManagerImp)?;
        write_rel_i32(&mut asm, CaveOffset::InfinitePoiseHook, fun.reloc("hook_loc"), Hook::InfinitePoise.add_offset(orig_instr_len), 4)?;
        install_hook(&asm, CaveOffset::InfinitePoiseHook, Hook::InfinitePoise, orig_instr_len)
    } else {
        let bytes: &[u8] = match is_32() {
            true => &VANILLA_INFINITE_POISE_ORIGINAL,
            false => &SCHOLAR_INFINITE_POISE_ORIGINAL,
        };
        write_bytes(Hook::InfinitePoise, bytes)
    }
}

pub fn is_infinite_poise() -> bool {
    if is_32() {
        read::<[u8; 7]>(Hook::InfinitePoise)
            .map(|val| val != VANILLA_INFINITE_POISE_ORIGINAL)
    } else {
        read::<[u8; 6]>(Hook::InfinitePoise)
            .map(|val| val != SCHOLAR_INFINITE_POISE_ORIGINAL)
    }
    .unwrap_or_default()
}

const VANILLA_NO_DAMAGE_ORIGINAL: [u8; 6] = [0x89, 0x8E, 0xFC, 0x00, 0x00, 0x00];
const SCHOLAR_NO_DAMAGE_ORIGINAL: [u8; 6] = [0x89, 0x83, 0x68, 0x01, 0x00, 0x00];
pub fn set_no_damage(state: bool) -> ProcResult {
    let hook_loc = Hook::PlayerNoDamage;
    let cave_loc = CaveOffset::PlayerNoDamageHook;
    if state {
        let mut fun = asm_function("player_no_damage");
        let mut asm = fun.take_bytes();
        write_addr_to_slice(&mut asm, fun.reloc("game_man_imp"), BasePointer::GameManagerImp)?;
        write_rel_i32(&mut asm, cave_loc, fun.reloc("hook_loc"), hook_loc.add_offset(6), 4)?;
        install_hook(&asm, cave_loc, hook_loc, 6)
    } else {
        let bytes: &[u8] = match is_32() {
            true => &VANILLA_NO_DAMAGE_ORIGINAL,
            false => &SCHOLAR_NO_DAMAGE_ORIGINAL,
        };
        write_bytes(hook_loc, bytes)
    }
}

pub fn is_no_damage() -> bool {
    let bytes: &[u8] = match is_32() {
        true => &VANILLA_NO_DAMAGE_ORIGINAL,
        false => &SCHOLAR_NO_DAMAGE_ORIGINAL,
    };
    read::<[u8; 6]>(Hook::PlayerNoDamage)
        .map(|val| val != bytes)
        .unwrap_or_default()
}

pub fn set_infinite_consumables(state: bool) -> ProcResult {
    let bytes: &[u8] = match (state, is_scholar()) {
        (true, true) | (true, false) => &[0x90; 4],
        (false, true) => &[0x66, 0x29, 0x73, 0x20],
        (false, false) => &[0x66, 0x29, 0x5E, 0x18],
    };
    write_bytes(Patch::InfiniteConsumables, bytes)
}

pub fn is_infinite_consumables() -> bool {
    read::<[u8; 4]> (Patch::InfiniteConsumables)
        .map(|val| val == [0x90; 4])
        .unwrap_or_default()
}

pub fn set_no_hollowing(state: bool) -> ProcResult {
    let bytes: &[u8] = match (state, is_scholar()) {
        (true, true) | (true, false) => &[0x90; 6],
        (false, true) => &[0x88, 0x81, 0xAC, 0x01, 0x00, 0x00],
        (false, false) => &[0x88, 0x91, 0xA8, 0x01, 0x00, 0x00],
    };
    write_bytes(Patch::NoHollowing, bytes)
}

pub fn is_no_hollowing() -> bool {
    read::<[u8; 6]> (Patch::NoHollowing)
        .map(|val| val == [0x90; 6])
        .unwrap_or_default()
}

pub fn set_infinite_durability(state: bool) -> ProcResult {
    let bytes: &[u8] = match (state, is_scholar()) {
        (true, true) => &[0x90; 9],
        (true, false) => &[0x90; 5],
        (false, true) => &[0xF3, 0x0F, 0x11, 0xB4, 0xC3, 0x94, 0x00, 0x00, 0x00],
        (false, false) => &[0xF3, 0x0F, 0x11, 0x47, 0x6C],
    };
    write_bytes(Patch::InfiniteDurability, bytes)
}

pub fn is_infinite_durability() -> bool {
    read::<[u8; 5]> (Patch::InfiniteDurability)
        .map(|val| val == [0x90; 5])
        .unwrap_or_default()
}

pub fn set_no_soul_gain(state: bool) -> ProcResult {
    let bytes: &[u8] = match (state, is_scholar()) {
        (true, true) | (true, false) => &[0x90; 5],
        (false, true) => &[0xE8, 0x71, 0x01, 0x00, 0x00],
        (false, false) => &[0xE8, 0xF7, 0xF5, 0xFF, 0xFF],
    };
    write_bytes(Patch::NoSoulGain, bytes)
}

pub fn is_no_soul_gain() -> bool {
    read::<[u8; 5]> (Patch::NoSoulGain)
        .map(|val| val == [0x90; 5])
        .unwrap_or_default()
}

pub fn set_no_soul_loss(state: bool) -> ProcResult {
    let bytes: &[u8] = match (state, is_scholar()) {
        (true, true) => &[0x90; 6],
        (true, false) => &[0x90; 10],
        (false, true) => &[0x89, 0x90, 0xEC, 0x00, 0x00, 0x00],
        (false, false) => &[0xC7, 0x80, 0xE8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    };
    write_bytes(Patch::NoSoulLoss, bytes)
}

pub fn is_no_soul_loss() -> bool {
    read::<[u8; 6]> (Patch::NoSoulLoss)
        .map(|val| val == [0x90; 6])
        .unwrap_or_default()
}

pub fn set_infinite_stamina(state: bool) -> ProcResult {
    let byte = if state { 0x82 } else { 0x83 };
    write::<u8>(Patch::InfiniteStamina, byte)
}

pub fn is_infinite_stamina() -> bool {
    read::<u8>(Patch::InfiniteStamina)
        .map(|val| val != 0x83)
        .unwrap_or_default()
}

pub fn set_hidden(state: bool) -> ProcResult {
    let byte = if state { 0x85 } else { 0x84 };
    write::<u8>(Patch::PlayerHidden, byte)
}

pub fn is_hidden() -> bool {
    read::<u8>(Patch::PlayerHidden)
        .map(|val| val != 0x84)
        .unwrap_or_default()
}

pub fn set_silent(state: bool) -> ProcResult {
    match is_scholar() {
        true => {
            if state {
                write_bytes(Patch::PlayerSilent, &[0x90; 5])
            } else {
                let mut bytes = vec![0xE8; 5];
                write_rel_i32(&mut bytes, Patch::PlayerSilent, 1, Function::MakeSound, 4)?;
                write_bytes(Patch::PlayerSilent, &bytes)
            }
        },
        false => {
            let push_op_neg_offset = match version() {
                Some(DarkSouls2Version::Vanilla1_0_12) => 4,
                _ => 1,
            };
            if state {
                write_bytes(Patch::PlayerSilent, &[0x90; 15])?;
                write::<u8>(Patch::PlayerSilent.sub_offset(push_op_neg_offset), 0x90)
            } else {
                let mut bytes = match version() {
                    Some(DarkSouls2Version::Vanilla1_0_12) => vec![
                        0xF3, 0x0F, 0x11, 0x04, 0x24, 0x51, 0x52, 0x53, 0x8B, 0xCF,
                        0xE8, 0x00, 0x00, 0x00, 0x00,
                    ],
                    _ => vec![
                        0xF3, 0x0F, 0x11, 0x04, 0x24, 0x52, 0x50, 0x53, 0x8B, 0xCF,
                        0xE8, 0x00, 0x00, 0x00, 0x00,
                    ],
                };
                write_rel_i32(&mut bytes, Patch::PlayerSilent, 11, Function::MakeSound, 4)?;
                write_bytes(Patch::PlayerSilent, &bytes)?;
                write::<u8>(Patch::PlayerSilent.sub_offset(push_op_neg_offset), 0x51)
            }
        }
    }
}

pub fn is_silent() -> bool {
    read::<[u8; 5]> (Patch::PlayerSilent)
        .map(|val| val == [0x90; 5])
        .unwrap_or_default()
}