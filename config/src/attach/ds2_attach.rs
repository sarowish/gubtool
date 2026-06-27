use crate::attach::{AttachConfig, AttachEntry, BoolEntryTrait};
use darksouls2::{
    event, game_state::{self, StateFlagOffset}, player, utility
};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Ds2AttachConfig {
    pub no_death: bool,
    pub no_damage: bool,
    pub infinite_poise: bool,
    pub infinite_stamina: bool,
    pub infinite_durability: bool,
    pub infinite_consumables: bool,
    pub no_hollowing: bool,
    pub no_soul_loss: bool,
    pub hidden: bool,
    pub silent: bool,
    pub skip_credits: bool,
    pub fast_quitout: bool,
    pub disable_roll: bool,
    pub disable_backstep: bool,
    pub gauntlet_skip: bool,
    pub disable_loyce: bool,
    pub start_logger: bool,
}

#[derive(Display)]
#[strum(serialize_all = "title_case")]
enum BoolEntry {
    NoDeath,
    NoDamage,
    InfinitePoise,
    InfiniteStamina,
    InfiniteDurability,
    InfiniteConsumables,
    NoHollowing,
    NoSoulLoss,
    Hidden,
    Silent,

    SkipCredits,
    FastQuitout,
    DisableRoll,
    DisableBackstep,
    IvoryKingGauntletSkip,
    DisableLoyceKnights,
    StartEventLogger,
}

macro_rules! match_bool_field {
    ($self:expr, $s:expr, $($acc:tt)+) => {
        match $self {
            Self::NoDeath => $($acc)+ $s.no_death,
            Self::NoDamage => $($acc)+ $s.no_damage,
            Self::InfinitePoise => $($acc)+ $s.infinite_poise,
            Self::InfiniteStamina => $($acc)+ $s.infinite_stamina,
            Self::InfiniteDurability => $($acc)+ $s.infinite_durability,
            Self::InfiniteConsumables => $($acc)+ $s.infinite_consumables,
            Self::NoHollowing => $($acc)+ $s.no_hollowing,
            Self::NoSoulLoss => $($acc)+ $s.no_soul_loss,
            Self::Hidden => $($acc)+ $s.hidden,
            Self::Silent => $($acc)+ $s.silent,
            Self::SkipCredits => $($acc)+ $s.skip_credits,
            Self::FastQuitout => $($acc)+ $s.fast_quitout,
            Self::DisableRoll => $($acc)+ $s.disable_roll,
            Self::DisableBackstep => $($acc)+ $s.disable_backstep,
            Self::IvoryKingGauntletSkip => $($acc)+ $s.gauntlet_skip,
            Self::DisableLoyceKnights => $($acc)+ $s.disable_loyce,
            Self::StartEventLogger => $($acc)+ $s.start_logger,
        }
    };
}

impl BoolEntryTrait for BoolEntry {
    fn get<'a>(&self, conf: &'a AttachConfig) -> &'a bool {
        match_bool_field!(self, conf.dark_souls_2, &)
    }

    fn get_mut<'a>(&self, conf: &'a mut AttachConfig) -> &'a mut bool {
        match_bool_field!(self, conf.dark_souls_2, &mut)
    }

    fn apply(&self, conf: &mut AttachConfig) -> anyhow::Result<()> {
        let apply = self.get(conf);
        if !*apply {
            return Ok(())
        }
        match self {
            Self::NoDeath => game_state::StateFlags::set(StateFlagOffset::PlayerNoDeath, true)?,
            Self::NoDamage => player::set_no_damage(true)?,
            Self::InfinitePoise => player::set_infinite_poise(true)?,
            Self::InfiniteStamina => player::set_infinite_stamina(true)?,
            Self::InfiniteDurability => player::set_infinite_durability(true)?,
            Self::InfiniteConsumables => player::set_infinite_consumables(true)?,
            Self::NoHollowing => player::set_no_hollowing(true)?,
            Self::NoSoulLoss => player::set_no_soul_loss(true)?,
            Self::Hidden => player::set_hidden(true)?,
            Self::Silent => player::set_silent(true)?,
            Self::SkipCredits => utility::set_credits_skip(true)?,
            Self::FastQuitout => game_state::StateFlags::set(StateFlagOffset::FastQuitout, true)?,
            Self::DisableRoll => utility::set_disable_roll(true)?,
            Self::DisableBackstep => utility::set_disable_backstep(true)?,
            Self::IvoryKingGauntletSkip => event::set_ivory_gauntlet_skip(true)?,
            Self::DisableLoyceKnights => event::set_ivory_no_knights(true)?,
            Self::StartEventLogger => event::set_event_log_hook(true)?,
        }
        Ok(())
    }
}

pub struct Ds2Entries {
    pub player: Vec<AttachEntry>,
    pub utility: Vec<AttachEntry>,
}

impl Ds2Entries {
    pub fn get_iter(&self) -> Box<dyn Iterator<Item = &AttachEntry> + '_> {
        Box::new(self.player.iter()
            .chain(self.utility.iter()))
    }
    pub fn new() -> Self {
        let player = vec![
            BoolEntry::NoDeath.to_attach_entry(),
            BoolEntry::NoDamage.to_attach_entry(),
            BoolEntry::InfinitePoise.to_attach_entry(),
            BoolEntry::InfiniteStamina.to_attach_entry(),
            BoolEntry::InfiniteDurability.to_attach_entry(),
            BoolEntry::InfiniteConsumables.to_attach_entry(),
            BoolEntry::NoHollowing.to_attach_entry(),
            BoolEntry::NoSoulLoss.to_attach_entry(),
            BoolEntry::Hidden.to_attach_entry(),
            BoolEntry::Silent.to_attach_entry(),
        ];
        let utility = vec![
            BoolEntry::SkipCredits.to_attach_entry(),
            BoolEntry::FastQuitout.to_attach_entry(),
            BoolEntry::DisableRoll.to_attach_entry(),
            BoolEntry::DisableBackstep.to_attach_entry(),
            BoolEntry::IvoryKingGauntletSkip.to_attach_entry(),
            BoolEntry::DisableLoyceKnights.to_attach_entry(),
            BoolEntry::StartEventLogger.to_attach_entry(),
        ];
        Self { player, utility }
    }
}