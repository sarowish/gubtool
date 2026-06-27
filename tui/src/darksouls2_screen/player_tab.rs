use crate::{
    common::{
        StrExt, draw_popup_selector, stateful_list::StatefulList, tab_state::TabState, tabs_list,
    },
    darksouls2_screen::GameState,
    event::ResultExt,
    input::request_input,
    spawn_task,
};
use crossterm::event::{KeyCode, KeyEvent};
use darksouls2::{
    chr_ctrl::ChrCtrlExt,
    game_state::{StateFlagOffset, StateFlags},
    player::{self, StatOffset, player_ctrl},
    resources::covenants::{COVENANTS, Covenant},
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{List, ListItem},
};

enum ActionsItems {
    Health,
    Souls,
    Covenant,
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

pub enum StatsItems {
    Vigor,
    Endurance,
    Vitality,
    Attunement,
    Strength,
    Dexterity,
    Intelligence,
    Faith,
    Adaptability,
}

const TOGGLES_IDX: usize = 0;
const ACTIONS_IDX: usize = 1;
pub const STATS_IDX: usize = 2;

pub struct PlayerTab {
    tab: TabState,
    show_covenant_selector: bool,
    covenant_list: StatefulList,
    stats: player::Stats,
}

impl PlayerTab {
    pub fn new() -> Self {
        let mut list_states = vec![StatefulList::new(0); 3];
        list_states[TOGGLES_IDX] = StatefulList::new(TogglesItems::ARRAY.len());
        list_states[ACTIONS_IDX] = StatefulList::new(ActionsItems::ARRAY.len());
        list_states[STATS_IDX] = StatefulList::new(StatsItems::ARRAY.len());
        PlayerTab {
            tab: TabState::new(list_states),
            show_covenant_selector: false,
            covenant_list: StatefulList::new(COVENANTS.len()),
            stats: player::Stats::read(),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect) {
        self.stats = player::Stats::read();

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
            StatsItems::list(self),
            layout[STATS_IDX],
            &mut self.tab.get_list_state(STATS_IDX),
        );

        if self.show_covenant_selector {
            draw_popup_selector(
                "Select Covenant",
                &COVENANTS,
                &mut self.covenant_list.state,
                frame,
            );
        }
    }

    pub fn handle_keys(&mut self, key: KeyEvent) {
        if self.show_covenant_selector {
            self.covenant_list.handle_keys(key);
            match key.code {
                KeyCode::Char('q') |
                KeyCode::Esc => self.show_covenant_selector = false,
                KeyCode::Enter => {
                    if let Some(selected) = self.covenant_list.selected() {
                        let covenant = &COVENANTS[selected];
                        player::player_ctrl().set_covenant(*covenant).send_error();
                    }
                }
                _ => (),
            }
            return;
        }

        self.tab.handle_keys(key);

        match key.code {
            KeyCode::Enter => self.handle_enter(),
            _ => (),
        }

        if self.tab.current_list == STATS_IDX &&
        let Some(selected_idx) = self.tab.lists_states[STATS_IDX].selected() {
            match key.code {
                KeyCode::Char('h') => {
                    StatsItems::ARRAY[selected_idx]
                        .increment_stat(&self.stats, -1)
                }
                KeyCode::Char('l') => {
                    StatsItems::ARRAY[selected_idx]
                        .increment_stat(&self.stats, 1)
                }
                _ => (),
            }
        }
    }

