use crate::{
    config::{Config, user::AttachConfig},
    er::{
        game_state::{self, GameStateFlags},
        resources::talk_commands::{MENUS, shops_array},
        utility,
    },
    send_input_event,
    tui::{
        common::{StrExt, stateful_list::StatefulList, tab_state::TabState, tabs_list},
        er::ErInfo,
        event::ResultExt,
        theme::theme,
    },
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    symbols,
    widgets::{List, ListItem, Tabs},
};

enum TogglesItems {
    RemoveLogos,
    ToggleMusic,
    ShowAllMaps,
    ShowAllGraces,
    StutterFix,
    FreezeWorld,
    DisableAreaTitleCards,
    DrawHitboxesA,
}

enum ActionsItems {
    FpsCap,
    GameSpeed,
    Quitout,
    ClearCount,
    TriggerNewGameCycle,
}

const OPTIONS_IDX: usize = 0;
const ACTIONS_IDX: usize = 1;
const MENUS_IDX: usize = 2;
const SHOPS_IDX: usize = 3;

pub struct UtilityTab {
    pub tab: TabState,
    preferences: AttachConfig,
    menu_shop_idx: usize,
}

impl UtilityTab {
    pub fn new() -> Self {
        let mut list_states = vec![StatefulList::new(0); 4];
        list_states[OPTIONS_IDX] = StatefulList::new(TogglesItems::ARRAY.len());
        list_states[ACTIONS_IDX] = StatefulList::new(ActionsItems::ARRAY.len());
        list_states[MENUS_IDX] = StatefulList::new(MENUS.len());
        list_states[SHOPS_IDX] = StatefulList::new(0);
        UtilityTab {
            tab: TabState::new(list_states),
            preferences: AttachConfig::read().unwrap_or_default(),
            menu_shop_idx: 0,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect, er: &ErInfo) {
        self.preferences = AttachConfig::read().unwrap_or_default();

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
            TogglesItems::list(self),
            layout[OPTIONS_IDX],
            &mut self.tab.get_list_state(OPTIONS_IDX),
        );
        frame.render_stateful_widget(
            ActionsItems::list(self),
            layout[ACTIONS_IDX],
            &mut self.tab.get_list_state(ACTIONS_IDX),
        );

        if self.menu_shop_idx == 1 {
            frame.render_stateful_widget(
                self.shops_list(er.dlc),
                layout[MENUS_IDX],
                &mut self.tab.get_list_state(SHOPS_IDX),
            );
        } else {
            frame.render_stateful_widget(
                self.menus_list(),
                layout[MENUS_IDX],
                &mut self.tab.get_list_state(MENUS_IDX),
            );
        }
        frame.render_widget(self.menu_shop_tab(), layout[MENUS_IDX]);
    }

    fn menu_shop_tab(&self) -> Tabs<'static> {
        Tabs::new(vec!["Menus", "Shops"])
            .highlight_style(
                if self.menu_shop_idx == 0 {
                    self.tab.block_style(MENUS_IDX).fg(theme().secondary)
                } else {
                    self.tab.block_style(SHOPS_IDX).fg(theme().secondary)
                })
            .select(self.menu_shop_idx)
            .divider(symbols::DOT)
    }

    pub fn handle_keys(&mut self, key: KeyEvent, er: &ErInfo) {
        if self.tab.current_list == SHOPS_IDX {
            self.tab.set_length(SHOPS_IDX, shops_array(er.dlc).len())
        }
        self.tab.handle_keys(key);
        match self.tab.current_list {
            MENUS_IDX if self.menu_shop_idx == 1 => self.tab.current_list = SHOPS_IDX,
            SHOPS_IDX if self.menu_shop_idx == 0 => self.tab.current_list = MENUS_IDX,
            _ => (),
        }
        match self.tab.current_list {
            SHOPS_IDX => match (key.code, key.modifiers) {
                (KeyCode::Char('h'), _) => {
                    self.tab.current_list = MENUS_IDX;
                    self.menu_shop_idx = 0
                }
                _ => (),
            },
            MENUS_IDX => {
                if key.code == KeyCode::Char('l') {
                    self.tab.current_list = SHOPS_IDX;
                    self.menu_shop_idx = 1
                }
            }
            _ => (),
        }
        match key.code {
            KeyCode::Char('s') => self.handle_input(),
            KeyCode::Enter => self.handle_enter(er.dlc),
            _ => (),
        }
    }

    fn handle_input(&self) {
        if let Some(selected) = self.tab.get_list_selected(self.tab.current_list) {
            match self.tab.current_list {
                ACTIONS_IDX => ActionsItems::ARRAY[selected].set_input(),
                _ => (),
            }
        }
    }

    fn handle_enter(&self, dlc: bool) {
        if let Some(selected) = self.tab.get_list_selected(self.tab.current_list) {
            match self.tab.current_list {
                OPTIONS_IDX => TogglesItems::ARRAY[selected].execute(),
                ACTIONS_IDX => ActionsItems::ARRAY[selected].execute(),
                MENUS_IDX => MENUS[selected].execute().send_error(),
                SHOPS_IDX => shops_array(dlc)[selected].execute().send_error(),
                _ => (),
            }
        }
    }

    fn menus_list(&self) -> List<'static> {
        let items: Vec<ListItem> = MENUS.iter().map(|menu| ListItem::new(menu.name)).collect();
        tabs_list(items, None, &self.tab, MENUS_IDX)
    }

    fn shops_list(&self, dlc: bool) -> List<'static> {
        let items: Vec<ListItem> = shops_array(dlc).iter().map(|shop| ListItem::from(shop.name)).collect();
        tabs_list(items, None, &self.tab, SHOPS_IDX)
    }
}

