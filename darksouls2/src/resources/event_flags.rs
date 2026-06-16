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