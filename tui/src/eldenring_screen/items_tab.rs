use crate::{
    common::{
        block, blockless_list, label_list, stateful_list::StatefulList, tab_state::TabState,
        tabs_list,
    }, eldenring_screen::GameState, event::{AnyhowExt}, input::{fuzzy_finder::FuzzyFinder, input_prompt::{InputPrompt, PromptType}}, theme::theme
};
use crossterm::event::{KeyCode, KeyEvent};
use eldenring::{
    item,
    resources::{
        aow::{AFFINITIES, Affinity, Aow, aow_array},
        items::{Categories, Item, items_array},
    },
};
use nucleo_matcher::Utf32String;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::Line,
    widgets::{List, ListItem},
};
use std::thread;

enum OptionsItems {
    Quantity,
    Upgrade,
    AshOfWar,
    Affinity,
}

pub struct ItemTab {
    tab: TabState,
    item: Item,
    pub quantity: u64,
    pub upgrade: u64,
    aow: Aow,
    affinity: Affinity,
    input: InputPrompt<InputRequest>,
    fuzzy_finder: FuzzyFinder,
    search_request: Option<SearchRequest>,
}

const ITEMS_IDX: usize = 0;
const OPTIONS_IDX: usize = 1;
const MASS_SPAWN_IDX: usize = 2;

impl ItemTab {
    pub fn new() -> Self {
        let mut list_states = vec![StatefulList::new(0); 3];
        list_states[ITEMS_IDX] = StatefulList::new(0);
        list_states[OPTIONS_IDX] = StatefulList::new(OptionsItems::ARRAY.len());
        list_states[MASS_SPAWN_IDX] = StatefulList::new(Categories::ARRAY.len());
        ItemTab {
            tab: TabState::new(list_states),
            item: items_array(false)[0],
            quantity: 1,
            upgrade: 0,
            aow: aow_array()[0],
            affinity: AFFINITIES[0],
            input: InputPrompt::new(),
            fuzzy_finder: FuzzyFinder::default(),
            search_request: None,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect) {
        let [item_area, right_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(60),
                Constraint::Fill(1)
            ])
            .areas(layout);

        let items_block = block(Some("Items"), Some(self.tab.block_style(ITEMS_IDX)));
        frame.render_widget(&items_block, item_area);
        let inner = items_block.inner(item_area);

        let [item_name, item_category] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(40),
                Constraint::Max(25)])
            .areas(inner);

        let [options, mass_spawn] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(6),
                Constraint::Fill(1)
            ])
            .areas(right_area);

        let (item_names, item_labels) = self.items_list();
        frame.render_stateful_widget(
            item_names,
            item_name,
            &mut self.tab.get_list_state(ITEMS_IDX),
        );
        frame.render_stateful_widget(
            item_labels,
            item_category,
            &mut self.tab.get_list_state(ITEMS_IDX),
        );
        frame.render_stateful_widget(
            OptionsItems::list(&self),
            options,
            &mut self.tab.get_list_state(OPTIONS_IDX),
        );
        frame.render_stateful_widget(
            self.mass_spawn_list(),
            mass_spawn,
            &mut self.tab.get_list_state(MASS_SPAWN_IDX),
        );

        self.input.draw_popup_checked(frame);
        self.fuzzy_finder.draw_checked(frame);
    }

    pub fn handle_keys(&mut self, key: KeyEvent) {
        self.handle_item_switch();

        if self.tab.current_list == ITEMS_IDX {
            self.tab.set_length(ITEMS_IDX, items_array(GameState::dlc()).len());
        }

        if self.input.show {
            self.input.handle_keys(key);
            if key.code == KeyCode::Enter {
                self.handle_input_enter();
            }
            return;
        }

        if self.fuzzy_finder.show {
            self.fuzzy_finder.handle_keys(key);
            if key.code == KeyCode::Enter {
                if let Some(selected) = self.fuzzy_finder.selected_idx() {
                    match self.search_request.unwrap() {
                        SearchRequest::Item => {
                            self.tab.set_list_selected(ITEMS_IDX, selected);
                            self.handle_item_switch();
                        }
                        SearchRequest::Affinity => {
                            let entries: Vec<Affinity> = AFFINITIES.iter()
                                .filter(|affinity| self.aow.supports_affinity(affinity.flag))
                                .cloned().collect();
                            self.affinity = entries[self.fuzzy_finder.selected_idx().unwrap()];
                        }
                        SearchRequest::Aow => {
                            let entries: Vec<Aow> = aow_array().iter()
                                .filter(|aow| aow.supports_item(self.item))
                                .cloned().collect();
                            self.aow = entries[selected];
                        }
                    }
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
                let list = items_array(GameState::dlc()).iter()
                    .map(|item| Utf32String::from(format!("{}|{}", item.name, item.category)))
                    .collect();
                self.fuzzy_finder.show(list);
                self.search_request = Some(SearchRequest::Item);
            }
            KeyCode::Char('s') => {
                if self.tab.current_list == OPTIONS_IDX &&
                let Some(selected_idx) = self.tab.get_list_selected(OPTIONS_IDX) {
                    OptionsItems::ARRAY[selected_idx].set_input(self);
                }
            }
            _ => ()
        }
        self.handle_item_switch();
    }

    fn handle_select(&self) {
        if self.tab.current_list == MASS_SPAWN_IDX &&
        let Some(selected) = self.tab.get_list_selected(MASS_SPAWN_IDX) {
            thread::spawn(move || {
                item::mass_spawn(Categories::ARRAY[selected]).send_error();
            });
        }

        if self.tab.current_list == ITEMS_IDX || self.tab.current_list == OPTIONS_IDX {
            self.item.spawn(
                self.quantity as i64,
                self.upgrade as i64,
                self.aow,
                self.affinity,
            ).send_error();
        }
    }

    fn handle_input_enter(&mut self) {
        match self.input.last_request.unwrap() {
            InputRequest::Quantity => {
                if let Some(val) = self.input.parse_text::<u64>() {
                    self.quantity = val;
                    self.handle_item_switch()
                }
            }
            InputRequest::Upgrade => {
                if let Some(val) = self.input.parse_text::<u64>() {
                    self.upgrade = val;
                    self.handle_item_switch()
                }
            }
        }
    }

    fn items_list(&self) -> (List<'static>, List<'static>) {
        let items: (Vec<ListItem>, Vec<ListItem>) = items_array(GameState::dlc()).iter()
            .map(|item| (
                    ListItem::from(item.name),
                    ListItem::from(Line::raw(format!("{}", item.category)).fg(theme().muted))
            ))
            .collect();
        (
            blockless_list(items.0, &self.tab, ITEMS_IDX),
            label_list(items.1, &self.tab, ITEMS_IDX)
        )
    }

    fn mass_spawn_list(&self) -> List<'static> {
        let items: Vec<ListItem> = Categories::ARRAY.iter().map(|item| ListItem::from(Line::raw(item.to_string()))).collect();
        tabs_list(items, Some("Mass Spawn"), &self.tab, MASS_SPAWN_IDX)
    }

    pub fn handle_item_switch(&mut self) {
        let Some(new_idx) = self.tab.get_list_selected(ITEMS_IDX) else { return };
        let new_item = items_array(GameState::dlc())[new_idx];
        self.item = new_item;

        if let Some(new_quantity) = new_item.clamp_quantity(self.quantity as i64) {
            self.quantity = new_quantity as u64;
        }

        if let Some(new_upgrade) = new_item.clamp_upgrade(self.upgrade as i64) {
            self.upgrade = new_upgrade as u64;
        }

        if !self.aow.supports_item(new_item) {
            self.aow = aow_array()[0];
        }
        if !self.aow.supports_affinity(self.affinity.flag) {
            self.affinity = AFFINITIES[0];
        }
    }

    fn can_aow(&self) -> bool {
        self.item.weapon_type.is_some() && (self.item.gem_mount_type != Some(0))
    }

    fn can_upgrade(&self) -> bool {
        matches!(self.item.category, Categories::Weapons | Categories::SpiritAshes)
    }

    fn can_quantity(&self) -> bool {
        self.item.stack_size > 1
    }
}

