use crate::{
    common::{
        block, blockless_list, label_list, stateful_list::StatefulList, tab_state::TabState,
        tabs_list,
    }, event::{AnyhowExt}, input::{fuzzy_finder::FuzzyFinder, input_prompt::{InputPrompt, PromptType}}, theme::theme
};
use crossterm::event::{KeyCode, KeyEvent};
use darksouls2::{
    item::mass_spawn,
    resources::items::{Categories, Item, infusions::Infusion, items_array},
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
    Infusion,
}

pub struct ItemTab {
    tab: TabState,
    item: Item,
    pub quantity: u32,
    pub upgrade: u32,
    pub infusion: Infusion,
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
        list_states[ITEMS_IDX] = StatefulList::new(1);
        list_states[OPTIONS_IDX] = StatefulList::new(3);
        list_states[MASS_SPAWN_IDX] = StatefulList::new(Categories::ARRAY.len());
        ItemTab {
            tab: TabState::new(list_states),
            item: items_array()[0],
            quantity: 1,
            upgrade: 0,
            infusion: Infusion::Normal,
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
                Constraint::Length(5),
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
            self.tab.set_length(ITEMS_IDX, items_array().len());
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
                        SearchRequest::Infusion => {
                            let entries = self.item.available_infusions();
                            self.infusion = entries[selected];
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
                let list = items_array().iter()
                    .map(|item| Utf32String::from(format!("{}|{}", item.name, item.category)))
                    .collect();
                self.fuzzy_finder.show(list);
                self.search_request = Some(SearchRequest::Item);
            }
            KeyCode::Char('s') => {
                if self.tab.current_list == OPTIONS_IDX &&
                let Some(selected) = self.tab.get_list_selected(OPTIONS_IDX) {
                    OptionsItems::ARRAY[selected].set_input(self);
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
                mass_spawn(Categories::ARRAY[selected]).send_error();
            });
        }

        if self.tab.current_list == ITEMS_IDX || self.tab.current_list == OPTIONS_IDX {
            self.item.spawn(
                self.quantity as i32,
                self.upgrade as i32,
                self.infusion as u8 as i32,
            ).send_error();
        }
    }

    fn handle_input_enter(&mut self) {
        match self.input.last_request.unwrap() {
            InputRequest::Quantity => {
                if let Some(val) = self.input.parse_text::<u32>() {
                    self.quantity = val;
                    self.handle_item_switch()
                }
            }
            InputRequest::Upgrade => {
                if let Some(val) = self.input.parse_text::<u32>() {
                    self.upgrade = val;
                    self.handle_item_switch()
                }
            }
        }
    }

    fn items_list(&self) -> (List<'static>, List<'static>) {
        let items: (Vec<ListItem>, Vec<ListItem>) = items_array().iter()
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
        let new_item = items_array()[new_idx];
        self.item = new_item;

        if self.upgrade as i32 > self.item.max_upgrade.unwrap_or_default() {
            self.upgrade = self.item.max_upgrade.unwrap_or_default() as u32
        }

        if self.quantity as i32 > self.item.stack_size {
            self.quantity = self.item.stack_size as u32
        }

        if !self.item.available_infusions().contains(&self.infusion) {
            self.infusion = Infusion::Normal
        }
    }
    fn can_quantity(&self) -> bool {
        self.item.stack_size > 1
    }
    fn can_upgrade(&self) -> bool {
        self.item.max_upgrade.is_some()
    }
    fn can_infuse(&self) -> bool {
        self.item.infuse_id.is_some()
    }
}

impl OptionsItems {
    fn set_input(&self, item_tab: &mut ItemTab) {
        match self {
            Self::Quantity => {
                if item_tab.can_quantity() {
                    item_tab.input.show("Set New Value", PromptType::I32, InputRequest::Quantity)
                }
            },
            Self::Upgrade => {
                if item_tab.can_upgrade() {
                    item_tab.input.show("Set New Value", PromptType::I32, InputRequest::Upgrade)
                }
            },
            Self::Infusion => {
                if item_tab.can_infuse() {
                    let list = item_tab.item.available_infusions().iter()
                        .map(|infusion| Utf32String::from(format!("{}", infusion)))
                        .collect();
                    item_tab.fuzzy_finder.show(list);
                    item_tab.search_request = Some(SearchRequest::Infusion);
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
            Self::Infusion => {
                ListItem::new(format!("Affinity: {}", item_tab.infusion))
                    .style(options_style(item_tab.can_infuse()))
            }
        }
    }
    const ARRAY: &[OptionsItems] = &[
        Self::Quantity,
        Self::Upgrade,
        Self::Infusion,
    ];
    fn list(item_tab: &ItemTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item(item_tab)).collect();
        tabs_list(items, None, &item_tab.tab, OPTIONS_IDX)
    }
}

fn options_style(show: bool) -> Style {
    if show { Style::default() } else { Style::new().add_modifier(Modifier::CROSSED_OUT) }
}

#[derive(Clone, Copy)]
enum InputRequest {
    Quantity,
    Upgrade,
}

#[derive(Clone, Copy)]
enum SearchRequest {
    Item,
    Infusion,
}