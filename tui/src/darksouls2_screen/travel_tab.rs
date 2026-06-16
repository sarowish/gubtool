use crate::{
    common::{
        block, blockless_list, controls::draw_controls, label_list, stateful_list::StatefulList,
        tab_state::TabState,
    }, darksouls2_screen::GameState, event::{AnyhowExt, ResultExt}, input::fuzzy_finder::FuzzyFinder, theme::theme
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use darksouls2::{bonfire, resources::{bonfires, bosses}};
use nucleo_matcher::Utf32String;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{List, ListItem},
};
use ratatui_themes::Style;
use std::thread;

const BOSSES_IDX: usize = 0;
const BONFIRES_IDX: usize = 1;

const BONFIRE_CONTROLS: &[(&str, &str)] = &[
    ("r", "rest"),
    ("t", "Light"),
    ("ctrl-t", "Light All"),
];

pub struct TravelTab {
    tab: TabState,
    fuzzy_finder: FuzzyFinder,
}

impl TravelTab {
    pub fn new() -> Self {
        let mut list_states = vec![StatefulList::new(0); 2];
        list_states[BOSSES_IDX] = StatefulList::new(bosses::BOSSES.len());
        list_states[BONFIRES_IDX] = StatefulList::new(bonfires::BONFIRES.len());
        TravelTab {
            tab: TabState::new(list_states),
            fuzzy_finder: FuzzyFinder::default(),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(40),
                Constraint::Percentage(60),
            ])
            .split(layout);

        frame.render_stateful_widget(
            self.bosses_list(),
            layout[BOSSES_IDX],
            &mut self.tab.get_list_state(BOSSES_IDX),
        );

        let bonfires_block = block(Some("Bonfires"), Some(self.tab.block_style(BONFIRES_IDX)))
            .title(self.bonfire_lit_status_line().right_aligned());
        let bonfires_inner = bonfires_block.inner(layout[BONFIRES_IDX]);
        frame.render_widget(&bonfires_block, layout[BONFIRES_IDX]);
        draw_controls(frame, layout[BONFIRES_IDX], BONFIRE_CONTROLS);

        let [bonfire_name, bonfire_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(30),
                Constraint::Max(26),
            ])
            .areas(bonfires_inner);

        let (bonfire_names, bonfire_areas) = self.bonfires_list();
        frame.render_stateful_widget(
            bonfire_names,
            bonfire_name,
            &mut self.tab.get_list_state(BONFIRES_IDX),
        );
        frame.render_stateful_widget(
            bonfire_areas,
            bonfire_area,
            &mut self.tab.get_list_state(BONFIRES_IDX),
        );

        self.fuzzy_finder.draw_checked(frame);
    }

    pub fn handle_keys(&mut self, key: KeyEvent) {
        if self.fuzzy_finder.show {
            self.fuzzy_finder.handle_keys(key);
            if key.code == KeyCode::Enter {
                if let Some(selected) = self.fuzzy_finder.selected_idx() {
                    self.tab.set_list_selected(self.tab.current_list, selected);
                }
            }
            return;
        }

        self.tab.handle_keys(key);

        match key.code {
            KeyCode::Enter => {
                self.handle_select()
            }
            KeyCode::Char('f') => {
                let list = if self.tab.current_list == BOSSES_IDX {
                    bosses::BOSSES.iter()
                        .map(|boss| Utf32String::from(format!("{}", boss.name)))
                        .collect::<Vec<Utf32String>>()
                } else {
                    bonfires::BONFIRES.iter()
                        .map(|bonfire| Utf32String::from(format!("{}|{}", bonfire.name, bonfire.main_area)))
                        .collect::<Vec<Utf32String>>()
                };
                self.fuzzy_finder.show(list);
            }
            KeyCode::Char('t') => {
                if self.tab.current_list == BONFIRES_IDX {
                    if key.modifiers == KeyModifiers::CONTROL {
                        bonfire::light_all_bonfires().send_error();
                    } else if let Some(selected) = self.tab.get_list_selected(self.tab.current_list) {
                        bonfires::BONFIRES[selected].unlock().send_error();
                    }
                }
            }
            KeyCode::Char('r') => {
                if self.tab.current_list == BONFIRES_IDX &&
                let Some(selected) = self.tab.get_list_selected(self.tab.current_list) {
                    bonfires::BONFIRES[selected].rest().send_error();
                }
            }
            _ => ()
        }
    }

    fn handle_select(&self) {
        let Some(selected) = self.tab.get_list_selected(self.tab.current_list) else { return };
        if self.tab.current_list == BOSSES_IDX {
            thread::spawn(move || {
                bosses::BOSSES[selected].warp().send_error()
            });
        } else if self.tab.current_list == BONFIRES_IDX {
            thread::spawn(move || {
                bonfires::BONFIRES[selected].warp().send_error()
            });
        }
    }

    fn bosses_list(&self) -> List<'static> {
        let items: Vec<ListItem> = bosses::BOSSES.iter()
            .map(|boss| ListItem::from(boss.name)).collect();

        blockless_list(items, &self.tab, BOSSES_IDX)
            .block(block(Some("Bosses"), None).title(self.boss_alive_status_line().right_aligned()))
    }

    fn bonfires_list(&self) -> (List<'static>, List<'static>) {
        let revive_bonfire_id = bonfire::get_last_bonfire_id().unwrap_or_default();
        let items: (Vec<ListItem>, Vec<ListItem>) = bonfires::BONFIRES.iter()
            .map(|bonfire| {
                let name_text = if revive_bonfire_id == bonfire.bonfire_id {
                        format!("(*) {}", bonfire.name)
                    } else {
                        format!("{}", bonfire.name)
                    };
                    (
                        ListItem::from(name_text),
                        ListItem::from(Line::raw(bonfire.main_area)).fg(theme().muted)
                    )
                })
            .collect();
        (
            blockless_list(items.0, &self.tab, BONFIRES_IDX),
            label_list(items.1, &self.tab, BONFIRES_IDX)
        )
    }

    fn boss_alive_status_line(&self) -> Line<'static> {
        let selected_idx = self.tab.lists_states[BOSSES_IDX].selected().unwrap_or_default();
        let boss = &bosses::BOSSES[selected_idx];
        let mut style = Style::from(theme().success);
        let text = if !GameState::loaded() {
            "".to_string()
        } else {
            boss.revive_status().to_string()
        };
        if self.tab.current_list != BOSSES_IDX {
            style = Style::from(theme().fg)
        } else if text == "Dead" {
            style = Style::from(theme().error)
        }
        Line::from(text)
            .style(style)
    }

    fn bonfire_lit_status_line(&self) -> Line<'static> {
        let selected_idx = self.tab.lists_states[BONFIRES_IDX].selected().unwrap_or_default();
        let bonfire = &bonfires::BONFIRES[selected_idx];
        let lit = bonfire.is_lit().unwrap_or_default();
        let text = if !GameState::loaded() { "" } else if lit { "Lit" } else { "Unlit" };
        let style = if self.tab.current_list != BONFIRES_IDX {
            Style::from(theme().fg)
        } else if lit {
            Style::from(theme().success)
        } else {
            Style::from(theme().error)
        };
        Line::from(text)
            .style(style)
    }
}