    fn handle_enter(&mut self) {
        if let Some(selected) = self.tab.get_list_selected(self.tab.current_list) {
            match self.tab.current_list {
                ACTIONS_IDX => ActionsItems::ARRAY[selected].execute(self),
                TOGGLES_IDX => TogglesItems::ARRAY[selected].execute(),
                STATS_IDX => {
                    spawn_task! {
                        if let Some(val) = request_input(None).await {
                            StatsItems::ARRAY[selected].set_stat(val);
                        }
                    }
                }
                _ => (),
            }
        }
    }
}

impl ActionsItems {
    fn execute(&self, player_tab: &mut PlayerTab) {
        match self {
            Self::Covenant => player_tab.show_covenant_selector = true,
            Self::Health => {
                spawn_task! {
                    if let Some(val) = request_input::<i32>(None).await {
                        GameState::player_ctrl().set_hp(val).send_error();
                    }
                }
            },
            Self::Souls => {
                spawn_task! {
                    if let Some(val) = request_input::<u32>(None).await {
                        player::set_souls(val).send_error();
                    }
                }
            },
        }
    }
    fn set_input(&self) {
        match self {
            _ => (),
        }
    }
    fn to_list_item(&self) -> ListItem<'static> {
        let text = match self {
            Self::Covenant => format!(
                "Covenant: {}",
                player::player_ctrl().get_covenant().unwrap_or(Covenant::None)
            ),
            Self::Health => format!(
                "Health: {}",
                GameState::player_ctrl().get_hp().unwrap_or_default()
            ),
            Self::Souls => format!(
                "Souls: {}",
                player::get_souls().unwrap_or_default()
            ),
        };
        ListItem::new(text)
    }
    const ARRAY: &[ActionsItems] = &[
        Self::Health,
        Self::Souls,
        Self::Covenant,
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
                let new_state = !GameState::state_flags().player_no_death;
                StateFlags::set(StateFlagOffset::PlayerNoDeath, new_state).send_error();
                player_ctrl().set_no_death(new_state).ok();
            }
            Self::NoDamage => {
                let new_state = !player::is_no_damage();
                player::set_no_damage(new_state).send_error();
            }
            Self::InfinitePoise => {
                let new_state = !player::is_infinite_poise();
                player::set_infinite_poise(new_state).send_error();
            }
            Self::InfiniteStamina => {
                let new_state = !player::is_infinite_stamina();
                player::set_infinite_stamina(new_state).send_error();
            }
            Self::InfiniteConsumables => {
                let new_state = !player::is_infinite_consumables();
                player::set_infinite_consumables(new_state).send_error();
            }
            Self::NoHollowing => {
                let new_state = !player::is_no_hollowing();
                player::set_no_hollowing(new_state).send_error();
            }
            Self::InfiniteDurability => {
                let new_state = !player::is_infinite_durability();
                player::set_infinite_durability(new_state).send_error();
            }
            Self::NoSoulGain => {
                let new_state = !player::is_no_soul_gain();
                player::set_no_soul_gain(new_state).send_error();
            }
            Self::NoSoulLoss => {
                let new_state = !player::is_no_soul_loss();
                player::set_no_soul_loss(new_state).send_error();
            }
            Self::Hidden => {
                let new_state = !player::is_hidden();
                player::set_hidden(new_state).send_error();
            }
            Self::Silent => {
                let new_state = !player::is_silent();
                player::set_silent(new_state).send_error();
            }
        }
    }
    fn to_list_item(&self) -> ListItem<'_> {
        let text = match self {
            Self::NoDeath => {
                let state = GameState::state_flags().player_no_death;
                "No Death".create_toggle_str(state)
            }
            Self::NoDamage => {
                let state = player::is_no_damage();
                "No Damage".create_toggle_str(state)
            }
            Self::InfinitePoise => {
                let state = player::is_infinite_poise();
                "Infinite Poise".create_toggle_str(state)
            }
            Self::InfiniteStamina => {
                let state = player::is_infinite_stamina();
                "Infinite Stamina".create_toggle_str(state)
            }
            Self::InfiniteConsumables => {
                let state = player::is_infinite_consumables();
                "Infinite Consumables".create_toggle_str(state)
            }
            Self::NoHollowing => {
                let state = player::is_no_hollowing();
                "No Hollowing".create_toggle_str(state)
            }
            Self::InfiniteDurability => {
                let state = player::is_infinite_durability();
                "Infinite Durability".create_toggle_str(state)
            }
            Self::NoSoulGain => {
                let state = player::is_no_soul_gain();
                "No Soul Gain".create_toggle_str(state)
            }
            Self::NoSoulLoss => {
                let state = player::is_no_soul_loss();
                "No Soul Loss".create_toggle_str(state)
            }
            Self::Hidden => {
                let state = player::is_hidden();
                "Hidden".create_toggle_str(state)
            }
            Self::Silent => {
                let state = player::is_silent();
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
        // Self::NoSoulGain,
        Self::NoSoulLoss,
        Self::Hidden,
        Self::Silent,
    ];
    fn list(player_tab: &PlayerTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item()).collect();
        tabs_list(items, None, &player_tab.tab, TOGGLES_IDX)
    }
}

