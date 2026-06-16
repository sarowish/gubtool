use crate::{
    chr_ctrl::ChrCtrlExt,
    mem::*,
    offsets::{ChainReadExt, code_cave::CaveOffset, game_manager_imp},
    player, utility,
};
use gubtool_core::sys::error::ProcResult;
use utils::slice_ops::*;

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
    pub fn poll(&mut self) -> ProcResult {
        if is_loaded() {
            if !self.has_invoked_load_delayed && !is_loading_screen() {
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
    fn on_loaded(&self) -> ProcResult {
        let flags = StateFlags::new()?;

        if flags.player_no_death {
            player::player_ctrl().set_no_death(true)?
        }

        Ok(())
    }
    fn on_load_delayed(&self) -> ProcResult {
        Ok(())
    }
    fn on_unloaded(&self) -> ProcResult {
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
    pub fn new() -> ProcResult<Self> {
        let mut flags = Self::default();
        flags.update()?;
        Ok(flags)
    }
    pub fn update(&mut self) -> ProcResult {
        let flags = read::<[u8; 0x100]>(CaveOffset::StateHandlerFlags.addr())?;
        self.player_no_death = read_flag_from_slice(&flags, StateFlagOffset::PlayerNoDeath)?;
        self.fast_quitout = read_flag_from_slice(&flags, StateFlagOffset::FastQuitout)?;
        Ok(())
    }
    pub fn set(flag_offset: StateFlagOffset, state: bool) -> ProcResult {
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

fn read_flag_from_slice(flags: &[u8; 0x100], flag_offset: StateFlagOffset) -> Result<bool, SliceError> {
    read_from_slice::<u8>(flags, flag_offset as u64).map(|val| val != 0x0)
}

pub fn is_loading_screen() -> bool {
    read_address(game_manager_imp::base_ptr())
        .add_offset(game_manager_imp::LOADING_FLAG)
        .read::<u8>()
        .map(|val| val == 0x1)
        .unwrap_or_default()
}

pub fn is_loaded() -> bool {
    read_address(game_manager_imp::base_ptr())
        .read_offset(game_manager_imp::PLAYER_CTRL)
        .map(|val| val != 0x0)
        .unwrap_or_default()
}