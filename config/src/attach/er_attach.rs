use crate::attach::{AttachConfig, AttachEntry, BoolEntryTrait, FloatEntryTrait};
use eldenring::{
    chr_ins::ChrInsExt,
    game_state,
    game_state::StateFlagOffset,
    player::{self, ChrDbgOffset},
    utility,
};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ErAttachConfig {
    pub no_death: bool,
    pub no_damage: bool,
    pub infinite_poise: bool,
    pub one_shot: bool,
    pub rfbs_on_load: bool,
    pub silent: bool,
    pub hidden: bool,
    pub infinite_stamina: bool,
    pub infinite_fp: bool,
    pub infinite_consumables: bool,
    pub infinite_arrows: bool,
    pub torrent_anywhere: bool,
    pub torrent_no_death: bool,
    pub disable_logos: bool,
    pub fps: Option<f32>,
    pub game_speed: Option<f32>,
    pub mute_music: bool,
    pub disable_area_welcome_message: bool,
    pub stutter_fix: bool,
    pub map_in_combat: bool,
    pub travel_in_dungeon: bool,
    pub draw_hitboxes: bool,
    pub show_all_graces: bool,
    pub show_all_maps: bool,
    pub disable_roll: bool,
    pub disable_jump: bool,
    pub disable_backstep: bool,
}

#[derive(Display)]
#[strum(serialize_all = "title_case")]
enum BoolEntry {
    NoDeath,
    NoDamage,
    InfinitePoise,
    OneShot,
    SetRfbsOnLoad,
    Silent,
    Hidden,
    InfiniteStamina,
    InfiniteFp,
    InfiniteConsumables,
    InfiniteArrows,
    TorrentAnywhere,
    TorrentNoDeath,

    MuteMusic,
    DisableLogos,
    DisableAreaWelcomeMessage,
    StutterFix,
    DrawHitboxes,
    ShowAllGraces,
    ShowAllMaps,
    AllowMapInCombat,
    AllowTravelInDungeon,
    DisableRoll,
    DisableJump,
    DisableBackstep,
}

#[derive(Display)]
#[strum(serialize_all = "title_case")]
enum FloatEntry {
    Fps,
    GameSpeed,
}

macro_rules! match_bool_field {
    ($self:expr, $s:expr, $($acc:tt)+) => {
        match $self {
            Self::NoDeath => $($acc)+ $s.no_death,
            Self::NoDamage => $($acc)+ $s.no_damage,
            Self::InfinitePoise => $($acc)+ $s.infinite_poise,
            Self::OneShot => $($acc)+ $s.one_shot,
            Self::SetRfbsOnLoad => $($acc)+ $s.rfbs_on_load,
            Self::Silent => $($acc)+ $s.silent,
            Self::Hidden => $($acc)+ $s.hidden,
            Self::InfiniteStamina => $($acc)+ $s.infinite_stamina,
            Self::InfiniteFp => $($acc)+ $s.infinite_fp,
            Self::InfiniteConsumables => $($acc)+ $s.infinite_consumables,
            Self::InfiniteArrows => $($acc)+ $s.infinite_arrows,
            Self::TorrentAnywhere => $($acc)+ $s.torrent_anywhere,
            Self::TorrentNoDeath => $($acc)+ $s.torrent_no_death,
            Self::MuteMusic => $($acc)+ $s.mute_music,
            Self::DisableLogos => $($acc)+ $s.disable_logos,
            Self::DisableAreaWelcomeMessage => $($acc)+ $s.disable_area_welcome_message,
            Self::StutterFix => $($acc)+ $s.stutter_fix,
            Self::DrawHitboxes => $($acc)+ $s.draw_hitboxes,
            Self::ShowAllGraces => $($acc)+ $s.show_all_graces,
            Self::ShowAllMaps => $($acc)+ $s.show_all_maps,
            Self::AllowMapInCombat => $($acc)+ $s.map_in_combat,
            Self::AllowTravelInDungeon => $($acc)+ $s.travel_in_dungeon,
            Self::DisableRoll => $($acc)+ $s.disable_roll,
            Self::DisableJump => $($acc)+ $s.disable_jump,
            Self::DisableBackstep => $($acc)+ $s.disable_backstep,
        }
    };
}

