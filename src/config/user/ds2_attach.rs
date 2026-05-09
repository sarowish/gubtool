use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    config::{Config, user::AttachConfig},
    ds2::{self, game_state::GameStateFlags},
};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Ds2Attach {
    pub no_death: bool,
    pub ivory_skip: bool,
    pub skip_credits: bool,
    pub fast_quitout: bool,
}

impl Ds2Attach {
    pub fn apply(&self) -> Result<()> {
        if self.no_death {
            ds2::game_state::set_state_flag(GameStateFlags::PlayerNoDeath, true)?
        }
        if self.ivory_skip {
            ds2::utility::set_ivory_skip(true)?
        }
        if self.skip_credits {
            ds2::utility::set_credits_skip(true)?
        }
        if self.fast_quitout {
            ds2::game_state::set_state_flag(GameStateFlags::FastQuitout, true)?
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