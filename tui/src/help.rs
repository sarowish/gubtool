use crate::{
    common::{block, centered_rect},
    theme::theme,
};
use ratatui::{
    Frame,
    text::{Line, Span, Text},
    widgets::{Clear, Paragraph},
};

const ENTRIES: &[(&str, &str)] = &[
    ("hjkl, ← ↑ ↓ → ", "Navigate list"),
    ("ctrl-hjkl, ← ↑ ↓ → ", "Switch list"),
    ("Enter", "Select"),
    ("s", "Set value"),
    ("f", "Search"),
    ("1-6", "Switch tab"),
    ("tab", "Select next tab"),
    ("backtab", "Select previous tab"),
    ("a", "Attach options"),
    ("p", "Show valid processes"),
    ("o", "Pick game screen"),
    ("g", "Jump to first entry"),
    ("G", "Jump to last entry"),
    ("ctrl-u", "Scroll up"),
    ("ctrl-d", "Scroll down"),
    ("f12", "Change TUI theme"),
    ("ctrl-f12", "Memory Editor"),
    ("f1", "Help"),
];

pub fn draw(frame: &mut Frame) {
    let layout = centered_rect(60, 75, frame.area());
    frame.render_widget(Clear, layout);
    frame.render_widget(help_paragraph(ENTRIES, 22), layout);
}

pub fn help_paragraph(entries: &'static [(&str, &str)], padding: usize) -> Paragraph<'static> {
    let lines: Vec<Line> = entries
        .iter()
        .map(|f| {
            Line::from(vec![
                Span::raw(format!("{:<padding$}",f.0)).style(theme().info),
                Span::raw(f.1).style(theme().fg),
            ])
        })
    .collect();
    Paragraph::new(Text::from(lines))
        .block(block(Some("Help"), None))
}
