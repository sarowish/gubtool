#[repr(u32)]
#[derive(Clone, Copy)]
pub enum MapId {
    ThingsBetwixed = 0xA020000,                     // m10_02_00_00
    Majula = 0xA040000,                             // m10_04_00_00
    ForestOfFallenGiants = 0xA0A0000,               // m10_10_00_00
    BrightstoneCoveTseldora = 0xA0E0000,            // m10_14_00_00
    AldiasKeep = 0xA0F0000,                         // m10_15_00_00
    TheLostBastille = 0xA100000,                    // m10_16_00_00
    HarvestValley = 0xA110000,                      // m10_17_00_00
    NoMansWharf = 0xA120000,                        // m10_18_00_00
    IronKeep = 0xA130000,                           // m10_19_00_00
    HuntsmansCorpse = 0xA170000,                    // m10_23_00_00
    TheGutter = 0xA190000,                          // m10_25_00_00
    DragonAerie = 0xA1B0000,                        // m10_27_00_00
    PathToShadedWoods = 0xA1D0000,                  // m10_29_00_00
    PathToNoMansWharf = 0xA1E0000,                  // m10_30_00_00
    HeidesTowerOfFlame = 0xA1F0000,                 // m10_31_00_00
    ShadedWoods = 0xA200000,                        // m10_32_00_00
    DoorsOfPharros = 0xA210000,                     // m10_33_00_00
    GraveOfSaints = 0xA220000,                      // m10_34_00_00
    GiantsMemory = 0x140A0000,                      // m20_10_00_00
    ShrineOfAmana = 0x140B0000,                     // m20_11_00_00
    DrangleicCastle = 0x14150000,                   // m20_21_00_00
    UndeadCrypt = 0x14180000,                       // m20_24_00_00
    DragonsMemory = 0x141A0000,                     // m20_26_00_00
    DarkChasmOfOld = 0x28030000,                    // m40_03_00_00
    ShulvaSanctumCity = 0x32230000,                 // m50_35_00_00
    BrumeTower = 0x32240000,                        // m50_36_00_00
    FrozenEleumLoyce = 0x32250000,                  // m50_37_00_00
    KingsMemory = 0x32260000,                       // m50_38_00_00
}