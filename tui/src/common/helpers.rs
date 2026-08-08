use crate::theme::{self, theme};
use ratatui::{
    style::{Modifier, Stylize},
    widgets::Block,
};
use ratatui_themes::Style;

pub fn create_toggle_string(str: &str, state: bool) -> String {
    let toggle = match state {
        true => "[X]",
        false => "[ ]",
    };
    format!("{toggle} {str}")
}

pub fn bordered_block<'a>(title: Option<&'a str>) -> Block<'a> {
    let block = Block::bordered()
        .fg(theme().fg)
        .bg(theme().bg)
        .border_type(theme::BORDER_TYPE);

    match title {
        Some(title) => block.title(title.fg(theme().secondary)),
        None => block,
    }
}

#[macro_export]
macro_rules! spawn_task {
    ($($body:tt)*) => {
        tokio::spawn(async move {
            $($body)*
        });
    };
}

pub fn item_options_style(show: bool) -> Style {
    if show { Style::default() } else { Style::new().add_modifier(Modifier::CROSSED_OUT) }
}