impl TogglesItems {
    fn execute(&self) {
        match self {
            Self::ToggleMusic => {
                let new_state = !utility::is_music_muted().unwrap_or_default();
                utility::mute_music(new_state).send_error()
            }
            Self::RemoveLogos => {
                let new_state = !utility::is_logo_patch().unwrap_or_default();
                utility::set_logo_patch(new_state).send_error()
            }
            Self::ShowAllMaps => {
                let new_state = !utility::is_show_all_maps_on().unwrap_or_default();
                utility::show_all_maps(new_state).send_error()
            }
            Self::ShowAllGraces => {
                let new_state = !utility::is_show_all_graces_on().unwrap_or_default();
                utility::show_all_graces(new_state).send_error()
            }
            Self::StutterFix => {
                let new_state = !game_state::get_state_flag(GameStateFlags::StutterFix);
                game_state::set_state_flag(GameStateFlags::StutterFix, new_state).send_error();
            }
            Self::FreezeWorld => {
                let new_state = !utility::is_freeze_world_on().unwrap_or_default();
                utility::set_freeze_world(new_state).send_error()
            }
            Self::DisableAreaTitleCards => {
                let new_state = !game_state::get_state_flag(GameStateFlags::TitleCards);
                game_state::set_state_flag(GameStateFlags::TitleCards, new_state).send_error();
            }
            Self::DrawHitboxesA => {
                let new_state = !utility::is_hitboxes(false).unwrap_or_default();
                utility::draw_hitboxes(new_state, false).send_error()
            }
        }
    }
    fn to_list_item(&self) -> ListItem<'_> {
        let text = match self {
            Self::ToggleMusic => {
                let state = utility::is_music_muted().unwrap_or_default();
                "Mute Music".create_toggle_str(state)
            }
            Self::RemoveLogos => {
                let state = utility::is_logo_patch().unwrap_or_default();
                "Remove Logos".create_toggle_str(state)
            }
            Self::ShowAllMaps => {
                let state = utility::is_show_all_maps_on().unwrap_or_default();
                "Show All Maps".create_toggle_str(state)
            }
            Self::ShowAllGraces => {
                let state = utility::is_show_all_graces_on().unwrap_or_default();
                "Show All Graces".create_toggle_str(state)
            }
            Self::StutterFix => {
                let state = game_state::get_state_flag(GameStateFlags::StutterFix);
                "Stutter Fix".create_toggle_str(state)
            }
            Self::FreezeWorld => {
                let state = utility::is_freeze_world_on().unwrap_or_default();
                "Freeze World".create_toggle_str(state)
            }
            Self::DisableAreaTitleCards => {
                let state = game_state::get_state_flag(GameStateFlags::TitleCards);
                "Disable Area Title Cards".create_toggle_str(state)
            }
            Self::DrawHitboxesA => {
                let state = utility::is_hitboxes(false).unwrap_or_default();
                "Draw Hitboxes".create_toggle_str(state)
            }
        };
        ListItem::new(text)
    }
    const ARRAY: &[TogglesItems] = &[
        Self::FreezeWorld,
        Self::ToggleMusic,
        Self::RemoveLogos,
        Self::DisableAreaTitleCards,
        Self::DrawHitboxesA,
        Self::ShowAllGraces,
        Self::ShowAllMaps,
        Self::StutterFix,
    ];
    fn list(utility_tab: &UtilityTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item()).collect();
        tabs_list(items, None, &utility_tab.tab, OPTIONS_IDX)
    }
}

impl ActionsItems {
    fn execute(&self) {
        match self {
            Self::FpsCap => {
                send_input_event!(text, _app, {
                    if let Ok(v) = text.parse() {
                        utility::set_fps_cap(v).send_error()
                    }
                })
            }
            Self::GameSpeed => {
                send_input_event!(text, _app, {
                    if let Ok(v) = text.parse() {
                        utility::set_game_speed(v).send_error()
                    }
                })
            }
            Self::Quitout => {
                utility::quitout().send_error()
            }
            Self::ClearCount => {
                send_input_event!(text, _app, {
                    if let Ok(v) = text.parse() {
                        utility::set_ng_cycle(v).send_error()
                    }
                })
            }
            Self::TriggerNewGameCycle => {
                utility::trigger_new_game().send_error()
            }
        }
    }
    fn set_input(&self) {
        match self {
            Self::FpsCap | Self::GameSpeed | Self::ClearCount => {
                self.execute()
            },
            _ => (),
        }
    }
    fn to_list_item(&self) -> ListItem<'_> {
        let text = match self {
            Self::FpsCap => {
                format!("FPS Cap: {}",
                    utility::get_fps_cap().unwrap_or_default())
            }
            Self::GameSpeed => {
                format!("Game Speed: {}",
                    utility::get_game_speed().unwrap_or_default())
            }
            Self::Quitout => {
                "Quitout".to_string()
            }
            Self::ClearCount => {
                format!("ClearCount: {}",
                    utility::get_ng_cycle().unwrap_or_default())
            }
            Self::TriggerNewGameCycle => {
                "Trigger New Game Cycle".to_string()
            }
        };
        ListItem::new(text)
    }
    const ARRAY: &[ActionsItems] = &[
        Self::FpsCap,
        Self::GameSpeed,
        Self::ClearCount,
        Self::TriggerNewGameCycle,
        Self::Quitout,
    ];
    fn list(utility_tab: &UtilityTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item()).collect();
        tabs_list(items, None, &utility_tab.tab, ACTIONS_IDX)
    }
}