impl OptionsItems {
    fn set_input(&self, item_tab: &mut ItemTab) {
        match self {
            Self::Quantity => {
                if item_tab.can_quantity() {
                    item_tab.input.show("Set New Value", PromptType::U64, InputRequest::Quantity)
                }
            },
            Self::Upgrade => {
                if item_tab.can_upgrade() {
                    item_tab.input.show("Set New Value", PromptType::U64, InputRequest::Upgrade)
                }
            },
            Self::AshOfWar => {
                if item_tab.can_aow() {
                    let list = aow_array().iter()
                        .filter(|aow| aow.supports_item(item_tab.item))
                        .map(|aow| Utf32String::from(aow.name))
                        .collect();
                    item_tab.fuzzy_finder.show(list);
                    item_tab.search_request = Some(SearchRequest::Aow);
                }
            },
            Self::Affinity => {
                if item_tab.can_aow() {
                    let list = AFFINITIES.iter()
                        .filter(|affinity| item_tab.aow.supports_affinity(affinity.flag))
                        .map(|affinity| Utf32String::from(affinity.name))
                        .collect();
                    item_tab.fuzzy_finder.show(list);
                    item_tab.search_request = Some(SearchRequest::Affinity);
                }
            },
        }
    }
    fn to_list_item(&self, item_tab: &ItemTab) -> ListItem<'static> {
        match self {
            Self::Quantity => {
                ListItem::new(format!("Quantity: {}", item_tab.quantity))
                    .style(options_style(item_tab.can_quantity()))
            }
            Self::Upgrade => {
                ListItem::new(format!("Upgrade: {}", item_tab.upgrade))
                    .style(options_style(item_tab.can_upgrade()))
            }
            Self::AshOfWar => {
                ListItem::new(format!("Ash of War: {}", item_tab.aow.name))
                    .style(options_style(item_tab.can_aow()))
            }
            Self::Affinity => {
                ListItem::new(format!("Affinity: {}", item_tab.affinity.name))
                    .style(options_style(item_tab.can_aow()))
            }
        }
    }
    const ARRAY: &[OptionsItems] = &[
        Self::Quantity,
        Self::Upgrade,
        Self::AshOfWar,
        Self::Affinity,
    ];
    fn list(item_tab: &ItemTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item(item_tab)).collect();
        tabs_list(items, None, &item_tab.tab, OPTIONS_IDX)
    }
}
fn options_style(show: bool) -> Style {
    if show {
        Style::default()
    } else {
        Style::new()
            .add_modifier(Modifier::CROSSED_OUT)
    }
}

#[derive(Clone, Copy)]
enum InputRequest {
    Quantity,
    Upgrade,
}

#[derive(Clone, Copy)]
enum SearchRequest {
    Item,
    Aow,
    Affinity,
}