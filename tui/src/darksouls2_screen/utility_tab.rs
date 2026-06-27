use crate::{
    common::{StrExt, stateful_list::StatefulList, tab_state::TabState, tabs_list},
    darksouls2_screen::GameState,
    event::ResultExt,
    input::request_input,
    spawn_task,
    theme::theme,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use darksouls2::{
    game_state::{StateFlagOffset, StateFlags},
    menu,
    resources::menus::{MENUS, SHOPS, TRADES},
    utility::{self},
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    symbols,
    widgets::{List, ListItem, Tabs},
};

enum TogglesItem {
    SkipCredits,
    FastQuitout,
    DisableRoll,
    DisableBackstep,
}

enum OptionsItem {
    NewGame,
}

const TOGGLES_IDX: usize = 0;
const OPTIONS_IDX: usize = 1;
const MENUS_IDX: usize = 2;
const SHOPS_IDX: usize = 3;
const TRADES_IDX: usize = 4;

pub struct UtilityTab {
    tab: TabState,
    menu_shop_idx: usize,
}

impl UtilityTab {
    pub fn new() -> Self {
        let mut list_states = vec![StatefulList::new(0); 5];
        list_states[TOGGLES_IDX] = StatefulList::new(TogglesItem::ARRAY.len());
        list_states[OPTIONS_IDX] = StatefulList::new(OptionsItem::ARRAY.len());
        list_states[MENUS_IDX] = StatefulList::new(MENUS.len());
        list_states[SHOPS_IDX] = StatefulList::new(SHOPS.len());
        list_states[TRADES_IDX] = StatefulList::new(TRADES.len());
        UtilityTab {
            tab: TabState::new(list_states),
            menu_shop_idx: 0,
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
            TogglesItem::list(self),
            layout[TOGGLES_IDX],
            &mut self.tab.get_list_state(TOGGLES_IDX),
        );
        frame.render_stateful_widget(
            OptionsItem::list(self),
            layout[OPTIONS_IDX],
            &mut self.tab.get_list_state(OPTIONS_IDX),
        );

        match self.menu_shop_idx {
            0 => {
                frame.render_stateful_widget(
                    self.menus_list(),
                    layout[MENUS_IDX],
                    &mut self.tab.get_list_state(MENUS_IDX),
                )
            }
            1 => {
                frame.render_stateful_widget(
                    self.shops_list(),
                    layout[MENUS_IDX],
                    &mut self.tab.get_list_state(SHOPS_IDX),
                )
            }
            2 => {
                frame.render_stateful_widget(
                    self.trades_list(),
                    layout[MENUS_IDX],
                    &mut self.tab.get_list_state(TRADES_IDX),
                )
            }
            _ => (),
        }
        frame.render_widget(self.menu_shop_tab(), layout[MENUS_IDX]);
    }
    pub fn handle_keys(&mut self, key: KeyEvent) {
        self.tab.handle_keys(key);

        match self.tab.current_list {
            MENUS_IDX => {
                if self.menu_shop_idx == 1 {
                    self.tab.current_list = SHOPS_IDX
                }
                if self.menu_shop_idx == 2 {
                    self.tab.current_list = TRADES_IDX
                }
                if key.code == KeyCode::Char('l') {
                    self.tab.current_list = SHOPS_IDX;
                    self.menu_shop_idx = 1
                }
            }
            SHOPS_IDX => match (key.code, key.modifiers) {
                (KeyCode::Char('h'), _) => {
                    self.tab.current_list = MENUS_IDX;
                    self.menu_shop_idx = 0
                }
                (KeyCode::Char('l'), _) => {
                    self.tab.current_list = TRADES_IDX;
                    self.menu_shop_idx = 2
                }
                (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                    self.tab.current_list = 1;
                }
                _ => (),
            },
            TRADES_IDX => match (key.code, key.modifiers) {
                (KeyCode::Char('h'), _) => {
                    self.tab.current_list = SHOPS_IDX;
                    self.menu_shop_idx = 1
                }
                (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                    self.tab.current_list = 1;
                }
                _ => (),
            }
            _ => (),
        }
        match key.code {
            KeyCode::Enter => self.handle_enter(),
            _ => (),
        }
    }

    fn handle_enter(&mut self) {
        if let Some(selected) = self.tab.get_list_selected(self.tab.current_list) {
            match self.tab.current_list {
                TOGGLES_IDX => TogglesItem::ARRAY[selected].execute(),
                OPTIONS_IDX => OptionsItem::ARRAY[selected].execute(),
                MENUS_IDX => menu::open_menu(MENUS[selected]).send_error(),
                SHOPS_IDX => menu::open_shop(SHOPS[selected]).send_error(),
                TRADES_IDX => menu::open_trade(TRADES[selected]).send_error(),
                _ => (),
            }
        }
    }

    fn menu_shop_tab(&self) -> Tabs<'static> {
        Tabs::new(vec!["Menus", "Shops", "Trades"])
            .highlight_style(
                match self.menu_shop_idx {
                    0 => self.tab.block_style(MENUS_IDX).fg(theme().secondary),
                    1 => self.tab.block_style(SHOPS_IDX).fg(theme().secondary),
                    _ => self.tab.block_style(TRADES_IDX).fg(theme().secondary),
                })
            .select(self.menu_shop_idx)
            .divider(symbols::DOT)
    }

    fn menus_list(&self) -> List<'static> {
        let items: Vec<ListItem> = MENUS
            .iter()
            .map(|menu| ListItem::from(menu.to_string()))
            .collect();
        tabs_list(items, None, &self.tab, MENUS_IDX)
    }
    fn shops_list(&self) -> List<'static> {
        let items: Vec<ListItem> = SHOPS
            .iter()
            .map(|shop| ListItem::from(shop.to_string()))
            .collect();
        tabs_list(items, None, &self.tab, SHOPS_IDX)
    }
    fn trades_list(&self) -> List<'static> {
        let items: Vec<ListItem> = TRADES
            .iter()
            .map(|trade| ListItem::from(trade.to_string()))
            .collect();
        tabs_list(items, None, &self.tab, TRADES_IDX)
    }
}

