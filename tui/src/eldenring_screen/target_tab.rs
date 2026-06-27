use crate::{
    app::App,
    common::{StrExt, stateful_list::StatefulList, tab_state::TabState, tabs_list},
    eldenring_screen::GameState,
    event::{AnyhowExt, ResultExt},
    input::request_input,
    mutate_app, spawn_task,
    theme::theme,
};
use crossterm::event::{KeyCode, KeyEvent};
use eldenring::{chr_ins::ChrInsExt, player, target};
use num_format::{
    Locale::{self},
    ToFormattedString,
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::{LineGauge, List, ListItem, Paragraph},
};
use shared::act_array::ActArray;
use std::thread;

enum ActionsItems {
    Health,
    HealthPercentage,
    Kill,
    NextPhase,
    RepeatAction,
    ForceActSequenceHeader,
    ForceActSequence,
    ActSequence,
    ResetPosition,
}

enum TogglesItems {
    NoDamage,
    NoStagger,
    DisableAi,
    RepeatLastAction,
}

const ACTIONS_IDX: usize = 0;
const TOGGLES_IDX: usize = 1;

pub struct TargetTab {
    tab: TabState,
    act_array: ActArray,
    show_act_sequence: bool,
}

impl TargetTab {
    pub fn new() -> Self {
        let mut list_states = vec![StatefulList::new(0); 2];
        list_states[TOGGLES_IDX] = StatefulList::new(TogglesItems::ARRAY.len());
        TargetTab {
            tab: TabState::new(list_states),
            act_array: ActArray::default(),
            show_act_sequence: false,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect) {
        let [chr_name, hp, poise, paragraph_area, main] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(1),
                Constraint::Length(6),
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
        frame.render_widget(Self::poise_line_gauge(), poise);
        frame.render_widget(Self::paragraph(), paragraph_area);

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
        self.tab.set_length(ACTIONS_IDX, ActionsItems::array(self).len());

        self.tab.handle_keys(key);

        if key.code == KeyCode::Enter {
            if let Some(selected_index) = self.tab.current_list_selected() {
                match self.tab.current_list {
                    ACTIONS_IDX => ActionsItems::array(self)[selected_index].execute(self),
                    TOGGLES_IDX => TogglesItems::ARRAY[selected_index].execute(),
                    _ => (),
                }
            }
        }
    }

    fn hp_line_gauge() -> LineGauge<'static> {
        let current = GameState::target_ins().get_current_hp().unwrap_or_default();
        let max = GameState::target_ins().get_max_hp().unwrap_or_default();
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
        let current = GameState::target_ins().get_current_poise().unwrap_or_default();
        let max = GameState::target_ins().get_max_poise().unwrap_or_default();
        LineGauge::default()
            .label(format!(
                "{:<22}", format!("Poise: {:.1}/{:.1}", current, max)))
            .filled_style(Style::from(theme().fg).bg(theme().fg).bold())
            .ratio(if max > 0.0 { (current as f64 / max as f64).clamp(0.0, 1.0) } else { 0.0 })
        .style(Style::from(theme().fg))
    }

    fn chr_name_paragraph() -> Paragraph<'static> {
        Paragraph::new(GameState::target_ins().name_from_chr_id())
        .centered()
        .style(Style::from(theme().fg))
        .bold()
    }

    fn paragraph() -> Paragraph<'static> {
        let poise_timer = GameState::target_ins().get_poise_timer().unwrap_or_default().abs();
        let last_act = GameState::target_ins().get_last_act().unwrap_or_default();
        let current_animation = GameState::target_ins().get_current_animation().unwrap_or_default();
        let distance = GameState::target_ins()
            .get_distance(&player::player_ins())
            .unwrap_or_default();
        Paragraph::new(format!(
            "Reset Timer: {:.1}\n\nLast Act: {last_act}\nCurrent Animation: {current_animation}\nDistance: {:.1}",
            poise_timer, distance
        ))
        .style(Style::from(theme().fg))
    }
}