impl StatsItems {
    fn to_list_item(&self, stats: &player::Stats) -> ListItem<'_> {
        let text = match self {
            Self::Adaptability => format!("{:<2} Adaptability", stats.adaptability),
            Self::Endurance => format!("{:<2} Endurance", stats.endurance),
            Self::Vigor => format!("{:<2} Vigor", stats.vigor),
            Self::Vitality => format!("{:<2} Vitality", stats.vitality),
            Self::Attunement => format!("{:<2} Attunement", stats.attunement),
            Self::Intelligence => format!("{:<2} Intelligence", stats.intelligence),
            Self::Dexterity => format!("{:<2} Dexterity", stats.dexterity),
            Self::Faith => format!("{:<2} Faith", stats.faith),
            Self::Strength => format!("{:<2} Strength", stats.strength),
        };
        ListItem::from(text)
    }

    pub fn set_stat(&self, val: u16) {
        match self {
            Self::Adaptability => player::set_stat(StatOffset::Adaptability, val),
            Self::Endurance => player::set_stat(StatOffset::Endurance, val),
            Self::Vigor => player::set_stat(StatOffset::Vigor, val),
            Self::Vitality => player::set_stat(StatOffset::Vitality, val),
            Self::Attunement => player::set_stat(StatOffset::Attunement, val),
            Self::Intelligence => player::set_stat(StatOffset::Intelligence, val),
            Self::Dexterity => player::set_stat(StatOffset::Dexterity, val),
            Self::Faith => player::set_stat(StatOffset::Faith, val),
            Self::Strength => player::set_stat(StatOffset::Strength, val),
        }
        .send_error();
    }

    fn increment_stat(&self, stats: &player::Stats, val: i16) {
        match self {
            Self::Adaptability => self.set_stat(stats.adaptability.saturating_add_signed(val)),
            Self::Endurance => self.set_stat(stats.endurance.saturating_add_signed(val)),
            Self::Vigor => self.set_stat(stats.vigor.saturating_add_signed(val)),
            Self::Vitality => self.set_stat(stats.vitality.saturating_add_signed(val)),
            Self::Attunement => self.set_stat(stats.attunement.saturating_add_signed(val)),
            Self::Intelligence => self.set_stat(stats.intelligence.saturating_add_signed(val)),
            Self::Dexterity => self.set_stat(stats.dexterity.saturating_add_signed(val)),
            Self::Faith => self.set_stat(stats.faith.saturating_add_signed(val)),
            Self::Strength => self.set_stat(stats.strength.saturating_add_signed(val)),
        }
    }

    const ARRAY: &[StatsItems] = &[
        Self::Vigor,
        Self::Endurance,
        Self::Vitality,
        Self::Attunement,
        Self::Strength,
        Self::Dexterity,
        Self::Adaptability,
        Self::Intelligence,
        Self::Faith,
    ];

    fn list(player_tab: &PlayerTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item(&player_tab.stats)).collect();
        tabs_list(items, Some("Stats"), &player_tab.tab, STATS_IDX)
    }
}