impl TogglesItem {
    fn execute(&self) {
        match self {
            Self::SkipCredits => {
                let new_state = !utility::is_credits_skip();
                utility::set_credits_skip(new_state).send_error();
            }
            Self::FastQuitout => {
                let new_state = !GameState::state_flags().fast_quitout;
                StateFlags::set(StateFlagOffset::FastQuitout, new_state).send_error();
            }
            Self::DisableRoll => {
                let new_state = !utility::is_disable_roll();
                utility::set_disable_roll(new_state).send_error();
            }
            Self::DisableBackstep => {
                let new_state = !utility::is_disable_backstep();
                utility::set_disable_backstep(new_state).send_error();
            }
        }
    }
    fn to_list_item(&self) -> ListItem<'_> {
        let text = match self {
            Self::SkipCredits => {
                let state = utility::is_credits_skip();
                "Skip Credits".create_toggle_str(state)
            }
            Self::FastQuitout => {
                let state = GameState::state_flags().fast_quitout;
                "Fast Quitout".create_toggle_str(state)
            }
            Self::DisableRoll => {
                let state = utility::is_disable_roll();
                "Disable Roll".create_toggle_str(state)
            }
            Self::DisableBackstep => {
                let state = utility::is_disable_backstep();
                "Disable Backstep".create_toggle_str(state)
            }
        };
        ListItem::new(text)
    }
    const ARRAY: &[TogglesItem] = &[
        Self::FastQuitout,
        Self::SkipCredits,
        Self::DisableRoll,
        Self::DisableBackstep,
    ];
    fn list(utility_tab: &UtilityTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item()).collect();
        tabs_list(items, None, &utility_tab.tab, TOGGLES_IDX)
    }
}

impl OptionsItem {
    fn execute(&self) {
        match self {
            Self::NewGame => {
                spawn_task! {
                    if let Some(val) = request_input::<u8>(None).await {
                        utility::set_ng(val).send_error();
                    }
                }
            }
        }
    }
    fn to_list_item(&self) -> ListItem<'_> {
        let text = match self {
            Self::NewGame => format!("New Game: {}", utility::get_ng().unwrap_or_default())
        };
        ListItem::new(text)
    }
    const ARRAY: &[OptionsItem] = &[
        Self::NewGame
    ];
    fn list(utility_tab: &UtilityTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item()).collect();
        tabs_list(items, None, &utility_tab.tab, OPTIONS_IDX)
    }
}