impl BoolEntryTrait for BoolEntry {
    fn get<'a>(&self, conf: &'a AttachConfig) -> &'a bool {
        match_bool_field!(self, conf.elden_ring, &)
    }

    fn get_mut<'a>(&self, conf: &'a mut AttachConfig) -> &'a mut bool {
        match_bool_field!(self, conf.elden_ring, &mut)
    }

    fn apply(&self, conf: &mut AttachConfig) -> anyhow::Result<()> {
        let apply = self.get(conf);
        if !*apply {
            return Ok(())
        }
        match self {
            Self::NoDeath => player::set_chr_dbg_flag(ChrDbgOffset::PlayerNoDeath, true)?,
            Self::NoDamage => {
                game_state::StateFlags::set(StateFlagOffset::PlayerNoDamage, true)?;
                let _ = player::player_ins().set_no_damage(true);
            }
            Self::InfinitePoise => player::set_infinite_poise(true)?,
            Self::OneShot => player::set_chr_dbg_flag(ChrDbgOffset::OneShot, true)?,
            Self::SetRfbsOnLoad => game_state::StateFlags::set(StateFlagOffset::Rfbs, true)?,
            Self::Silent => player::set_chr_dbg_flag(ChrDbgOffset::Silent, true)?,
            Self::Hidden => player::set_chr_dbg_flag(ChrDbgOffset::Hidden, true)?,
            Self::InfiniteStamina => player::set_chr_dbg_flag(ChrDbgOffset::InfiniteStam, true)?,
            Self::InfiniteFp => player::set_chr_dbg_flag(ChrDbgOffset::InfiniteFp, true)?,
            Self::InfiniteConsumables => player::set_chr_dbg_flag(ChrDbgOffset::InfiniteGoods, true)?,
            Self::InfiniteArrows => player::set_chr_dbg_flag(ChrDbgOffset::InfiniteArrows, true)?,
            Self::TorrentAnywhere => player::set_torrent_anywhere(true)?,
            Self::TorrentNoDeath => {
                game_state::StateFlags::set(StateFlagOffset::TorrentNoDeath, true)?;
                let _ = player::torrent_ins().set_no_death(true);
            }
            Self::MuteMusic => utility::mute_music(true)?,
            Self::DisableLogos => utility::set_logo_patch(true)?,
            Self::DisableAreaWelcomeMessage => game_state::StateFlags::set(StateFlagOffset::TitleCards, true)?,
            Self::StutterFix => game_state::StateFlags::set(StateFlagOffset::StutterFix, true)?,
            Self::DrawHitboxes => {
                game_state::StateFlags::set(StateFlagOffset::Hitboxes, true)?;
                let _ = utility::draw_hitboxes(true, false);
            },
            Self::ShowAllGraces => utility::show_all_graces(true)?,
            Self::ShowAllMaps => utility::show_all_maps(true)?,
            Self::AllowMapInCombat => utility::set_map_anywhere(true)?,
            Self::AllowTravelInDungeon => utility::set_travel_anywhere(true)?,
            Self::DisableRoll => utility::set_control(utility::ControlFlag::Roll, true)?,
            Self::DisableJump => utility::set_control(utility::ControlFlag::Jump, true)?,
            Self::DisableBackstep => utility::set_control(utility::ControlFlag::Backstep, true)?,
        }
        Ok(())
    }
}

impl FloatEntryTrait for FloatEntry {
    fn get<'a>(&self, conf: &'a AttachConfig) -> &'a Option<f32> {
        let er = &conf.elden_ring;
        match self {
            Self::Fps => &er.fps,
            Self::GameSpeed => &er.game_speed,
        }
    }

    fn get_mut<'a>(&self, conf: &'a mut AttachConfig) -> &'a mut Option<f32> {
        let er = &mut conf.elden_ring;
        match self {
            Self::Fps => &mut er.fps,
            Self::GameSpeed => &mut er.game_speed,
        }
    }

    fn apply(&self, conf: &mut AttachConfig) -> anyhow::Result<()> {
        if let Some(val) = *self.get(conf) {
            match self {
                Self::Fps => utility::set_fps_cap(val)?,
                Self::GameSpeed => utility::set_game_speed(val)?,
            }
        }
        Ok(())
    }
}

pub struct ErEntries {
    pub player: Vec<AttachEntry>,
    pub utility: Vec<AttachEntry>,
}

impl ErEntries {
    pub fn get_iter(&self) -> Box<dyn Iterator<Item = &AttachEntry> + '_> {
        Box::new(self.player.iter()
            .chain(self.utility.iter()))
    }
    pub fn new() -> Self {
        let player = vec![
            BoolEntry::NoDeath.to_attach_entry(),
            BoolEntry::NoDamage.to_attach_entry(),
            BoolEntry::InfinitePoise.to_attach_entry(),
            BoolEntry::OneShot.to_attach_entry(),
            BoolEntry::SetRfbsOnLoad.to_attach_entry(),
            BoolEntry::Silent.to_attach_entry(),
            BoolEntry::Hidden.to_attach_entry(),
            BoolEntry::InfiniteStamina.to_attach_entry(),
            BoolEntry::InfiniteFp.to_attach_entry(),
            BoolEntry::InfiniteConsumables.to_attach_entry(),
            BoolEntry::InfiniteArrows.to_attach_entry(),
            BoolEntry::TorrentAnywhere.to_attach_entry(),
            BoolEntry::TorrentNoDeath.to_attach_entry(),
        ];
        let utility = vec![
            FloatEntry::Fps.to_attach_entry(),
            FloatEntry::GameSpeed.to_attach_entry(),
            BoolEntry::MuteMusic.to_attach_entry(),
            BoolEntry::DisableLogos.to_attach_entry(),
            BoolEntry::DisableAreaWelcomeMessage.to_attach_entry(),
            BoolEntry::StutterFix.to_attach_entry(),
            BoolEntry::DrawHitboxes.to_attach_entry(),
            BoolEntry::ShowAllGraces.to_attach_entry(),
            BoolEntry::ShowAllMaps.to_attach_entry(),
            BoolEntry::AllowMapInCombat.to_attach_entry(),
            BoolEntry::AllowTravelInDungeon.to_attach_entry(),
            BoolEntry::DisableRoll.to_attach_entry(),
            BoolEntry::DisableJump.to_attach_entry(),
            BoolEntry::DisableBackstep.to_attach_entry(),
        ];
        Self { player, utility }
    }
}