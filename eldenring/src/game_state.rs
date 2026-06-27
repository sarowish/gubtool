use crate::{
    chr_ins::{ChrInsExt, chr_ins_from_handle},
    emevd,
    mem::*,
    offsets::{
        ChainReadExt, code_cave::CaveOffset, game_data_man, menu_man, module_offsets::BasePointer,
        world_chr_man,
    },
    player::{self, player_ins, torrent_ins},
    target::target_ins,
    utility,
    utils::{is_dlc_available, is_version_dlc_compat},
};
use gubtool_core::{address::Address, slice_ops::*, sys::error::ProcResult};

#[derive(Default)]
pub struct GameStateHandler {
    pub loaded: bool,
    has_invoked_load_delayed: bool,
    has_invoked_loaded: bool,
    pub dlc: bool,
}

#[derive(Default, Clone, Copy)]
pub struct StateFlags {
    pub player_no_damage: bool,
    pub rfbs: bool,
    pub title_cards: bool,
    pub rune_arc: bool,
    pub torrent_no_death: bool,
    pub stutter_fix: bool,
    pub hitboxes: bool,
}

impl GameStateHandler {
    pub fn new() -> Self {
        Self {
            loaded: false,
            has_invoked_load_delayed: true,
            has_invoked_loaded: true,
            dlc: is_version_dlc_compat(),
        }
    }

    pub fn poll(&mut self) -> ProcResult {
        if is_loaded() {
            if !self.has_invoked_load_delayed && self.has_invoked_loaded && is_faded_in() {
                self.on_load_delayed()?;
                self.has_invoked_load_delayed = true;
            }
            if !self.loaded {
                self.loaded = true;
                self.on_loaded()?;
                self.has_invoked_loaded = true;

                if is_new_game() {
                    self.on_new_game()?;
                }
            }
        } else if self.loaded {
            self.on_unloaded();
            self.has_invoked_load_delayed = false;
            self.has_invoked_loaded = false;
            self.loaded = false;
        }
        Ok(())
    }
    fn on_loaded(&mut self) -> ProcResult {
        let flags = StateFlags::new()?;

        if flags.player_no_damage {
            player_ins().set_no_damage(true)?;
        }
        if flags.title_cards {
            emevd::disable_title_card()?;
        }
        if flags.rune_arc {
            player::set_rune_arc(true)?;
        }
        if flags.stutter_fix {
            utility::set_stutter_fix(true)?;
        }
        if flags.hitboxes {
            utility::draw_hitboxes(true, false)?;
        }

        let handle = read::<u64>(CaveOffset::LookedUpHandle)?;
        write::<u64>(CaveOffset::SavedTargetPointer, chr_ins_from_handle(handle).unwrap_or_default())?;

        self.dlc = is_dlc_available();
        Ok(())
    }

    fn on_load_delayed(&self) -> ProcResult {
        let flags = StateFlags::new()?;

        if flags.rfbs {
            player::set_rfbs()?;
        }
        if flags.torrent_no_death {
            torrent_ins().set_no_death(true)?;
        }
        Ok(())
    }

    fn on_unloaded(&self) {
        write::<u64>(CaveOffset::LookedUpHandle, target_ins().handle().unwrap_or_default()).ok();
    }

    fn on_new_game(&self) -> ProcResult {
        Ok(())
    }
}

impl StateFlags {
    pub fn new() -> ProcResult<Self> {
        let mut flags = Self::default();
        flags.update()?;
        Ok(flags)
    }
    pub fn update(&mut self) -> ProcResult {
        let flags = read::<[u8; 0x100]>(CaveOffset::StateHandlerFlags)?;
        self.player_no_damage = read_flag_from_slice(&flags, StateFlagOffset::PlayerNoDamage)?;
        self.rfbs = read_flag_from_slice(&flags, StateFlagOffset::Rfbs)?;
        self.title_cards = read_flag_from_slice(&flags, StateFlagOffset::TitleCards)?;
        self.rune_arc = read_flag_from_slice(&flags, StateFlagOffset::RuneArc)?;
        self.torrent_no_death = read_flag_from_slice(&flags, StateFlagOffset::TorrentNoDeath)?;
        self.stutter_fix = read_flag_from_slice(&flags, StateFlagOffset::StutterFix)?;
        self.hitboxes = read_flag_from_slice(&flags, StateFlagOffset::Hitboxes)?;
        Ok(())
    }
    pub fn set(flag_offset: StateFlagOffset, state: bool) -> ProcResult {
        write::<u8>(CaveOffset::StateHandlerFlags.add_offset(flag_offset as u64), state as u8)
    }
    pub const fn const_default() -> Self {
        Self {
            player_no_damage: false,
            rfbs: false,
            title_cards: false,
            rune_arc: false,
            torrent_no_death: false,
            stutter_fix: false,
            hitboxes: false,
        }
    }
}

#[repr(u64)]
pub enum StateFlagOffset {
    PlayerNoDamage = 0x0,
    Rfbs = 0x1,
    TitleCards = 0x2,
    RuneArc = 0x3,
    TorrentNoDeath = 0x4,
    StutterFix = 0x5,
    Hitboxes = 0x6,
}

fn read_flag_from_slice(flags: &[u8; 0x100], flag_offset: StateFlagOffset) -> Result<bool, SliceError> {
    read_from_slice::<u8>(flags, flag_offset as u64).map(|val| val != 0x0)
}

pub fn is_loaded() -> bool {
    read::<u64>(BasePointer::WorldChrMan)
        .read_offset(world_chr_man::player_ins())
        .map(|val| val != 0)
        .unwrap_or_default()
}

fn is_faded_in() -> bool {
    read::<u64>(BasePointer::MenuMan)
        .add_offset(menu_man::is_fading())
        .read::<u8>()
        .map(|val| val == 0x0)
        .unwrap_or_default()
}

fn is_new_game() -> bool {
    read::<u64>(BasePointer::GameDataMan)
        .add_offset(game_data_man::IGT)
        .read::<u64>()
        .map(|val| val < 5000)
        .unwrap_or_default()
}