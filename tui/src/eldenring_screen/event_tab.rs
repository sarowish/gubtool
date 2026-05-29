use crate::{
    common::{
        StrExt,
        controls::draw_controls,
        event_log_table::{self, handle_log_table_keys, logs_table},
        stateful_list::StatefulList,
        tab_state::TabState,
        tabs_list,
    },
    eldenring_screen::GameState,
    event::ResultExt,
    send_input_event,
    ui_state::UiState,
};
use crossterm::event::{KeyCode, KeyEvent};
use eldenring::event::{self, ErEventLogger, get_dlc_clear, is_event_log_hook, set_event_log_hook};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{List, ListItem, TableState},
};
use shared::event_log::EventLogger;

enum CommandsItems {
    Event,
    FightFortissax,
    FightEldenBeast,
    UnlockMetyr,
    DlcClear,
}

const COMMANDS_IDX: usize = 0;
const LOG_IDX: usize = 1;

pub struct EventTab {
    pub tab: TabState,
    pub event: Option<u32>,
    pub first_encounter: bool,
    pub warp: bool,
    log: ErEventLogger,
    table_state: TableState,
}

impl EventTab {
    pub fn new() -> Self {
        let mut list_states = vec![StatefulList::new(0); 3];
        list_states[COMMANDS_IDX] = StatefulList::new(CommandsItems::ARRAY.len());
        EventTab {
            tab: TabState::new(list_states),
            event: None,
            first_encounter: false,
            warp: true,
            log: ErEventLogger::default(),
            table_state: TableState::default(),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect) {
        let _ = self.log.poll();

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(40),
                Constraint::Percentage(60),
            ])
            .split(layout);

        frame.render_stateful_widget(
            CommandsItems::list(self),
            layout[COMMANDS_IDX],
            &mut self.tab.get_list_state(COMMANDS_IDX),
        );

        frame.render_stateful_widget(
            logs_table(
                &self.log,
                self.tab.block_style(LOG_IDX),
                is_event_log_hook().unwrap_or_default(),
            ),
            layout[LOG_IDX],
            &mut self.table_state,
        );

        draw_controls(frame, layout[LOG_IDX], event_log_table::CONTROLS);
    }

    pub fn handle_keys(&mut self, key: KeyEvent) {
        if self.tab.current_list == LOG_IDX {
            handle_log_table_keys(&mut self.table_state, &mut self.log, key);
        }
        self.tab.handle_keys(key);
        match key.code {
            KeyCode::Char('s') => self.handle_input(),
            KeyCode::Enter => self.handle_select(),
            _ => (),
        }
    }

    fn handle_input(&self) {
        if let Some(selected) = self.tab.get_list_selected(self.tab.current_list) {
            match self.tab.current_list {
                COMMANDS_IDX => CommandsItems::ARRAY[selected].set_input(),
                _ => (),
            }
        }
    }

    fn handle_select(&mut self) {
        let Some(selected) = self.tab.get_list_selected(self.tab.current_list) else { return };
        match self.tab.current_list {
            COMMANDS_IDX => CommandsItems::array()[selected].execute(self),
            LOG_IDX => {
                let new_state = !is_event_log_hook().unwrap_or_default();
                set_event_log_hook(new_state).send_error();
            }
            _ => (),
        }
    }
}

impl CommandsItems {
    fn execute(&self, event_tab: &EventTab) {
        match self {
            Self::Event => {
                if let Some(event) = event_tab.event {
                    let new_state = !event::get_event(event).unwrap_or_default();
                    event::set_event(event, new_state).send_error()
                }
            }
            Self::FightFortissax => {
                event::fight_fortissax().send_error()
            }
            Self::FightEldenBeast => {
                event::fight_elden_beast().send_error()
            }
            Self::UnlockMetyr => {
                event::unlock_metyr().send_error()
            }
            Self::DlcClear => {
                let new_state = !get_dlc_clear().unwrap_or_default();
                event::set_dlc_clear(new_state).send_error()
            }
        }
    }
    fn set_input(&self) {
        match self {
            Self::Event => {
                send_input_event!(text, app, {
                    if let Ok(v) = text.parse() {
                        app.elden_ring.event.event = Some(v);
                        UiState::update_er(|c| { c.event = Some(v); }).ok();
                    } else if text.is_empty() {
                        app.elden_ring.event.event = None;
                        UiState::update_er(|c| { c.event = None; }).ok();
                    }
                })
            },
            _ => (),
        }
    }
    fn to_list_item(&self, event_tab: &EventTab) -> ListItem<'_> {
        let text = match self {
            Self::Event => {
                let state = event::get_event(event_tab.event.unwrap_or_default()).unwrap_or_default();
                format!("Event ({})",
                event_tab.event.map(|v| v.to_string()).unwrap_or_default())
                    .create_toggle_str(state)
            }
            Self::FightFortissax => {
                "Fight Fortissax".to_string()
            }
            Self::FightEldenBeast => {
                "Fight Elden Beast".to_string()
            }
            Self::UnlockMetyr => {
                "Unlock Metyr".to_string()
            }
            Self::DlcClear => {
                let state = get_dlc_clear().unwrap_or_default();
                "DLC Clear Flag".create_toggle_str(state)
            }
        };
        ListItem::new(text)
    }
    const ARRAY: &[CommandsItems] = &[
        Self::Event,
        Self::FightFortissax,
        Self::FightEldenBeast,
        Self::UnlockMetyr,
        Self::DlcClear,
    ];
    const NO_DLC_ARRAY: &[CommandsItems] = &[
        Self::Event,
        Self::FightFortissax,
        Self::FightEldenBeast,
    ];
    fn array() -> &'static [CommandsItems] {
        if !GameState::dlc(){ Self::NO_DLC_ARRAY } else { Self::ARRAY }
    }
    fn list(event_tab: &EventTab) -> List<'static> {
        let array = Self::array();
        let items: Vec<ListItem> = array.iter().map(|i| i.to_list_item(event_tab)).collect();
        tabs_list(items, None, &event_tab.tab, COMMANDS_IDX)
    }
}