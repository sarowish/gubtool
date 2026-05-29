use crate::{
    common::{
        StrExt,
        controls::draw_controls,
        event_log_table::{self, handle_log_table_keys, logs_table},
        stateful_list::StatefulList,
        tab_state::TabState,
        tabs_list,
    },
    event::ResultExt,
    send_input_event,
    ui_state::UiState,
};
use crossterm::event::{KeyCode, KeyEvent};
use darksouls2::{
    event::{self, Ds2EventLogger, is_event_log_hook, set_event_log_hook},
    resources::{areas::MapId, event_flags::EventFlag},
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{List, ListItem, TableState},
};
use shared::event_log::EventLogger;

enum CommandsItems {
    Event,
    VisibleAava,
    UnlockNashandra,
    UnlockAldia,
    KingsRingAcquired,
    ShadedWoodsChasmCleared,
    DrangleicCastleChasmCleared,
    BlackGulchChasmCleared,
    UndoAlsanaSeal,
    SkipIvoryGauntlet,
    DisableLoyceKnights,
    LoyceKnightOuterWall,
    LoyceKnightAbandonedDwelling,
    LoyceKnightLowerGarrison,
    ActivateBrume,
}

const COMMANDS_IDX: usize = 0;
const LOG_IDX: usize = 1;


pub struct EventTab {
    tab: TabState,
    event: Option<u32>,
    log: Ds2EventLogger,
    table_state: TableState,
}