impl ActionsItems {
    fn execute(&self, target_tab: &mut TargetTab) {
        match self {
            Self::Kill => GameState::target_ins().set_hp(0).send_error(),
            Self::NextPhase => {
                thread::spawn(|| {
                    GameState::target_ins().next_phase().send_error()
                });
            }
            Self::Health => {
                spawn_task! {
                    if let Some(val) = request_input::<i32>(None).await {
                        GameState::target_ins().set_hp(val).send_error()
                    }
                }
            }
            Self::HealthPercentage => {
                spawn_task! {
                    if let Some(val) = request_input::<f32>(None).await {
                        GameState::target_ins().set_hp_pct(val).send_error()
                    }
                }
            }
            Self::RepeatAction => {
                spawn_task! {
                    if let Some(val) = request_input::<u8>(Some("Enter act id")).await {
                        GameState::target_ins().repeat_act(val).send_error()
                    }
                }
            }
            Self::ForceActSequence => {
                target::force_act_sequence(
                    target_tab.act_array.to_owned(),
                    GameState::target_ins().npc_think_param_id().unwrap_or_default(),
                ).send_error()
            }
            Self::ActSequence => {
                spawn_task! {
                    if let Some(val) = request_input::<ActArray>(Some("Enter act ids seperated by spaces")).await {
                        mutate_app!(|app: &mut App| {
                            app.elden_ring.target.act_array = val
                        });
                    }
                }
            }
            Self::ResetPosition => GameState::target_ins().reset_position().send_error(),
            Self::ForceActSequenceHeader => {
                target_tab.show_act_sequence = !target_tab.show_act_sequence
            },
        }
    }
    fn to_list_item(&self, target: &TargetTab) -> ListItem<'_> {
        let text = match self {
            Self::Kill => "Kill".to_string(),
            Self::NextPhase => "Next Phase".to_string(),
            Self::Health => format!("Health: {}", GameState::target_ins().get_current_hp().unwrap_or_default()),
            Self::HealthPercentage => format!("Health %: {:.2}%", GameState::target_ins().get_hp_pct().unwrap_or_default()),
            Self::RepeatAction => "Repeat Action".to_string(),
            Self::ForceActSequence => "  Force".to_string(),
            Self::ActSequence => format!("  Sequence: {}", target.act_array),
            Self::ResetPosition => "Reset Position".to_string(),
            Self::ForceActSequenceHeader => "Force Act Sequence".to_string(),
        };
        ListItem::new(text)
    }
    const ARRAY_NO_SEQUENCE: &[ActionsItems] = &[
        Self::Health,
        Self::HealthPercentage,
        Self::Kill,
        Self::NextPhase,
        Self::RepeatAction,
        Self::ForceActSequenceHeader,
        Self::ResetPosition,
    ];
    const ARRAY_SHOW_SEQUENCE: &[ActionsItems] = &[
        Self::Health,
        Self::HealthPercentage,
        Self::Kill,
        Self::NextPhase,
        Self::RepeatAction,
        Self::ForceActSequenceHeader,
        Self::ForceActSequence,
        Self::ActSequence,
        Self::ResetPosition,
    ];
    fn array(target_tab: &TargetTab) -> &'static [ActionsItems] {
        if target_tab.show_act_sequence { Self::ARRAY_SHOW_SEQUENCE } else { Self::ARRAY_NO_SEQUENCE}
    }
    fn list(target_tab: &TargetTab) -> List<'static> {
        let items: Vec<ListItem> = Self::array(target_tab).iter().map(|i| i.to_list_item(target_tab)).collect();
        tabs_list(items, None, &target_tab.tab, ACTIONS_IDX)
    }
}

impl TogglesItems {
    fn execute(&self) {
        match self {
            Self::NoDamage => {
                let new_state = !GameState::target_ins().is_no_damage().unwrap_or_default();
                GameState::target_ins().set_no_damage(new_state).send_error()
            }
            Self::RepeatLastAction => {
                let new_state = !GameState::target_ins().is_repeat_act().unwrap_or_default();
                GameState::target_ins().set_repeat_last_act(new_state).send_error()
            }
            Self::DisableAi => {
                let new_state = !GameState::target_ins().is_disable_ai().unwrap_or_default();
                GameState::target_ins().set_disable_ai(new_state).send_error()
            }
            Self::NoStagger => {
                target::toggle_stagger_hook().send_error()
            }
        }
    }
    fn to_list_item(&self) -> ListItem<'_> {
        let text = match self {
            Self::NoDamage => {
                let state = GameState::target_ins().is_no_damage().unwrap_or_default();
                "No Damage".create_toggle_str(state)
            }
            Self::NoStagger => {
                let state = target::is_stagger_hook_active().unwrap_or_default();
                "No Stagger".create_toggle_str(state)
            }
            Self::DisableAi => {
                let state = GameState::target_ins().is_disable_ai().unwrap_or_default();
                "Disable AI".create_toggle_str(state)
            }
            Self::RepeatLastAction => {
                let state = GameState::target_ins().is_repeat_act().unwrap_or_default();
                "Repeat Last Action".create_toggle_str(state)
            }
        };
        ListItem::from(text)
    }
    const ARRAY: &[TogglesItems] = &[
        Self::NoDamage,
        Self::NoStagger,
        Self::DisableAi,
        Self::RepeatLastAction,
    ];

    fn list(target_tab: &TargetTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item()).collect();
        tabs_list(items, None, &target_tab.tab, TOGGLES_IDX)
    }
}