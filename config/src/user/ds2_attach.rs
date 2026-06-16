use crate::{Config, user::{AttachConfig, AttachConfigTrait, }};
use anyhow::Result;
use darksouls2::{
    event,
    game_state::{self, StateFlagOffset},
    utility,
};
use gubtool_core::{sys::error::ProcessError};
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

impl AttachConfigTrait for Ds2Attach {
    fn apply_and_collect_errors(&self) -> Vec<ProcessError> {
        let mut errors = Vec::new();

        if self.no_death {
            if let Err(err) = game_state::StateFlags::set(StateFlagOffset::PlayerNoDeath, true) {
                errors.push(err);
            }
        }
        if self.gauntlet_skip {
            if let Err(err) = event::set_ivory_gauntlet_skip(true) {
                errors.push(err);
            }
        }
        if self.disable_loyce {
            if let Err(err) = event::set_ivory_no_knights(true) {
                errors.push(err);
            }
        }
        if self.skip_credits {
            if let Err(err) = utility::set_credits_skip(true) {
                errors.push(err);
            }
        }
        if self.fast_quitout {
            if let Err(err) = game_state::StateFlags::set(StateFlagOffset::FastQuitout, true) {
                errors.push(err);
            }
        }
        if self.start_logger {
            if let Err(err) = event::set_event_log_hook(true) {
                errors.push(err);
            }
        }
        errors
    }
}

impl Ds2Attach {
    pub fn update<F>(modifier: F) -> Result<()>
    where
        F: FnOnce(&mut Self),
    {
        let mut ds2_conf: Self = AttachConfig::read().unwrap_or_default().dark_souls_2;
        modifier(&mut ds2_conf);
        AttachConfig::update(|c| c.dark_souls_2 = ds2_conf)
    }
}