impl EventTab {
    pub fn new() -> Self {
        let mut list_states = vec![StatefulList::new(0); 2];
        list_states[COMMANDS_IDX] = StatefulList::new(CommandsItems::ARRAY.len());

        EventTab {
            tab: TabState::new(list_states),
            event: None,
            log: Ds2EventLogger::default(),
            table_state: TableState::default(),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect) {
        let _ = self.log.poll();

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
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

    fn handle_select(&self) {
        if let Some(selected) = self.tab.get_list_selected(self.tab.current_list) {
            match self.tab.current_list {
                COMMANDS_IDX => CommandsItems::ARRAY[selected].execute(&self),
                LOG_IDX => {
                    let new_state = !is_event_log_hook().unwrap_or_default();
                    set_event_log_hook(new_state).send_error();
                }
                _ => (),
            }
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
}

impl CommandsItems {
    fn execute(&self, event_tab: &EventTab) {
        match self {
            Self::Event => {
                if let Some(event) = event_tab.event {
                    event::get_event_flag(event)
                        .and_then(|state| event::set_event_flag(event, !state))
                        .send_error()
                }
            }
            Self::VisibleAava => {
                let new_state = !EventFlag::VisibleAava.get().unwrap_or_default();
                EventFlag::VisibleAava
                    .set_area_conditional_event(new_state, MapId::FrozenEleumLoyce)
                    .send_error();
            }
            Self::KingsRingAcquired => {
                let new_state = !EventFlag::KingsRingAcquired.get().unwrap_or_default();
                EventFlag::KingsRingAcquired.set(new_state).send_error();
            }
            Self::UnlockNashandra => {
                let new_state = !EventFlag::GiantLordDefeated.get().unwrap_or_default();
                EventFlag::GiantLordDefeated.set(new_state).send_error();
            }
            Self::UnlockAldia => {
                let new_state = !EventFlag::get_flags(&[
                    EventFlag::VendrickDefeated,
                    EventFlag::UnlockAldia
                    ]).unwrap_or_default();
                EventFlag::set_flags(&[
                    (EventFlag::VendrickDefeated, new_state),
                    (EventFlag::UnlockAldia, new_state),
                ]).send_error();
            }
            Self::BlackGulchChasmCleared => {
                let new_state = !EventFlag::BlackGulchChasmCleared.get().unwrap_or_default();
                EventFlag::BlackGulchChasmCleared
                    .set_area_conditional_event(new_state, MapId::DarkChasmOfOld)
                    .send_error();
            }
            Self::DrangleicCastleChasmCleared => {
                let new_state = !EventFlag::DrangleicCastleChasmCleared.get().unwrap_or_default();
                EventFlag::DrangleicCastleChasmCleared
                    .set_area_conditional_event(new_state, MapId::DarkChasmOfOld)
                    .send_error();
            }
            Self::ShadedWoodsChasmCleared => {
                let new_state = !EventFlag::ShadedWoodsChasmCleared.get().unwrap_or_default();
                EventFlag::ShadedWoodsChasmCleared
                    .set_area_conditional_event(new_state, MapId::DarkChasmOfOld)
                    .send_error();
            }
            Self::LoyceKnightAbandonedDwelling => {
                let new_state = !EventFlag::LoyceKnightAbandonedDwelling.get().unwrap_or_default();
                EventFlag::LoyceKnightAbandonedDwelling
                    .set_area_conditional_event(new_state, MapId::FrozenEleumLoyce)
                    .send_error();
            }
            Self::LoyceKnightLowerGarrison => {
                let new_state = !EventFlag::LoyceKnightLowerGarrison.get().unwrap_or_default();
                EventFlag::LoyceKnightLowerGarrison
                    .set_area_conditional_event(new_state, MapId::FrozenEleumLoyce)
                    .send_error();
            }
            Self::LoyceKnightOuterWall => {
                let new_state = !EventFlag::LoyceKnightOuterWall.get().unwrap_or_default();
                EventFlag::LoyceKnightOuterWall
                    .set_area_conditional_event(new_state, MapId::FrozenEleumLoyce)
                    .send_error();
            }
            Self::UndoAlsanaSeal => {
                let new_state = !EventFlag::EleumLoyceIce.get().unwrap_or_default();
                EventFlag::EleumLoyceWinds
                    .set_area_conditional_event(new_state, MapId::FrozenEleumLoyce)
                    .send_error();
                EventFlag::EleumLoyceIce
                    .set_area_conditional_event(new_state, MapId::FrozenEleumLoyce)
                    .send_error();
            }
            Self::SkipIvoryGauntlet => {
                let new_state = !event::is_ivory_gauntlet_skip().unwrap_or_default();
                event::set_ivory_gauntlet_skip(new_state).send_error();
            }
            Self::DisableLoyceKnights => {
                let new_state = !event::is_ivory_no_knights().unwrap_or_default();
                event::set_ivory_no_knights(new_state).send_error();
            }
            Self::ActivateBrume => {
                let new_state = !EventFlag::ActivateBrume.get().unwrap_or_default();
                EventFlag::ActivateBrume
                    .set_area_conditional_event(new_state, MapId::BrumeTower)
                    .send_error();
            }
        }
    }
    fn set_input(&self) {
        match self {
            Self::Event => {
                send_input_event!(text, app, {
                    if let Ok(v) = text.parse() {
                        app.dark_souls_2.event.event = Some(v);
                        UiState::update_ds2(|c| {
                            c.event = Some(v);
                        })
                        .ok();
                    } else if text.is_empty() {
                        app.dark_souls_2.event.event = None;
                        UiState::update_ds2(|c| {
                            c.event = None;
                        })
                        .ok();
                    }
                })
            }
            _ => (),
        }
    }
    fn to_list_item(&self, event_tab: &EventTab) -> ListItem<'_> {
        let text = match self {
            Self::Event => {
                let state =
                    event::get_event_flag(event_tab.event.unwrap_or_default()).unwrap_or_default();
                format!(
                    "Event Flag ({})",
                    event_tab.event.map(|v| v.to_string()).unwrap_or_default()
                )
                .create_toggle_str(state)
            }
            Self::VisibleAava => {
                let state = EventFlag::VisibleAava.get().unwrap_or_default();
                "Visible Aava".create_toggle_str(state)
            }
            Self::KingsRingAcquired => {
                let state = EventFlag::KingsRingAcquired.get().unwrap_or_default();
                "King's Ring Acquired".create_toggle_str(state)
            }
            Self::UnlockNashandra => {
                let state = EventFlag::GiantLordDefeated.get().unwrap_or_default();
                "Unlock Nashandra".create_toggle_str(state)
            }
            Self::UnlockAldia => {
                let state = EventFlag::get_flags(&[
                    EventFlag::VendrickDefeated,
                    EventFlag::UnlockAldia
                    ]).unwrap_or_default();

                "Unlock Aldia".create_toggle_str(state)
            }
            Self::ShadedWoodsChasmCleared => {
                let state = EventFlag::ShadedWoodsChasmCleared.get().unwrap_or_default();
                "Dark Chasm Lit (Shaded Woods)".create_toggle_str(state)
            }
            Self::DrangleicCastleChasmCleared => {
                let state = EventFlag::DrangleicCastleChasmCleared.get().unwrap_or_default();
                "Dark Chasm Lit (Drangleic Castle)".create_toggle_str(state)
            }
            Self::BlackGulchChasmCleared => {
                let state = EventFlag::BlackGulchChasmCleared.get().unwrap_or_default();
                "Dark Chasm Lit (Black Gulch)".create_toggle_str(state)
            }
            Self::LoyceKnightAbandonedDwelling => {
                let state = EventFlag::LoyceKnightAbandonedDwelling.get().unwrap_or_default();
                "Free Loyce Knight (Abandoned Dwelling)".create_toggle_str(state)
            }
            Self::LoyceKnightLowerGarrison => {
                let state = EventFlag::LoyceKnightLowerGarrison.get().unwrap_or_default();
                "Free Loyce Knight (Lower Garrison)".create_toggle_str(state)
            }
            Self::LoyceKnightOuterWall => {
                let state = EventFlag::LoyceKnightOuterWall.get().unwrap_or_default();
                "Free Loyce Knight (Outer Wall)".create_toggle_str(state)
            }
            Self::UndoAlsanaSeal => {
                let state = EventFlag::EleumLoyceIce.get().unwrap_or_default();
                "Undo Alsana's Seal".create_toggle_str(state)
            }
            Self::SkipIvoryGauntlet => {
                let state = event::is_ivory_gauntlet_skip().unwrap_or_default();
                "Skip Ivory King Gauntlet".create_toggle_str(state)
            }
            Self::DisableLoyceKnights => {
                let state = event::is_ivory_no_knights().unwrap_or_default();
                "Disable Loyce Knights".create_toggle_str(state)
            }
            Self::ActivateBrume => {
                let state = EventFlag::ActivateBrume.get().unwrap_or_default();
                "Activate Brume Tower".create_toggle_str(state)
            }
        };
        ListItem::new(text)
    }
    const ARRAY: &[CommandsItems] = &[
        Self::Event,
        Self::KingsRingAcquired,
        Self::UnlockNashandra,
        Self::UnlockAldia,
        Self::ShadedWoodsChasmCleared,
        Self::DrangleicCastleChasmCleared,
        Self::BlackGulchChasmCleared,
        Self::ActivateBrume,
        Self::VisibleAava,
        Self::UndoAlsanaSeal,
        Self::SkipIvoryGauntlet,
        Self::DisableLoyceKnights,
        Self::LoyceKnightOuterWall,
        Self::LoyceKnightAbandonedDwelling,
        Self::LoyceKnightLowerGarrison,
    ];
    fn list(event_tab: &EventTab) -> List<'static> {
        let array = Self::ARRAY;
        let items: Vec<ListItem> = array.iter().map(|i| i.to_list_item(event_tab)).collect();
        tabs_list(items, None, &event_tab.tab, COMMANDS_IDX)
    }
}
