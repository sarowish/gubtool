use crate::{Config, user::{AttachConfig, AttachConfigTrait}};
use anyhow::Result;
use eldenring::{
    chr_ins::ChrInsExt,
    game_state,
    game_state::StateFlagOffset,
    player::{self, ChrDbgOffsets},
    utility,
};
use gubtool_core::sys::error::ProcessError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ErAttach {
    pub no_death: bool,
    pub no_damage: bool,
    pub rfbs_on_load: bool,
    pub infinite_poise: bool,
    pub fps: Option<f32>,
    pub remove_logo: bool,
    pub mute_music: bool,
    pub disable_area_target_cards: bool,
    pub stutter_fix: bool,
    pub map_in_combat: bool,
    pub travel_in_dungeon: bool,
}

impl AttachConfigTrait for ErAttach {
    fn apply_and_collect_errors(&self) -> Vec<ProcessError> {
        let mut errors = Vec::new();

        if self.no_death {
            if let Err(err) = player::set_chr_dbg_flag(ChrDbgOffsets::PlayerNoDeath, true) {
                errors.push(err);
            }
        }
        if self.no_damage {
            if let Err(err) = game_state::StateFlags::set(StateFlagOffset::PlayerNoDamage, true) {
                errors.push(err);
            }
            let _ = player::player_ins().set_no_damage(true);
        }
        if self.rfbs_on_load {
            if let Err(err) = game_state::StateFlags::set(StateFlagOffset::Rfbs, true) {
                errors.push(err);
            }
        }
        if self.infinite_poise {
            if let Err(err) = player::set_infinite_poise(true) {
                errors.push(err);
            }
        }
        if let Some(val) = self.fps {
            if let Err(err) = utility::set_fps_cap(val) {
                errors.push(err);
            }
        }
        if self.remove_logo {
            if let Err(err) = utility::set_logo_patch(true) {
                errors.push(err);
            }
        }
        if self.mute_music {
            if let Err(err) = utility::mute_music(true) {
                errors.push(err);
            }
        }
        if self.stutter_fix {
            if let Err(err) = game_state::StateFlags::set(StateFlagOffset::StutterFix, true) {
                errors.push(err);
            }
        }
        if self.disable_area_target_cards {
            if let Err(err) = game_state::StateFlags::set(StateFlagOffset::TitleCards, true) {
                errors.push(err);
            }
        }
        if self.map_in_combat {
            if let Err(err) = utility::set_map_anywhere(true) {
                errors.push(err);
            }
        }
        if self.travel_in_dungeon {
            if let Err(err) = utility::set_travel_anywhere(true) {
                errors.push(err);
            }
        }
        errors
    }
}

impl ErAttach {
    pub fn update<F>(modifier: F) -> Result<()>
    where
        F: FnOnce(&mut Self),
    {
        let mut er_conf: Self = AttachConfig::read().unwrap_or_default().elden_ring;
        modifier(&mut er_conf);
        AttachConfig::update(|c| c.elden_ring = er_conf)
    }
}
