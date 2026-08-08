use crate::{
    darksouls2_screen, eldenring_screen,
    event::KeyContext,
    popup::{Popup, PopupState},
    screen::Screen,
};
use gubtool_core::{attached, game_version::Game};
use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Text},
    widgets::{Paragraph, Wrap},
};

#[derive(Default)]
pub struct DebugPopup {
    popup_state: PopupState,
}

impl Popup for DebugPopup {
    fn popup_state(&mut self) -> &mut PopupState {
        &mut self.popup_state
    }
    fn screen(&mut self) -> &mut dyn Screen {
        self
    }
    fn popup_rect(&self, frame: &mut Frame) -> Rect {
        frame.area()
    }
    fn close_on_key(&self, ctx: &mut KeyContext) -> bool {
        ctx.key_any()
    }
}

impl Screen for DebugPopup {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let mut debug_info = vec![
            format!("comm: {}", attached::comm().unwrap_or("")),
            format!("exe_path: {:#?}", attached::path()),
            format!("module_base: {:#X}", attached::module_base()),
            format!("is 32 bit: {}", attached::is_32()),
            format!("process uptime: {:.1}", attached::uptime()),
            format!("\n"),
        ];

        match attached::game() {
            Some(Game::DarkSouls2) => debug_info.append(&mut darksouls2_screen::dbg_lines()),
            Some(Game::EldenRing) => debug_info.append(&mut eldenring_screen::dbg_lines()),
            None => (),
        }
        let lines: Vec<Line> = debug_info
            .iter()
            .map(|f| Line::raw(f.to_string()))
            .collect();
        let paragraph = Paragraph::new(Text::from(lines)).wrap(Wrap { trim: false });
        frame.render_widget(paragraph, rect);
    }
}