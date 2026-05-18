use crate::{Config, user::AttachConfig};
use anyhow::Result;
use darksouls2::{
    event,
    game_state::{self, StateFlagsOffsets},
    utility,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Ds2Attach {
    pub no_death: bool,
    pub gauntlet_skip: bool,
    pub disable_loyce: bool,
    pub skip_credits: bool,
    pub fast_quitout: bool,
    pub start_logger: bool,
}

impl Ds2Attach {
    pub fn apply(&self) -> Result<()> {
        if self.no_death {
            game_state::StateFlags::set(StateFlagsOffsets::PlayerNoDeath, true)?
        }
        if self.gauntlet_skip {
            event::set_ivory_gauntlet_skip(true)?
        }
        if self.disable_loyce {
            event::set_ivory_no_knights(true)?
        }
        if self.skip_credits {
            utility::set_credits_skip(true)?
        }
        if self.fast_quitout {
            game_state::StateFlags::set(StateFlagsOffsets::FastQuitout, true)?
        }
        if self.start_logger {
            event::set_event_log_hook(true)?
        }
        Ok(())
    }

    pub fn update<F>(modifier: F) -> Result<()>
    where
        F: FnOnce(&mut Self),
    {
        let mut ds2_conf: Self = AttachConfig::read().unwrap_or_default().dark_souls_2;
        modifier(&mut ds2_conf);
        AttachConfig::update(|c| c.dark_souls_2 = ds2_conf)
    }
}
