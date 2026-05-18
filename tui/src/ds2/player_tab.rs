use crate::{
    common::{StrExt, stateful_list::StatefulList, tab_state::TabState, tabs_list},
    ds2::state_flags,
    event::ResultExt,
};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use darksouls2::{
    chr_ctrl::ChrCtrlExt,
    game_state::{StateFlags, StateFlagsOffsets},
    player::{self, player_ctrl},
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{List, ListItem},
};

enum ActionsItems {
}

enum TogglesItems {
    NoDeath,
    NoDamage,
    InfinitePoise,
    InfiniteStamina,
    InfiniteConsumables,
    NoHollowing,
    InfiniteDurability,
    NoSoulGain,
    NoSoulLoss,
    Hidden,
    Silent,
}

pub enum Stats {
}

const TOGGLES_IDX: usize = 0;
const ACTIONS_IDX: usize = 1;
pub const STATS_IDX: usize = 2;

pub struct PlayerTab {
    tab: TabState,
}

impl PlayerTab {
    pub fn new() -> Self {
        let mut list_states = vec![StatefulList::new(0); 3];
        list_states[TOGGLES_IDX] = StatefulList::new(TogglesItems::ARRAY.len());
        list_states[ACTIONS_IDX] = StatefulList::new(ActionsItems::ARRAY.len());
        list_states[STATS_IDX] = StatefulList::new(0);
        PlayerTab {
            tab: TabState::new(list_states),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect) {
        let [area_one, right] = Layout::default()
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
            .areas(right);

        let layout = [area_one, area_two, area_three];

        frame.render_stateful_widget(
            ActionsItems::list(self),
            layout[ACTIONS_IDX],
            &mut self.tab.get_list_state(ACTIONS_IDX),
        );
        frame.render_stateful_widget(
            TogglesItems::list(self),
            layout[TOGGLES_IDX],
            &mut self.tab.get_list_state(TOGGLES_IDX),
            );
        frame.render_stateful_widget(
            Stats::list(self),
            layout[STATS_IDX],
            &mut self.tab.get_list_state(STATS_IDX),
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

    fn handle_input(&mut self) {
        if let Some(selected) = self.tab.get_list_selected(self.tab.current_list) {
            match self.tab.current_list {
                ACTIONS_IDX => ActionsItems::ARRAY[selected].set_input(),
                STATS_IDX => Stats::ARRAY[selected].set_input(),
                _ => (),
            }
        }
    }
    fn handle_enter(&mut self) {
        if let Some(selected) = self.tab.get_list_selected(self.tab.current_list) {
            match self.tab.current_list {
                ACTIONS_IDX => ActionsItems::ARRAY[selected].execute(),
                TOGGLES_IDX => TogglesItems::ARRAY[selected].execute(),
                STATS_IDX => Stats::ARRAY[selected].set_input(),
                _ => (),
            }
        }
    }
}

impl ActionsItems {
    fn execute(&self) {
        match self {
            _ => (),
        }
    }
    fn set_input(&self) {
        match self {
            _ => (),
        }
    }
    fn to_list_item(&self) -> ListItem<'static> {
        let text = match self {
            _ => "",
        };
        ListItem::new(text)
    }
    const ARRAY: &[ActionsItems] = &[
    ];
    fn list(player_tab: &PlayerTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item()).collect();
        tabs_list(items, None, &player_tab.tab, ACTIONS_IDX)
    }
}

impl TogglesItems {
    fn execute(&self) {
        match self {
            Self::NoDeath => {
                let new_state = !state_flags().player_no_death;
                StateFlags::set(StateFlagsOffsets::PlayerNoDeath, new_state).send_error();
                player_ctrl().set_no_death(new_state).ok();
            }
            Self::NoDamage => {
                let new_state = !player::is_no_damage().unwrap_or_default();
                player::set_no_damage(new_state).send_error();
            }
            Self::InfinitePoise => {
                let new_state = !player::is_infinite_poise().unwrap_or_default();
                player::set_infinite_poise(new_state).send_error();
            }
            Self::InfiniteStamina => {
                let new_state = !player::is_infinite_stamina().unwrap_or_default();
                player::set_infinite_stamina(new_state).send_error();
            }
            Self::InfiniteConsumables => {
                let new_state = !player::is_infinite_consumables().unwrap_or_default();
                player::set_infinite_consumables(new_state).send_error();
            }
            Self::NoHollowing => {
                let new_state = !player::is_no_hollowing().unwrap_or_default();
                player::set_no_hollowing(new_state).send_error();
            }
            Self::InfiniteDurability => {
                let new_state = !player::is_infinite_durability().unwrap_or_default();
                player::set_infinite_durability(new_state).send_error();
            }
            Self::NoSoulGain => {
                let new_state = !player::is_no_soul_gain().unwrap_or_default();
                player::set_no_soul_gain(new_state).send_error();
            }
            Self::NoSoulLoss => {
                let new_state = !player::is_no_soul_loss().unwrap_or_default();
                player::set_no_soul_loss(new_state).send_error();
            }
            Self::Hidden => {
                let new_state = !player::is_hidden().unwrap_or_default();
                player::set_hidden(new_state).send_error();
            }
            Self::Silent => {
                let new_state = !player::is_silent().unwrap_or_default();
                player::set_silent(new_state).send_error();
            }
        }
    }
    fn to_list_item(&self) -> ListItem<'_> {
        let text = match self {
            Self::NoDeath => {
                let state = state_flags().player_no_death;
                "No Death".create_toggle_str(state)
            }
            Self::NoDamage => {
                let state = player::is_no_damage().unwrap_or_default();
                "No Damage".create_toggle_str(state)
            }
            Self::InfinitePoise => {
                let state = player::is_infinite_poise().unwrap_or_default();
                "Infinite Poise".create_toggle_str(state)
            }
            Self::InfiniteStamina => {
                let state = player::is_infinite_stamina().unwrap_or_default();
                "Infinite Stamina".create_toggle_str(state)
            }
            Self::InfiniteConsumables => {
                let state = player::is_infinite_consumables().unwrap_or_default();
                "Infinite Consumables".create_toggle_str(state)
            }
            Self::NoHollowing => {
                let state = player::is_no_hollowing().unwrap_or_default();
                "No Hollowing".create_toggle_str(state)
            }
            Self::InfiniteDurability => {
                let state = player::is_infinite_durability().unwrap_or_default();
                "Infinite Durability".create_toggle_str(state)
            }
            Self::NoSoulGain => {
                let state = player::is_no_soul_gain().unwrap_or_default();
                "No Soul Gain".create_toggle_str(state)
            }
            Self::NoSoulLoss => {
                let state = player::is_no_soul_loss().unwrap_or_default();
                "No Soul Loss".create_toggle_str(state)
            }
            Self::Hidden => {
                let state = player::is_hidden().unwrap_or_default();
                "Hidden".create_toggle_str(state)
            }
            Self::Silent => {
                let state = player::is_silent().unwrap_or_default();
                "Silent".create_toggle_str(state)
            }
        };
        ListItem::from(text)
    }
    const ARRAY: &[TogglesItems] = &[
        Self::NoDeath,
        Self::NoDamage,
        Self::InfinitePoise,
        Self::InfiniteStamina,
        Self::InfiniteDurability,
        Self::InfiniteConsumables,
        Self::NoHollowing,
        Self::NoSoulGain,
        Self::NoSoulLoss,
        Self::Hidden,
        Self::Silent,
    ];
    fn list(player_tab: &PlayerTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item()).collect();
        tabs_list(items, None, &player_tab.tab, TOGGLES_IDX)
    }
}

impl Stats {
    fn to_list_item(&self) -> ListItem<'_> {
        let text = match self {
            _ => ""
        };
        ListItem::from(text)
    }
    fn set_input(&self) {

    }

    pub fn set_stat(&self) -> Result<()> {
        match self {
            _ => Ok(()),
        }
    }
    fn increment_stat(&self) -> Result<()> {
        match self {
            _ => Ok(()),
        }
    }
    const ARRAY: &[Stats] = &[
    ];
    fn list(player_tab: &PlayerTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item()).collect();
        tabs_list(items, Some("Stats"), &player_tab.tab, STATS_IDX)
    }
}
