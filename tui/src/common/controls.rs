use crate::theme::theme;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
};

pub fn draw_controls(frame: &mut Frame, layout: Rect, controls: &[(&str, &str)]) {
    let controls = controls_line(controls);
    let [_, controls_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(layout);
    frame.render_widget(controls, controls_area);
}

fn controls_line(controls: &[(&str, &str)]) -> Line<'static> {
    let mut spans = controls
        .iter()
        .flat_map(|(key, action)| {
            vec![
                Span::raw("[").style(theme().fg),
                Span::raw(key.to_string()).style(theme().info),
                Span::raw("→ ").style(theme().fg),
                Span::raw(action.to_string()).style(theme().fg),
                Span::raw("] ").style(theme().fg),
            ]
        })
        .collect::<Vec<_>>();
    spans.pop();
    spans.push(Span::raw("]").style(theme().fg));
    Line::from(spans)
        .alignment(Alignment::Center)
}