use crate::{
    core::attach::{self, Game, game},
    ds2::{self, chr_ctrl::ChrCtrlExt},
    er::{self, chr_ins::ChrInsExt},
    tui::tui,
};

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use std::{thread, time::Duration};

#[derive(Parser)]
#[command(name = "gubtool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Quitout,
    KillTarget,
    Test,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    if cli.command.is_none() {
        tui().ok();
        return Ok(());
    }

    attach::auto_attach()?;

    match cli.command.unwrap() {
        Commands::Quitout => {
            match game() {
                Game::EldenRing => er::utility::quitout(),
                Game::DarkSoulsII => ds2::utility::quitout(),
            }
        }
        Commands::KillTarget => {
            match game() {
                Game::EldenRing => {
                    if !er::target::is_target_hook_active()? {
                        er::target::install_target_hook()?;
                        thread::sleep(Duration::from_millis(50));
                    }
                    ChrInsExt::set_hp(&er::target::target_ins(), 0)
                }
                Game::DarkSoulsII => {
                    if !ds2::target::is_target_hook_active()? {
                        ds2::target::install_target_hook()?;
                        thread::sleep(Duration::from_millis(50));
                    }
                    ChrCtrlExt::set_hp(&ds2::target::target_ctrl(), 0)
                }
            }
        }
        Commands::Test => {
            Ok(())
        }
    }
}

#[derive(Clone, ValueEnum)]
pub enum OnOff {
    On = 1,
    Off = 0,
}

impl From<OnOff> for bool {
    fn from(val: OnOff) -> Self {
        val as u8 != 0
    }
}
