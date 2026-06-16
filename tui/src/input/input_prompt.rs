use std::str::FromStr;
use crate::input::Input;
use ratatui::{Frame, layout::{Constraint, Direction, Layout}, text::Line, widgets::{Clear, Paragraph}};
use shared::act_array::ActArray;
use crate::{common::block, event::{Event, send_event}, theme::theme};
use crossterm::event::{KeyCode, KeyEvent};

pub struct InputPrompt<C> {
    input: Input,
    prompt: &'static str,
    prompt_type: Option<PromptType>,
    pub last_request: Option<C>,
    pub show: bool,
}

impl<C> InputPrompt<C> {
    pub fn new() -> Self {
        Self {
            input: Input::new(),
            prompt: "",
            prompt_type: None,
            last_request: None,
            show: false,
        }
    }

    pub fn show(&mut self, prompt: &'static str, prompt_type: PromptType, request: C) {
        send_event(Event::BlockInputs(true));
        self.last_request = Some(request);
        self.prompt = prompt;
        self.prompt_type = Some(prompt_type);
        self.input.clear_line();
        self.show = true;
    }

    fn hide(&mut self) {
        send_event(Event::BlockInputs(false));
        self.prompt_type = None;
        self.show = false;
    }

    pub fn text(&self) -> String {
        self.input.text.clone()
    }

    pub fn parse_text<T: FromStr>(&self) -> Option<T> {
        if let Ok(v) = self.input.text.parse::<T>() {
            Some(v)
        } else {
            None
        }
    }

    pub fn handle_keys(&mut self, key: KeyEvent) {
        self.input.handle_keys(key);
        match key.code {
            KeyCode::Esc | KeyCode::Enter => self.hide(),
            _ => (),
        }
    }

    pub fn draw_popup_checked(&mut self, frame: &mut Frame) {
        if !self.show {
            return;
        }
        let vert = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(3),
                Constraint::Fill(1),
            ])
            .split(frame.area());

        let rect = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(frame.area().width / 3),
                Constraint::Fill(1),
            ])
            .split(vert[1])[1];


        let block_theme = if self.can_input_be_parsed_from_type() {
            theme().success
        } else {
            theme().error
        };

        let block = block(None, None).style(block_theme)
            .title(Line::from(self.prompt).style(block_theme));
        let inner = block.inner(rect);

        self.input.update_width(inner.width);
        let input = Paragraph::new(self.input.to_string())
            .style(theme().fg);

        self.input.set_cursor(frame, inner);

        frame.render_widget(Clear, rect);
        frame.render_widget(block, rect);
        frame.render_widget(input, inner);
    }

    fn can_input_be_parsed_from_type(&self) -> bool {
        let text = &self.input.text;
        match self.prompt_type.unwrap() {
            PromptType::U8 => text.parse::<u8>().is_ok(),
            PromptType::I32 => text.parse::<i32>().is_ok(),
            PromptType::I64 => text.parse::<i64>().is_ok(),
            PromptType::U32 => text.parse::<u32>().is_ok(),
            PromptType::U64 => text.parse::<u64>().is_ok(),
            PromptType::F32 => text.parse::<f32>().is_ok(),
            PromptType::ActArray => text.parse::<ActArray>().is_ok(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum PromptType {
    U8, I32, I64, U32, U64, F32, ActArray,
}