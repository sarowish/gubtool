use std::fmt::Display;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Covenant {
    None = 0,
    HeirsOfTheSun = 1,
    BlueSentinels = 2,
    BrotherhoodOfBlood = 3,
    WayOfBlue = 4,
    RatKing = 5,
    BellKeepers = 6,
    DragonRemnants = 7,
    CompanyOfChampions = 8,
    PilgrimsOfDark = 9,
}

impl TryFrom<u8> for Covenant {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::HeirsOfTheSun),
            2 => Ok(Self::BlueSentinels),
            3 => Ok(Self::BrotherhoodOfBlood),
            4 => Ok(Self::WayOfBlue),
            5 => Ok(Self::RatKing),
            6 => Ok(Self::BellKeepers),
            7 => Ok(Self::DragonRemnants),
            8 => Ok(Self::CompanyOfChampions),
            9 => Ok(Self::PilgrimsOfDark),
            _ => Err(()),
        }
    }
}

impl Display for Covenant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::None => "None",
            Self::HeirsOfTheSun => "Heirs of the Sun",
            Self::BlueSentinels => "Blue Sentinels",
            Self::BrotherhoodOfBlood => "Brotherhood of Blood",
            Self::WayOfBlue => "Way of Blue",
            Self::RatKing => "Rat King",
            Self::BellKeepers => "Bell Keepers",
            Self::DragonRemnants => "Dragon Remnants",
            Self::CompanyOfChampions => "Company of Champions",
            Self::PilgrimsOfDark => "Pilgrims of Dark",
        };
        write!(f, "{}", name)
    }
}

pub const COVENANTS: [Covenant; 10] = [
    Covenant::None,
    Covenant::HeirsOfTheSun,
    Covenant::BlueSentinels,
    Covenant::BrotherhoodOfBlood,
    Covenant::WayOfBlue,
    Covenant::RatKing,
    Covenant::BellKeepers,
    Covenant::DragonRemnants,
    Covenant::CompanyOfChampions,
    Covenant::PilgrimsOfDark,
];