use crate::{
    common::{stateful_list::StatefulList, tab_state::TabState, tabs_list}, darksouls2_screen::{GAME_STATE, GameState, target_tab::ActionsItems::KillTarget}, event::ResultExt, theme::theme
};
use crossterm::event::{KeyCode, KeyEvent};
use darksouls2::{chr_ctrl::ChrCtrlExt, target::{self, target_ctrl}};
use num_format::{Locale, ToFormattedString};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::{LineGauge, List, ListItem, Paragraph},
};

enum ActionsItems {
    KillTarget
}

enum TogglesItems {
}

const ACTIONS_IDX: usize = 0;
const TOGGLES_IDX: usize = 1;

pub struct TargetTab {
    tab: TabState,
}

impl TargetTab {
    pub fn new() -> Self {
        let mut list_states = vec![StatefulList::new(0); 2];
        list_states[ACTIONS_IDX] = StatefulList::new(ActionsItems::ARRAY.len());
        list_states[TOGGLES_IDX] = StatefulList::new(TogglesItems::ARRAY.len());
        TargetTab {
            tab: TabState::new(list_states),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect) {
        unsafe {
            GAME_STATE.target_ctrl = target::target_ctrl()
        }

        let [chr_name, hp, posture, poise, _paragraph_area, main] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(1),
                Constraint::Length(5),
                Constraint::Fill(1),
            ])
            .areas(layout);

        let lists_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ])
            .split(main);

        frame.render_widget(Self::chr_name_paragraph(), chr_name);
        frame.render_widget(Self::hp_line_gauge(), hp);
        frame.render_widget(Self::posture_line_gauge(), posture);
        frame.render_widget(Self::poise_line_gauge(), poise);
        //frame.render_widget(Self::paragraph(), paragraph_area);

        frame.render_stateful_widget(
            ActionsItems::list(&self),
            lists_layout[ACTIONS_IDX],
            &mut self.tab.get_list_state(ACTIONS_IDX),
        );
        frame.render_stateful_widget(
            TogglesItems::list(&self),
            lists_layout[TOGGLES_IDX],
            &mut self.tab.get_list_state(TOGGLES_IDX),
        );
    }

    pub fn handle_keys(&mut self, key: KeyEvent) {
        self.tab.handle_keys(key);
        match key.code {
            KeyCode::Char('s') => self.handle_input(),
            KeyCode::Enter => self.handle_select(),
            _ => (),
        }
    }

    fn handle_input(&self) {
        let current_list = self.tab.current_list;
        if let Some(selected_index) = self.tab.lists_states[current_list].selected() {
            match current_list {
                ACTIONS_IDX => ActionsItems::ARRAY[selected_index].set_input(),
                _ => (),
            }
        }
    }

    fn handle_select(&self) {
        let current_list = self.tab.current_list;
        if let Some(selected_index) = self.tab.lists_states[current_list].selected() {
            match current_list {
                ACTIONS_IDX => ActionsItems::ARRAY[selected_index].execute(),
                TOGGLES_IDX => TogglesItems::ARRAY[selected_index].execute(),
                _ => (),
            }
        }
    }

    fn hp_line_gauge() -> LineGauge<'static> {
        let current = GameState::target_ctrl().get_hp().unwrap_or_default();
        let max = GameState::target_ctrl().max_hp().unwrap_or_default();
        LineGauge::default()
            .label(format!(
                "{:<22}", format!("Health: {}/{}",
                    current.to_formatted_string(&Locale::en),
                    max.to_formatted_string(&Locale::en)
                )
            ))
            .filled_style(Style::from(theme().fg).bg(theme().fg).bold())
            .ratio(if max > 0 { (current as f64 / max as f64).clamp(0.0, 1.0) } else { 0.0 })
        .style(Style::from(theme().fg))
    }

    fn poise_line_gauge() -> LineGauge<'static> {
        let current = GameState::target_ctrl().poise().unwrap_or_default();
        let max = GameState::target_ctrl().max_poise().unwrap_or_default();
        let vals = if max != 0.0 {
            format!("{:.1}/{:.1}", current, max)
        } else {
            "Immune".to_string()
        };
        LineGauge::default()
            .label(format!(
                "{:<22}", format!("Poise: {vals}")))
            .filled_style(Style::from(theme().fg).bg(theme().fg).bold())
            .ratio(if max > 0.0 { (current as f64 / max as f64).clamp(0.0, 1.0) } else { 0.0 })
        .style(Style::from(theme().fg))
    }

    fn posture_line_gauge() -> LineGauge<'static> {
        let current = GameState::target_ctrl().posture().unwrap_or_default();
        let max = GameState::target_ctrl().max_posture().unwrap_or_default();
        LineGauge::default()
            .label(format!(
                "{:<22}", format!("Posture: {}/{}",
                    (current as i64).to_formatted_string(&Locale::en),
                    (max as i64).to_formatted_string(&Locale::en)
            )))
            .filled_style(Style::from(theme().fg).bg(theme().fg).bold())
            .ratio(if max > 0.0 { (current as f64 / max as f64).clamp(0.0, 1.0) } else { 0.0 })
        .style(Style::from(theme().fg))
    }

    fn chr_name_paragraph() -> Paragraph<'static> {
        Paragraph::new(GameState::target_ctrl().name_from_chr_id())
        .centered()
        .style(Style::from(theme().fg))
        .bold()
    }

    // fn paragraph() -> Paragraph<'static> {
        // let poise_timer = GameState::target_ins().get_poise_timer().unwrap_or_default().abs();
        // let last_act = GameState::target_ins().get_last_act().unwrap_or_default();
        // let current_animation = GameState::target_ins().get_current_animation().unwrap_or_default();
        // let distance = GameState::target_ins()
            // .get_distance(&player::player_ins())
            // .unwrap_or_default();
        // Paragraph::new(format!(
            // "Reset Timer: {:.1}\n\nLast Act: {last_act}\nCurrent Animation: {current_animation}\nDistance: {:.1}",
            // poise_timer, distance
        // ))
        // .style(Style::from(theme().fg))
    // }
}

impl ActionsItems {
    fn execute(&self) {
        match self {
            KillTarget => target_ctrl().set_hp(0).send_error(),
        }
    }
    fn set_input(&self) {
        match self {
            _ => (),
        }
    }
    fn to_list_item(&self) -> ListItem<'_> {
        let text = match self {
            KillTarget => "Kill",
        };
        ListItem::new(text)
    }
    const ARRAY: &[ActionsItems] = &[
        KillTarget
    ];
    fn list(target_tab: &TargetTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item()).collect();
        tabs_list(items, None, &target_tab.tab, ACTIONS_IDX)
    }
}

impl TogglesItems {
    fn execute(&self) {
        match self {
            _ => (),
        }
    }
    fn to_list_item(&self) -> ListItem<'_> {
        let text = match self {
            _ => "",
        };
        ListItem::from(text)
    }
    const ARRAY: &[TogglesItems] = &[
    ];

    fn list(target_tab: &TargetTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item()).collect();
        tabs_list(items, None, &target_tab.tab, TOGGLES_IDX)
    }
}