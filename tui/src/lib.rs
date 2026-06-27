mod app;
mod attach_options;
mod common;
mod darksouls2_screen;
mod eldenring_screen;
mod event;
mod game_screen_selector;
mod help;
mod input;
mod process_selector;
mod macros;
mod memory_viewer_screen;
mod traits;
mod theme;
mod ui_state;

pub fn run() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    crate::app::App::new().run(terminal)?;
    ratatui::restore();
    Ok(())
}