use crate::common::tab_state::TabState;
use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};

pub trait GameScreen {
    fn draw(&mut self, frame: &mut Frame, layout: Rect);
    async fn handle_keys(&mut self, key: KeyEvent, block_inputs: bool);
    fn render_tick(&mut self);
    fn background_tick(&mut self);
    fn dbg_lines() -> Vec<String>;
}

pub trait TabScreen {
    fn tab_state(&self) -> TabState;
}