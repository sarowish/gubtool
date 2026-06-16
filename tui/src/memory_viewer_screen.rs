use crate::{
    app::CurrentScreen, common::centered_rect, event::AnyhowExt, help::help_paragraph,
    input::{input_prompt::{InputPrompt, PromptType}}, memory_viewer_screen::InputRequest::*, theme::theme,
};
use anyhow::{Result, anyhow};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use gubtool_core::memory_viewer::{self, MemoryViewer};
use ratatui::{
    Frame,
    layout::{
        Constraint,
        Direction::{Horizontal},
        Layout, Rect,
    },
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

const HELP_ENTRIES: &[(&str, &str)] = &[
    ("g", "Jump to module relative address"),
    ("ctrl-g", "Jump to absolute address"),
    ("enter", "Write byte"),
    ("d", "Write dword"),
    ("q", "Write qword"),
    ("ctrl-d", "Copy dword"),
    ("ctrl-q", "Copy qword"),
    ("y", "Copy module relative address"),
    ("ctrl-y", "Copy absolute address"),
    ("b", "Jump to absolute address at selected"),
    ("ctrl-b", "Jump to relative address at selected"),
    ("u", "Undo jump"),
    ("ctrl-r", "Redo jump"),
    ("i", "Increment selected"),
];

pub struct MemoryViewerScreen {
    memory_viewer: MemoryViewer,
    bytes_per_row: i64,
    frame_heigth: i64,
    show_help: bool,
    input: InputPrompt<InputRequest>,
}

impl MemoryViewerScreen {
    pub fn new() -> Self {
        Self {
            memory_viewer: MemoryViewer::new(),
            bytes_per_row: 0,
            frame_heigth: 0,
            show_help: false,
            input: InputPrompt::new(),
        }
    }
    pub fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        self.memory_viewer.poll();

        frame.render_widget(Clear, rect);

        let block = Block::new().borders(Borders::TOP | Borders::BOTTOM).bg(theme().bg);
        let [address, bytes] = Layout::default()
            .direction(Horizontal)
            .constraints(vec![Constraint::Max(15), Constraint::Fill(1)])
            .areas(block.inner(rect));

        frame.render_widget(block, rect);

        self.update_width_and_heigth(bytes);

        frame.render_widget(self.addresses_paragraph(), address);
        frame.render_widget(self.memory_paragraph(), bytes);

        if self.show_help {
            let layout = centered_rect(60, 75, frame.area());
            frame.render_widget(Clear, layout);
            frame.render_widget(help_paragraph(HELP_ENTRIES, 13), layout);
        }

        self.input.draw_popup_checked(frame);
    }

    pub fn handle_keys(&mut self, key: KeyEvent, current_screen: &mut CurrentScreen) {
        if self.show_help {
            self.show_help = false;
            return;
        }

        if self.input.show {
            self.input.handle_keys(key);
            if key.code == KeyCode::Enter {
                self.input_enter();
            }
            return;
        }

        match (key.code, key.modifiers) {
            (KeyCode::Char('l'), _) => {
                self.memory_viewer.increment_highlighted(1);
                if let Some(current_row) = self.current_row() &&
                    current_row >= self.frame_heigth as u64
                {
                    self.memory_viewer.increment_base(self.bytes_per_row);
                }
            }
            (KeyCode::Char('h'), _) => {
                if self.memory_viewer.highlighted_offset == 0 {
                    self.memory_viewer.increment_base(-self.bytes_per_row);
                }
                self.memory_viewer.increment_highlighted(-1);
            }
            (KeyCode::Char('j'), _) => {
                if self.memory_viewer.highlighted_offset
                    >= (self.bytes_per_row * (self.frame_heigth - 1)) as u64
                {
                    self.memory_viewer.increment_base(self.bytes_per_row)
                }
                self.memory_viewer.increment_highlighted(self.bytes_per_row)
            }
            (KeyCode::Char('k'), _) => {
                if self.memory_viewer.highlighted_offset < self.bytes_per_row as u64 {
                    self.memory_viewer.increment_base(-self.bytes_per_row)
                }
                self.memory_viewer
                    .increment_highlighted(-self.bytes_per_row)
            }
            (KeyCode::F(1), _) => self.show_help = true,
            (KeyCode::Char('y'), KeyModifiers::CONTROL) => {
                self.memory_viewer.copy_absolute_address_at_highlighted();
            }
            (KeyCode::Char('y'), _) => {
                self.memory_viewer.copy_relative_address_at_highlighted();
            }
            (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                self.memory_viewer.copy_qword_at_highlighted();
            }
            (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                self.memory_viewer.copy_dword_at_highlighted();
            }
            (KeyCode::Enter, _) => {
                self.input.show("Write Byte", PromptType::U8, WriteByte);
            }
            (KeyCode::Char('q'), _) => {
                self.input.show("Write Qword", PromptType::U64, WriteQword);
            }
            (KeyCode::Char('d'), _) => {
                self.input.show("Write Dword", PromptType::U32, WriteDword);
            }
            (KeyCode::Char('g'), KeyModifiers::CONTROL) => {
                self.input.show("Jump Absolute", PromptType::U64, JumpAbsolute);
            }
            (KeyCode::Char('g'), _) => {
                self.input.show("Jump Relative", PromptType::I32, JumpRelative);
            }
            (KeyCode::Char('i'), _) => {
                self.input.show("Increment Highlighted", PromptType::I32, IncrementHighlight);
            }
            (KeyCode::Char('u'), _) => {
                self.memory_viewer.jump_backwards();
            }
            (KeyCode::Char('r'), KeyModifiers::CONTROL) => {
                self.memory_viewer.jump_forwards();
            }
            (KeyCode::Char('b'), KeyModifiers::CONTROL) => {
                self.memory_viewer.jump_relative_i32_at_highlighted();
            }
            (KeyCode::Char('b'), _) => {
                self.memory_viewer.jump_absolute_at_highlighted();
            }
            (KeyCode::Esc, _) => *current_screen = CurrentScreen::Main,
            _ => (),
        }
    }

    fn memory_paragraph(&self) -> Paragraph<'static> {
        let mut spans = Vec::new();
        self.memory_viewer.bytes.iter().enumerate().for_each(|(idx, byte)| {
            let address = self.memory_viewer.base_address + idx as u64;
            let is_highlighted = self.memory_viewer.highlighted_offset == idx as u64;
            let text_color = if self.memory_viewer.changed_highlights.contains_key(&(&address)) {
                theme().error
            } else if self.memory_viewer.copied_highlights.contains_key(&(&address)) {
                theme().success
            } else if is_highlighted {
                theme().bg
            } else {
                theme().fg
            };
            let backlight = if is_highlighted {
                theme().fg
            } else {
                theme().bg
            };
            if self.memory_viewer.read_successful {
                spans.push(
                    Span::raw(format!("{:02x}", byte))
                        .bg(backlight)
                        .fg(text_color),
                )
            } else {
                spans.push(Span::raw(format!("??")).bg(backlight).fg(text_color))
            };
            spans.push(Span::raw(" "))
        });
        Paragraph::new(Line::from(spans)).wrap(Wrap { trim: false })
    }

    fn addresses_paragraph(&self) -> Paragraph<'static> {
        let mut lines = Vec::new();
        if let Some(rows) = memory_viewer::READ_SIZE.checked_div(self.bytes_per_row as usize) {
            for i in 0..rows {
                lines.push(Line::from(format!(
                    "{:#X}",
                    self.memory_viewer.base_address + self.bytes_per_row as u64 * i as u64
                )));
            }
        }
        Paragraph::new(lines).block(Block::new().borders(Borders::RIGHT))
    }

    fn update_width_and_heigth(&mut self, rect: Rect) {
        self.bytes_per_row = ((rect.width + 1) / 3) as i64;
        self.frame_heigth = rect.height as i64
    }

    fn input_enter(&mut self) {
        let text = self.input.text();
        let stripped_text = text.strip_prefix("0x").unwrap_or(&text);
        match self.input.last_request.unwrap() {
            WriteByte => {
                u8::from_str_radix(&stripped_text, 16)
                    .map_err(|_| anyhow!("Expected a hex u8"))
                    .and_then(|val| Ok(self.memory_viewer.write_at_highlighted::<u8>(val)?))
                    .send_error();
            }
            WriteDword => {
                u32::from_str_radix(&stripped_text, 16)
                    .map_err(|_| anyhow!("Expected a hex u32"))
                    .and_then(|val| Ok(self.memory_viewer.write_at_highlighted::<u32>(val)?))
                    .send_error();
            }
            WriteQword => {
                u64::from_str_radix(&stripped_text, 16)
                    .map_err(|_| anyhow!("Expected a hex u64"))
                    .and_then(|val| Ok(self.memory_viewer.write_at_highlighted::<u64>(val)?))
                    .send_error();
            }
            JumpAbsolute => {
                u64::from_str_radix(&stripped_text, 16)
                    .map_err(|_| anyhow!("Expected a hex u64"))
                    .map(|val| self.memory_viewer.jump(val))
                    .send_error();
            }
            JumpRelative => {
                u64::from_str_radix(&stripped_text, 16)
                    .map_err(|_| anyhow!("Expected a hex u64"))
                    .map(|val| self.memory_viewer.jump_module_relative(val))
                    .send_error();
            }
            IncrementHighlight => {
                parse_i64(&text)
                    .map(|val| self.memory_viewer.increment_highlighted(val))
                    .send_error();
                if let Some(current_row) = self.current_row() {
                    let diff = current_row.saturating_sub(self.frame_heigth as u64);
                    if diff > 0 {
                        self.memory_viewer.increment_base((diff as i64 + 3) * self.bytes_per_row);
                    }
                }
            }
        }
    }
    fn current_row(&self) -> Option<u64> {
        self.memory_viewer.highlighted_offset
            .checked_div_euclid(self.bytes_per_row as u64)
    }
}

fn parse_i64(text: &str) -> Result<i64> {
    if let Some(hex) = text.strip_prefix("0x") {
        i64::from_str_radix(hex, 16)
    } else {
        text.parse::<i64>()
    }
    .map_err(|_| anyhow!("Expected a decimal i64 or a hex prefixed with 0x"))
}

#[derive(Clone, Copy)]
enum InputRequest {
    WriteByte,
    WriteDword,
    WriteQword,
    JumpAbsolute,
    JumpRelative,
    IncrementHighlight,
}
