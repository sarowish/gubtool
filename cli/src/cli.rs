use anyhow::{Ok, ensure};
use clap::{Parser, Subcommand, ValueEnum};
use eldenring::{self, chr_ins::ChrInsExt};
use gubtool_core::{
    attached::{self, game, process_manager::ProcessManager},
    game_version::Game,
};
use std::{thread, time::Duration};
use tui;

#[derive(Parser)]
#[command(name = "gubtool")]
#[derive(Clone, Copy)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
#[derive(Clone, Copy)]
pub enum Commands {
    Quitout,
    KillTarget,
    NextPhase,

    #[cfg(debug_assertions)]
    AobScan,
    #[cfg(debug_assertions)]
    AsmSizes,
    #[cfg(debug_assertions)]
    Test,
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.command.is_none() {
        if let Err(e) = tui::run() {
            eprintln!("{e:?}");
        }
        return Ok(());
    }

    #[cfg(debug_assertions)]
    match cli.command.unwrap() {
        Commands::AsmSizes => {
            gubtool_core::sys::print_asm_sizes();
            darksouls2::utils::print_asm_sizes();
            eldenring::utils::print_asm_sizes();
        },
        Commands::Test => {
        },
        _ => (),
    }

    let _proc_manager = ProcessManager::new().try_auto_attach();

    ensure!(attached::is_attached(), "Game not found");
    let game = game().unwrap();

    match cli.command.unwrap() {
        Commands::Quitout => match game {
            Game::EldenRing => eldenring::utility::quitout()?,
            Game::DarkSouls2 => darksouls2::utility::quitout()?,
        },
        Commands::KillTarget => match game {
            Game::EldenRing => {
                if !eldenring::target::is_target_hook_active()? {
                    eldenring::target::install_target_hook()?;
                    thread::sleep(Duration::from_millis(50));
                }
                eldenring::chr_ins::ChrInsExt::set_hp(&eldenring::target::target_ins(), 0)?
            }
            Game::DarkSouls2 => {
                if !darksouls2::target::is_target_hook_active() {
                    darksouls2::target::install_target_hook()?;
                    thread::sleep(Duration::from_millis(50));
                }
                darksouls2::chr_ctrl::ChrCtrlExt::set_hp(&darksouls2::target::target_ctrl(), 0)?
            }
        },
        Commands::NextPhase => match game {
            Game::EldenRing => eldenring::target::target_ins().next_phase()?,
            Game::DarkSouls2 => (),
        },
        #[cfg(debug_assertions)]
        Commands::AobScan => match game {
            Game::EldenRing => eldenring::utils::scan_and_print_base_offsets()?,
            Game::DarkSouls2 => darksouls2::utils::scan_and_print_base_offsets()?,
        },
        _ => (),
    }
    Ok(())
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
