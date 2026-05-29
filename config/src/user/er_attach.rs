use crate::{Config, user::AttachConfig};
use anyhow::Result;
use eldenring::{
    chr_ins::ChrInsExt,
    game_state,
    game_state::StateFlagOffset,
    player::{self, ChrDbgOffsets},
    utility,
};
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

impl ErAttach {
    pub fn apply(&self) -> Result<()> {
        if self.no_death {
            player::set_chr_dbg_flag(ChrDbgOffsets::PlayerNoDeath, true)?
        }
        if self.no_damage {
            game_state::StateFlags::set(StateFlagOffset::PlayerNoDamage, true)?;
            let _ = player::player_ins().set_no_damage(true);
        }
        if self.rfbs_on_load {
            game_state::StateFlags::set(StateFlagOffset::Rfbs, true)?
        }
        if self.infinite_poise {
            player::set_infinite_poise(true)?
        }
        if let Some(val) = self.fps {
            utility::set_fps_cap(val)?
        }
        if self.remove_logo {
            utility::set_logo_patch(true)?
        }
        if self.mute_music {
            utility::mute_music(true)?
        }
        if self.stutter_fix {
            game_state::StateFlags::set(StateFlagOffset::StutterFix, true)?
        }
        if self.disable_area_target_cards {
            game_state::StateFlags::set(StateFlagOffset::TitleCards, true)?
        }
        if self.map_in_combat {
            utility::set_map_anywhere(true)?
        }
        if self.travel_in_dungeon {
            utility::set_travel_anywhere(true)?
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
