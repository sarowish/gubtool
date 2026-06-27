use crate::{
    app::App,
    common::{
        block, blockless_list, label_list, stateful_list::StatefulList, tab_state::TabState,
        tabs_list,
    },
    event::AnyhowExt,
    input::{request_input, request_search},
    mutate_app, spawn_task,
    theme::theme,
};
use crossterm::event::{KeyCode, KeyEvent};
use darksouls2::{
    item,
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
    quantity: u32,
    upgrade: u32,
    infusion: Infusion,
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
    }

    pub fn handle_keys(&mut self, key: KeyEvent) {
        self.handle_item_switch();

        if self.tab.current_list == ITEMS_IDX {
            self.tab.set_length(ITEMS_IDX, items_array().len());
        }

        self.tab.handle_keys(key);

        match key.code {
            KeyCode::Enter => {
                self.handle_enter()
            }
            KeyCode::Char('f') => {
                let entries = items_array().iter()
                    .map(|item| Utf32String::from(format!("{}|{}", item.name, item.category)))
                    .collect();
                spawn_task! {
                    if let Some(selected) = request_search(entries).await {
                        mutate_app!(|app: &mut App| {
                            let items_tab = &mut app.dark_souls_2.items;
                            items_tab.tab.set_list_selected(ITEMS_IDX, selected);
                            items_tab.handle_item_switch();
                        });
                    }
                }
            }
            _ => ()
        }
        self.handle_item_switch();
    }

    fn handle_enter(&mut self) {
        let Some(selected) = self.tab.current_list_selected() else { return };

        match self.tab.current_list {
            ITEMS_IDX => {
                self.item.spawn(
                    self.quantity as i32,
                    self.upgrade as i32,
                    self.infusion as u8 as i32,
                ).send_error();
            }
            OPTIONS_IDX => {
                OptionsItems::ARRAY[selected].execute(self);

            }
            MASS_SPAWN_IDX => {
                thread::spawn(move || {
                    item::mass_spawn(Categories::ARRAY[selected]).send_error();
                });
            }
            _ => (),
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
    fn execute(&self, item_tab: &mut ItemTab) {
        match self {
            Self::Quantity => {
                if item_tab.can_quantity() {
                    spawn_task! {
                        if let Some(val) = request_input::<u32>(None).await {
                            mutate_app!(|app: &mut App| {
                                let items_tab = &mut app.dark_souls_2.items;
                                items_tab.quantity = val;
                                items_tab.handle_item_switch();
                            });
                        }
                    }
                }
            },
            Self::Upgrade => {
                if item_tab.can_upgrade() {
                    spawn_task! {
                        if let Some(val) = request_input::<u32>(None).await {
                            mutate_app!(|app: &mut App| {
                                let items_tab = &mut app.dark_souls_2.items;
                                items_tab.upgrade = val;
                                items_tab.handle_item_switch();
                            });
                        }
                    }
                }
            },
            Self::Infusion => {
                if item_tab.can_infuse() {
                    let entries = item_tab.item.available_infusions().iter()
                        .map(|infusion| Utf32String::from(format!("{}", infusion)))
                        .collect();
                    spawn_task! {
                        if let Some(selected) = request_search(entries).await {
                            mutate_app!(|app: &mut App| {
                                let items_tab = &mut app.dark_souls_2.items;
                                let entries = items_tab.item.available_infusions();
                                items_tab.infusion = entries[selected];
                            });
                        }
                    }
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