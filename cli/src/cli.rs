use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use darksouls2;
use eldenring;
use engine::{Game, attach, game};
use std::{thread, time::Duration};
use tui::tui;

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
        Commands::Quitout => match game() {
            Game::EldenRing => eldenring::utility::quitout(),
            Game::DarkSoulsII => darksouls2::utility::quitout(),
        },
        Commands::KillTarget => match game() {
            Game::EldenRing => {
                if !eldenring::target::is_target_hook_active()? {
                    eldenring::target::install_target_hook()?;
                    thread::sleep(Duration::from_millis(50));
                }
                eldenring::chr_ins::ChrInsExt::set_hp(&eldenring::target::target_ins(), 0)
            }
            Game::DarkSoulsII => {
                if !darksouls2::target::is_target_hook_active()? {
                    darksouls2::target::install_target_hook()?;
                    thread::sleep(Duration::from_millis(50));
                }
                darksouls2::chr_ctrl::ChrCtrlExt::set_hp(&darksouls2::target::target_ctrl(), 0)
            }
        },
        Commands::Test => {
            darksouls2::utility::set_faster_menu(true)
        },
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
