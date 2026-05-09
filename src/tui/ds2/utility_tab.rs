use crate::{
    ds2::{
        game_state,
        utility::{self},
    },
    tui::{
        common::{
            StrExt, blockless_list, stateful_list::StatefulList, tab_state::TabState, tabs_list,
        },
        event::ResultExt,
    },
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{List, ListItem},
};

enum OptionsItems {
    IvorySkip,
    SkipCredits,
    FastQuitout,
}

enum TigglesItems {
}

const OPTIONS_IDX: usize = 0;
const TOGGLES_IDX: usize = 1;
const MENUS_IDX: usize = 2;

pub struct UtilityTab {
    pub tab: TabState,
}

impl UtilityTab {
    pub fn new() -> Self {
        let mut list_states = vec![StatefulList::new(0); 4];
        list_states[OPTIONS_IDX] = StatefulList::new(OptionsItems::ARRAY.len());
        list_states[TOGGLES_IDX] = StatefulList::new(TigglesItems::ARRAY.len());
        list_states[MENUS_IDX] = StatefulList::new(0);
        UtilityTab {
            tab: TabState::new(list_states),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect) {
        let [area_one, right_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .areas(layout);

        let [area_two, area_three] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .areas(right_area);

        let layout = [area_one, area_two, area_three];

        frame.render_stateful_widget(
            OptionsItems::list(self),
            layout[OPTIONS_IDX],
            &mut self.tab.get_list_state(OPTIONS_IDX),
        );
    }
    pub fn handle_keys(&mut self, key: KeyEvent) {
        self.tab.handle_keys(key);
        match key.code {
            KeyCode::Char('s') => self.handle_input(),
            KeyCode::Enter => self.handle_enter(),
            _ => (),
        }
    }

    fn handle_input(&self) {
        if let Some(selected) = self.tab.get_list_selected(self.tab.current_list) {
            match self.tab.current_list {
                OPTIONS_IDX => OptionsItems::ARRAY[selected].set_input(),
                TOGGLES_IDX => TigglesItems::ARRAY[selected].set_input(),
                _ => (),
            }
        }
    }

    fn handle_enter(&self) {
        if let Some(selected) = self.tab.get_list_selected(self.tab.current_list) {
            match self.tab.current_list {
                OPTIONS_IDX => OptionsItems::ARRAY[selected].execute(),
                TOGGLES_IDX => TigglesItems::ARRAY[selected].execute(),
                MENUS_IDX => (),
                _ => (),
            }
        }
    }
}

impl OptionsItems {
    fn execute(&self) {
        match self {
            Self::IvorySkip => {
                let new_state = !utility::is_ivory_skip().unwrap_or_default();
                utility::set_ivory_skip(new_state).send_error();
            }
            Self::SkipCredits => {
                let new_state = !utility::is_credits_skip().unwrap_or_default();
                utility::set_credits_skip(new_state).send_error();
            }
            Self::FastQuitout => {
                let new_state = !game_state::get_state_flag(game_state::GameStateFlags::FastQuitout);
                game_state::set_state_flag(game_state::GameStateFlags::FastQuitout, new_state).send_error();
            }
        }
    }
    fn set_input(&self) {
        match self {
            _ => (),
        }
    }
    fn to_list_item(&self) -> ListItem<'_> {
        let text = match self {
            Self::IvorySkip => {
                let state = utility::is_ivory_skip().unwrap_or_default();
                "Skip Ivory King Gauntlet".create_toggle_str(state)
            }
            Self::SkipCredits => {
                let state = utility::is_credits_skip().unwrap_or_default();
                "Skip Credits".create_toggle_str(state)
            }
            Self::FastQuitout => {
                let state = game_state::get_state_flag(game_state::GameStateFlags::FastQuitout);
                "Fast Quitout".create_toggle_str(state)
            }
        };
        ListItem::new(text)
    }
    const ARRAY: &[OptionsItems] = &[
        Self::FastQuitout,
        Self::SkipCredits,
        Self::IvorySkip,
    ];
    fn list(utility_tab: &UtilityTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item()).collect();
        tabs_list(items, None, &utility_tab.tab, OPTIONS_IDX)
    }
}

impl TigglesItems {
    fn execute(&self) {
        match self {
            _ => ()
        }
    }
    fn set_input(&self) {
        match self {
            _ => ()
        }
    }
    fn to_list_item(&self) -> ListItem<'_> {
        let text = match self {
            _ => ""
        };
        ListItem::new(text)
    }
    const ARRAY: &[TigglesItems] = &[
    ];
    fn list(utility_tab: &UtilityTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item()).collect();
        blockless_list(items, &utility_tab.tab, TOGGLES_IDX)
    }
}