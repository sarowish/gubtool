use anyhow::{Ok, Result, ensure};
use clap::{Parser, Subcommand, ValueEnum};
use darksouls2;
use eldenring::{self, chr_ins::ChrInsExt};
use engine::{attached::{self, game}, game_version::Game};
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
    NextPhase,
    AobScan,
    Test,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    if cli.command.is_none() {
        tui().ok();
        return Ok(());
    }

    ensure!(attached::auto_attach().is_some(), "Game not found");

    let game = game().unwrap();

    match cli.command.unwrap() {
        Commands::Quitout => match game {
            Game::EldenRing => eldenring::utility::quitout(),
            Game::DarkSouls2 => darksouls2::utility::quitout(),
        },
        Commands::KillTarget => match game {
            Game::EldenRing => {
                if !eldenring::target::is_target_hook_active()? {
                    eldenring::target::install_target_hook()?;
                    thread::sleep(Duration::from_millis(50));
                }
                eldenring::chr_ins::ChrInsExt::set_hp(&eldenring::target::target_ins(), 0)
            }
            Game::DarkSouls2 => {
                if !darksouls2::target::is_target_hook_active()? {
                    darksouls2::target::install_target_hook()?;
                    thread::sleep(Duration::from_millis(50));
                }
                darksouls2::chr_ctrl::ChrCtrlExt::set_hp(&darksouls2::target::target_ctrl(), 0)
            }
        },
        Commands::NextPhase => match game {
            Game::EldenRing => eldenring::target::target_ins().next_phase(),
            Game::DarkSouls2 => Ok(()),
        },
        Commands::AobScan => match game {
            Game::EldenRing => eldenring::utils::scan_and_print_base_offsets(),
            Game::DarkSouls2 => darksouls2::utils::scan_and_print_base_offsets(),
        },
        Commands::Test => {
            eldenring::utils::print_asm_sizes();
            Ok(())
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
