use crate::{
    chr_ctrl::ChrCtrlExt,
    mem::*,
    offsets::{code_cave::CaveOffset, game_manager_imp},
    player, utility,
};
use anyhow::Result;
use shared::slice_ops::*;

pub struct GameStateHandler {
    pub loaded: bool,
    has_invoked_loaded: bool,
    has_invoked_load_delayed: bool,
}

#[derive(Default, Clone, Copy)]
pub struct StateFlags {
    pub player_no_death: bool,
    pub fast_quitout: bool,
}

impl GameStateHandler {
    pub fn new() -> Self {
        Self {
            loaded: true,
            has_invoked_loaded: true,
            has_invoked_load_delayed: true,
        }
    }
    pub fn poll(&mut self) -> Result<()> {
        if is_loaded().unwrap_or_default() {
            if !self.has_invoked_load_delayed && !is_loading_screen().unwrap_or_default() {
                self.on_load_delayed()?;
                self.has_invoked_load_delayed = true;
            }
            if !self.loaded {
                self.loaded = true;
                self.on_loaded()?;
                self.has_invoked_loaded = true;
            }
        } else if self.loaded {
            self.on_unloaded()?;
            self.has_invoked_load_delayed = false;
            self.has_invoked_loaded = false;
            self.loaded = false;
        }
        Ok(())
    }
    fn on_loaded(&self) -> Result<()> {
        let flags = StateFlags::new()?;

        if flags.player_no_death {
            player::player_ctrl().set_no_death(true)?
        }

        Ok(())
    }
    fn on_load_delayed(&self) -> Result<()> {
        Ok(())
    }
    fn on_unloaded(&self) -> Result<()> {
        let flags = StateFlags::new()?;

        if flags.fast_quitout {
            utility::set_faster_menu(true)?
        } else if utility::is_faster_menu()? {
            utility::set_faster_menu(false)?
        }

        Ok(())
    }
}

impl StateFlags {
    pub fn new() -> Result<Self> {
        let mut flags = Self::default();
        flags.update()?;
        Ok(flags)
    }
    pub fn update(&mut self) -> Result<()> {
        let flags = read::<[u8; 0x100]>(CaveOffset::StateHandlerFlags.addr())?;
        self.player_no_death = read_flag_from_slice(&flags, StateFlagOffset::PlayerNoDeath)?;
        self.fast_quitout = read_flag_from_slice(&flags, StateFlagOffset::FastQuitout)?;
        Ok(())
    }
    pub fn set(flag_offset: StateFlagOffset, state: bool) -> Result<()> {
        write::<u8>(CaveOffset::StateHandlerFlags.addr() + flag_offset as u64, state as u8)
    }
    pub const fn const_default() -> Self {
        Self {
            player_no_death: false,
            fast_quitout: false,
        }
    }
}

#[repr(u64)]
pub enum StateFlagOffset {
    PlayerNoDeath = 0x0,
    FastQuitout = 0x1,
}

fn read_flag_from_slice(flags: &[u8; 0x100], flag_offset: StateFlagOffset) -> Result<bool> {
    read_from_slice::<u8>(flags, flag_offset as u64).map(|val| val != 0x0)
}

pub fn is_loading_screen() -> Result<bool> {
    read::<u64>(game_manager_imp::base_ptr())
        .and_then(|addr| read::<u8>(addr + game_manager_imp::loading_flag()))
        .map(|val| val == 1)
}

pub fn is_loaded() -> Result<bool> {
    read_address(game_manager_imp::base_ptr())
        .and_then(|addr| read_address(addr + game_manager_imp::player_ctrl()))
        .map(|val| val != 0)
}
