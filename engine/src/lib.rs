pub mod attach;
pub mod sys;

use anyhow::Result;
use nix::unistd::Pid;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
};

static mut ATTACHED_PROCESS: GameProcess = GameProcess::detached();

pub struct GameProcess {
    pub pid: Pid,
    pub comm: &'static str,
    pub path: PathBuf,
    pub game: Game,
    pub version: Version,
    pub module_handle: u64,
    pub attach_result: Result<()>,
}

impl GameProcess {
    pub const fn detached() -> Self {
        Self {
            pid: Pid::from_raw(-1),
            comm: "",
            path: PathBuf::new(),
            game: Game::EldenRing,
            version: Version::ERUnknown,
            module_handle: 0,
            attach_result: Ok(()),
        }
    }
}

pub fn detach() {
    unsafe { ATTACHED_PROCESS  = GameProcess::detached() }
}

pub fn pid() -> Pid {
    unsafe { ATTACHED_PROCESS.pid }
}

pub fn game() -> Game {
    unsafe { ATTACHED_PROCESS.game }
}

pub fn module_handle() -> u64 {
    unsafe { ATTACHED_PROCESS.module_handle }
}

pub fn version() -> Version {
    unsafe { ATTACHED_PROCESS.version }
}

#[derive(Clone, Copy)]
pub enum Version {
    ER1_2_0, ER1_2_1, ER1_2_2, ER1_2_3,
    ER1_3_0, ER1_3_1, ER1_3_2, ER1_4_0,
    ER1_4_1, ER1_5_0, ER1_6_0, ER1_7_0,
    ER1_8_0, ER1_8_1, ER1_9_0, ER1_9_1,
    ER2_0_0, ER2_0_1, ER2_2_0, ER2_2_3,
    ER2_3_0, ER2_4_0, ER2_5_0, ER2_6_0,
    ER2_6_1, ERUnknown,

    Vanilla1_0_3, Vanilla1_0_4, Vanilla1_0_5, Vanilla1_0_6,
    Vanilla1_0_7, Vanilla1_0_10, Vanilla1_0_11, Vanilla1_0_12,
    Scholar1_0_1, Scholar1_0_2, Scholar1_0_3, VanillaUnknown,
    ScholarUnknown,
}

#[derive(PartialEq)]
#[derive(Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub enum Game {
    EldenRing,
    DarkSoulsII,
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let name = match self {
            Game::EldenRing => "Elden Ring",
            Game::DarkSoulsII => "Dark Souls II",
        };
        write!(f, "{}", name)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self {
            Version::ER1_2_0 => "Elden Ring v1.02",
            Version::ER1_2_1 => "Elden Ring v1.02.1",
            Version::ER1_2_2 => "Elden Ring v1.02.2",
            Version::ER1_2_3 => "Elden Ring v1.02.3",
            Version::ER1_3_0 => "Elden Ring v1.03",
            Version::ER1_3_1 => "Elden Ring v1.03.1",
            Version::ER1_3_2 => "Elden Ring v1.03.2",
            Version::ER1_4_0 => "Elden Ring v1.04",
            Version::ER1_4_1 => "Elden Ring v1.04.1",
            Version::ER1_5_0 => "Elden Ring v1.05",
            Version::ER1_6_0 => "Elden Ring v1.06",
            Version::ER1_7_0 => "Elden Ring v1.07",
            Version::ER1_8_0 => "Elden Ring v1.08",
            Version::ER1_8_1 => "Elden Ring v1.08.1",
            Version::ER1_9_0 => "Elden Ring v1.09",
            Version::ER1_9_1 => "Elden Ring v1.09.1",
            Version::ER2_0_0 => "Elden Ring v1.10",
            Version::ER2_0_1 => "Elden Ring v1.10.1",
            Version::ER2_2_0 => "Elden Ring v1.12",
            Version::ER2_2_3 => "Elden Ring v1.12.3",
            Version::ER2_3_0 => "Elden Ring v1.13",
            Version::ER2_4_0 => "Elden Ring v1.14",
            Version::ER2_5_0 => "Elden Ring v1.15",
            Version::ER2_6_0 => "Elden Ring v1.16",
            Version::ER2_6_1 => "Elden Ring v1.16.1",
            Version::ERUnknown => "Unknown",

            Version::Vanilla1_0_3 => "Dark Souls II v1.0.3",
            Version::Vanilla1_0_4 => "Dark Souls II v1.0.4",
            Version::Vanilla1_0_5 => "Dark Souls II v1.0.5",
            Version::Vanilla1_0_6 => "Dark Souls II v1.0.6",
            Version::Vanilla1_0_7 => "Dark Souls II v1.0.7",
            Version::Vanilla1_0_10 => "Dark Souls II v1.0.10",
            Version::Vanilla1_0_11 => "Dark Souls II v1.0.11",
            Version::Vanilla1_0_12 => "Dark Souls II v1.0.12",
            Version::VanillaUnknown => "Unknown",
            Version::Scholar1_0_1 => "Dark Souls II (Scholar) v1.0.1",
            Version::Scholar1_0_2 => "Dark Souls II (Scholar) v1.0.2",
            Version::Scholar1_0_3 => "Dark Souls II (Scholar) v1.0.3",
            Version::ScholarUnknown => "Unknown",
        };
        write!(f, "{}", name)
    }
}