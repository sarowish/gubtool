use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    config::{Config, user::AttachConfig},
    er::{
        self, chr_ins::ChrInsExt, game_state::GameStateFlags, offsets::chr_dbg_flags::ChrDbgOffsets,
    },
};

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
}

impl ErAttach {
    pub fn apply(&self) -> Result<()> {
        if self.no_death {
            er::player::set_chr_dbg_flag(ChrDbgOffsets::PlayerNoDeath, true)?
        }
        if self.no_damage {
            er::game_state::set_state_flag(GameStateFlags::PlayerNoDamage, true)?;
            er::player::player_ins().set_no_damage(true)?
        }
        if self.rfbs_on_load {
            er::game_state::set_state_flag(GameStateFlags::Rfbs, true)?
        }
        if self.infinite_poise {
            er::player::set_infinite_poise(true)?
        }
        if let Some(val) = self.fps {
            er::utility::set_fps_cap(val)?
        }
        if self.remove_logo {
            er::utility::set_logo_patch(true)?
        }
        if self.mute_music {
            er::utility::mute_music(true)?
        }
        if self.stutter_fix {
            er::game_state::set_state_flag(GameStateFlags::StutterFix, true)?
        }
        if self.disable_area_target_cards {
            er::game_state::set_state_flag(GameStateFlags::TitleCards, true)?
        }
        Ok(())
    }
    pub fn update<F>(modifier: F) -> Result<()>
    where
        F: FnOnce(&mut Self),
    {
        let mut er_conf: Self = AttachConfig::read().unwrap_or_default().elden_ring;
        modifier(&mut er_conf);
        AttachConfig::update(|c| c.elden_ring = er_conf)
    }
}
