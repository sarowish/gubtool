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
mod memory_viewer_screen;
mod theme;
mod ui_state;

use crate::app::App;

pub fn tui() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let _app_result = App::new().run(terminal);
    ratatui::restore();
    Ok(())
}