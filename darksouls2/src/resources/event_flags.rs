use crate::{event, resources::areas::AreaId, utility, utils::character_loaded_check};
use anyhow::{Result, ensure};

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum EventFlag {
    GiantLordDefeated = 100972,
    ThroneDuoDefeated = 100974,
    NashandraDefeated = 100973,
    VendrickDefeated = 100978,
    UnlockAldia = 100747,
    KingsRingAcquired = 100804,
    VisibleAava = 537000012,
    FridgidSnowstorm = 537010014,
    ShadedWoodsChasmCleared = 403000001,
    DrangleicCastleChasmCleared = 403000002,
    BlackGulchChasmCleared = 403000003,
    ActivateBrume = 536000010,
    EleumLoyceWinds = 537000001,
    EleumLoyceIce = 537000011,
    LoyceKnightOuterWall = 537000020,
    LoyceKnightAbandonedDwelling = 537000021,
    LoyceKnightLowerGarrison = 537000022,
}

impl EventFlag {
    pub fn get(&self) -> Result<bool> {
        event::get_event_flag(*self as u32)
    }

    pub fn set(&self, state: bool) -> Result<()> {
        event::set_event_flag(*self as u32, state)
    }

    pub fn get_flags(flags: &[Self]) -> Result<bool> {
        for flag in flags {
            if !event::get_event_flag(*flag as u32)? {
               return Ok(false);
            }
        }
        Ok(true)
    }

    pub fn set_flags(flags: &[(Self, bool)]) -> Result<()> {
        flags.iter().try_for_each(|(flag, state)| event::set_event_flag(*flag as u32, *state))
    }

    pub fn set_area_conditional_event(&self, state: bool, area_id: AreaId) -> Result<()> {
        character_loaded_check()?;
        ensure!(utility::get_area_id()? == area_id as u32, "Must be in general area");
        event::set_event_flag(*self as u32, state)
    }
}