use crate::{
    common::block,
    event::{ResultExt, send_success},
    theme::theme,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::Constraint,
    style::Stylize,
    text::{Line, Span},
    widgets::{Cell, Row, Table, TableState},
};
use ratatui_themes::Style;
use shared::event_log::EventLogger;

pub const CONTROLS: &[(&str, &str)] = &[
    ("Enter", "Toggle"),
    ("c", "Clear"),
    ("x", "Export"),
];

pub fn logs_table(logger: &impl EventLogger, style: Style, enabled: bool) -> Table<'static> {
    let rows = logger.entries().iter().enumerate().rev()
        .map(|(idx, record)| {
            let state = match record.state {
                true => Span::raw("TRUE").style(theme().success),
                false => Span::raw("FALSE").style(theme().error),
            };
            Row::new(vec![
                Cell::from((idx + 1).to_string()),
                Cell::from(record.event_id.to_string()),
                Cell::from(state),
                Cell::from(record.time_stamp.format("%H:%M:%S").to_string()),
            ])
        })
        .collect::<Vec<Row>>();
    let header = Row::new(vec![
        Cell::from("Index"),
        Cell::from("Flag ID"),
        Cell::from("State"),
        Cell::from("Time Stamp"),
    ])
    .bold();
    let widths = [
        Constraint::Max(7),
        Constraint::Min(12),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ];
    Table::new(rows, widths).header(header).block(
        block(Some("Event Logs"), Some(style))
            .title(logging_enabled_line(enabled).right_aligned()),
    )
}

fn logging_enabled_line(enabled: bool) -> Line<'static> {
    match enabled {
        true => Line::from("Enabled").style(theme().success),
        false => Line::from("Disabled").style(theme().error),
    }
}

pub fn handle_log_table_keys(table_state: &mut TableState, log: &mut impl EventLogger, key: KeyEvent) {
    match (key.code, key.modifiers) {
        (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
            *table_state.offset_mut() = table_state.offset().saturating_sub(28);
        }
        (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
            *table_state.offset_mut() = table_state.offset().saturating_add(28);
        }
        (KeyCode::Char('k') | KeyCode::Up, _) => {
            *table_state.offset_mut() = table_state.offset().saturating_sub(1);
        }
        (KeyCode::Char('j') | KeyCode::Down, _) => {
            *table_state.offset_mut() = table_state.offset().saturating_add(1);
        }
        (KeyCode::Char('g'), _) => {
            *table_state.offset_mut() = 0;
        }
        (KeyCode::Char('G'), _) => {
            *table_state.offset_mut() = log.entries().len();
        }
        (KeyCode::Char('c'), _) => log.clear().send_error(),
        (KeyCode::Char('v'), _) => log.toggle_show_duplicates(),
        (KeyCode::Char('x'), _) => {
            log.export()
                .map(|path| send_success(format!("Exported to {}", path)))
                .send_error();
        }
        _ => (),
    }
}