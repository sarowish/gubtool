use crate::{
    common::{StrExt, stateful_list::StatefulList, tab_state::TabState, tabs_list}, eldenring_screen::GameState, event::{AnyhowExt, ResultExt}, input::input_prompt::{InputPrompt, PromptType}, ui_state::UiState
};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use eldenring::{
    chr_ins::ChrInsExt,
    emevd,
    game_state::{StateFlagOffset, StateFlags},
    player::{
        self, ChrDbgOffsets, PlayerGameDataOffset, PlayerStats, is_chr_dbg_flag, torrent_ins,
    },
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{List, ListItem},
};

enum ActionsItems {
    SetHealth,
    Die,
    GiveRunes,
    AnimationSpeed,
    Rest,
}

enum TogglesItems {
    NoDeath,
    NoDamage,
    InfinitePoise,
    SetRfbsOnLoad,
    OneShot,
    RuneArc,
    Silent,
    Hidden,
    InfiniteStamina,
    InfiniteFp,
    InfiniteConsumables,
    InfiniteArrows,
    TorrentAnywhere,
    TorrentNoDeath,
}

pub enum Stats {
    Vigor,
    Mind,
    Endurance,
    Strength,
    Dexterity,
    Intelligence,
    Faith,
    Arcane,
    Scadutree,
    SpiritAsh,
    RuneLevel,
    RuneMem,
}

const TOGGLES_IDX: usize = 0;
const ACTIONS_IDX: usize = 1;
pub const STATS_IDX: usize = 2;

pub struct PlayerTab {
    pub tab: TabState,
    pub stats: PlayerStats,
    pub hp: i32,
    pub runes: i64,
    input: InputPrompt<InputRequest>,
}

impl PlayerTab {
    pub fn new() -> Self {
        let mut list_states = vec![StatefulList::new(0); 3];
        list_states[TOGGLES_IDX] = StatefulList::new(TogglesItems::ARRAY.len());
        list_states[ACTIONS_IDX] = StatefulList::new(ActionsItems::ARRAY.len());
        list_states[STATS_IDX] = StatefulList::new(0);
        PlayerTab {
            tab: TabState::new(list_states),
            stats: PlayerStats::new(),
            hp: 100,
            runes: 10000,
            input: InputPrompt::new(),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect) {
        self.stats.update().ok();

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

        self.input.draw_popup_checked(frame);
    }

    pub fn handle_keys(&mut self, key: KeyEvent) {
        if self.tab.current_list == STATS_IDX {
            self.tab.set_length(STATS_IDX, Stats::array().len());
        }

        if self.input.show {
            self.input.handle_keys(key);
            if key.code == KeyCode::Enter {
                self.handle_input_enter();
            }
            return;
        }

        self.tab.handle_keys(key);
        match key.code {
            KeyCode::Char('s') => self.handle_input(),
            KeyCode::Enter => self.handle_enter(),
            _ => (),
        }
        if self.tab.current_list == STATS_IDX &&
        let Some(selected_idx) = self.tab.lists_states[STATS_IDX].selected() {
            match key.code {
                KeyCode::Char('h') => {
                    Stats::array()[selected_idx]
                        .increment_stat(&self.stats, -1)
                        .send_error();
                }
                KeyCode::Char('l') => {
                    Stats::array()[selected_idx]
                        .increment_stat(&self.stats, 1)
                        .send_error();
                }
                _ => (),
            }
        }
    }
    fn handle_input(&mut self) {
        let current_list = self.tab.current_list;
        if let Some(selected_index) = self.tab.lists_states[current_list].selected() {
            match current_list {
                ACTIONS_IDX => ActionsItems::ARRAY[selected_index].set_input(&mut self.input),
                STATS_IDX => Stats::ARRAY[selected_index].set_input(&mut self.input),
                _ => (),
            }
        }
    }
    fn handle_enter(&mut self) {
        let current_list = self.tab.current_list;
        if let Some(selected_index) = self.tab.lists_states[current_list].selected() {
            match current_list {
                ACTIONS_IDX => ActionsItems::ARRAY[selected_index].execute(self),
                TOGGLES_IDX => TogglesItems::ARRAY[selected_index].execute(&self.stats),
                STATS_IDX => Stats::ARRAY[selected_index].set_input(&mut self.input),
                _ => (),
            }
        }
    }

    fn handle_input_enter(&mut self) {
        match self.input.last_request.unwrap() {
            InputRequest::Health => {
                if let Some(val) = self.input.parse_text::<i32>() {
                    self.hp = val;
                    UiState::update_er(|c| { c.player_set_health = val; }).ok();
                }
            }
            InputRequest::Runes => {
                if let Some(val) = self.input.parse_text::<i64>() {
                    self.runes = val;
                    UiState::update_er(|c| { c.give_runes = val; }).ok();
                }
            }
            InputRequest::AnimationSpeed => {
                if let Some(val) = self.input.parse_text::<f32>() {
                    GameState::player_ins().set_animation_speed(val).send_error()
                }
            }
            InputRequest::Stat => {
                if let Some(val) = self.input.parse_text::<i32>() {
                    let idx = self.tab.lists_states[STATS_IDX].selected().unwrap_or_default();
                    let stat  = &Stats::array()[idx];
                    stat.set_stat(val).send_error();
                }
            }
        }
    }
}

impl ActionsItems {
    fn execute(&self, player_tab: &mut PlayerTab) {
        match self {
            Self::SetHealth => GameState::player_ins().set_hp(player_tab.hp).send_error(),
            Self::Die => GameState::player_ins().set_hp(0).send_error(),
            Self::Rest => emevd::rest().send_error(),
            Self::GiveRunes => player::give_runes(player_tab.runes).send_error(),
            Self::AnimationSpeed => player_tab.input.show("Set Animation Speed", PromptType::F32, InputRequest::AnimationSpeed),
        }
    }
    fn set_input(&self, input: &mut InputPrompt<InputRequest>) {
        match self {
            Self::SetHealth => input.show("Set New Value", PromptType::I32, InputRequest::Health),
            Self::GiveRunes => input.show("Set New Value", PromptType::I64, InputRequest::Runes),
            _ => (),
        }
    }
    fn to_list_item(&self, player_tab: &PlayerTab) -> ListItem<'static> {
        let text = match self {
            Self::SetHealth => {
                format!("Set Health ({})", player_tab.hp)
            }
            Self::Die => {
                "Die".to_string()
            }
            Self::Rest => {
                "Rest".to_string()
            }
            Self::GiveRunes => {
                format!("Give Runes ({})", player_tab.runes)
            }
            Self::AnimationSpeed => {
                format!("Animation Speed: {}",
                    GameState::player_ins().get_animation_speed().unwrap_or_default())
            }
        };
        ListItem::new(text)
    }
    const ARRAY: &[ActionsItems] = &[
        Self::SetHealth,
        Self::GiveRunes,
        Self::AnimationSpeed,
        Self::Die,
        Self::Rest,
    ];
    fn list(player_tab: &PlayerTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item(player_tab)).collect();
        tabs_list(items, None, &player_tab.tab, ACTIONS_IDX)
    }
}

impl TogglesItems {
    fn execute(&self, stats: &PlayerStats) {
        match self {
            Self::NoDeath => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffsets::PlayerNoDeath).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffsets::PlayerNoDeath, new_state).send_error();
            }
            Self::NoDamage => {
                let new_state = !GameState::state_flags().player_no_damage;
                StateFlags::set(StateFlagOffset::PlayerNoDamage, new_state).send_error();
                GameState::player_ins().set_no_damage(new_state).ok();
            }
            Self::SetRfbsOnLoad => {
                let new_state = !GameState::state_flags().rfbs;
                StateFlags::set(StateFlagOffset::Rfbs, new_state).send_error();
            }
            Self::InfinitePoise => {
                let new_state = !player::is_infinite_poise().unwrap_or_default();
                player::set_infinite_poise(new_state).send_error();
            }
            Self::OneShot => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffsets::OneShot).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffsets::OneShot, new_state).send_error();
            }
            Self::RuneArc => {
                let new_state = !(stats.rune_arc || GameState::state_flags().rune_arc);
                StateFlags::set(StateFlagOffset::RuneArc, new_state).send_error();
                player::set_rune_arc(new_state).ok();
            }
            Self::InfiniteStamina => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffsets::InfiniteStam).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffsets::InfiniteStam , new_state).send_error();
            }
            Self::InfiniteFp => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffsets::InfiniteFp).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffsets::InfiniteFp, new_state).send_error();
            }
            Self::InfiniteConsumables => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffsets::InfiniteGoods).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffsets::InfiniteGoods, new_state).send_error();
            }
            Self::Hidden => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffsets::Hidden).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffsets::Hidden, new_state).send_error();
            }
            Self::Silent => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffsets::Silent).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffsets::Silent, new_state).send_error();
            }
            Self::InfiniteArrows => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffsets::InfiniteArrows).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffsets::InfiniteArrows, new_state).send_error();
            }
            Self::TorrentNoDeath => {
                let new_state = !GameState::state_flags().torrent_no_death;
                StateFlags::set(StateFlagOffset::TorrentNoDeath, new_state).send_error();
                let torrent_ins = torrent_ins();
                torrent_ins.set_no_death(!torrent_ins.is_no_death().unwrap_or_default()).ok();
            }
            Self::TorrentAnywhere => {
                let new_state = !player::is_torrent_anywhere().unwrap_or_default();
                player::set_torrent_anywhere(new_state).send_error();
            }
        }
    }
    fn to_list_item(&self, player_tab: &PlayerTab) -> ListItem<'_> {
        let text = match self {
            Self::NoDeath => {
                let state = player::is_chr_dbg_flag(ChrDbgOffsets::PlayerNoDeath).unwrap_or_default();
                "No Death".create_toggle_str(state)
            }
            Self::NoDamage => {
                let state = GameState::state_flags().player_no_damage;
                "No Damage".create_toggle_str(state)
            }
            Self::SetRfbsOnLoad => {
                let state = GameState::state_flags().rfbs;
                "Set RFBS on load".create_toggle_str(state)
            }
            Self::InfinitePoise => {
                let state = player::is_infinite_poise().unwrap_or_default();
                "Infinite Poise".create_toggle_str(state)
            }
            Self::OneShot => {
                let state = player::is_chr_dbg_flag(ChrDbgOffsets::OneShot).unwrap_or_default();
                "One Shot".create_toggle_str(state)
            }
            Self::RuneArc => {
                let state = player_tab.stats.rune_arc || GameState::state_flags().rune_arc;
                "Rune Arc".create_toggle_str(state)
            }
            Self::InfiniteStamina => {
                let state = player::is_chr_dbg_flag(ChrDbgOffsets::InfiniteStam).unwrap_or_default();
                "Infinite Stamina".create_toggle_str(state)
            }
            Self::InfiniteFp => {
                let state = player::is_chr_dbg_flag(ChrDbgOffsets::InfiniteFp).unwrap_or_default();
                "Infinite FP".create_toggle_str(state)
            }
            Self::InfiniteConsumables => {
                let state = player::is_chr_dbg_flag(ChrDbgOffsets::InfiniteGoods).unwrap_or_default();
                "Infinite Consumables".create_toggle_str(state)
            }
            Self::Silent => {
                let state = player::is_chr_dbg_flag(ChrDbgOffsets::Silent).unwrap_or_default();
                "Silent".create_toggle_str(state)
            }
            Self::Hidden => {
                let state = player::is_chr_dbg_flag(ChrDbgOffsets::Hidden).unwrap_or_default();
                "Hidden".create_toggle_str(state)
            }
            Self::InfiniteArrows => {
                let state = player::is_chr_dbg_flag(ChrDbgOffsets::InfiniteArrows).unwrap_or_default();
                "Infinite Arrows".create_toggle_str(state)
            }
            Self::TorrentNoDeath => {
                let state = GameState::state_flags().torrent_no_death;
                "Torrent No Death".create_toggle_str(state)
            }
            Self::TorrentAnywhere=> {
                let state = player::is_torrent_anywhere().unwrap_or_default();
                "Torrent Anywhere".create_toggle_str(state)
            }
        };
        ListItem::from(text)
    }
    const ARRAY: &[TogglesItems] = &[
        Self::NoDeath,
        Self::NoDamage,
        Self::InfinitePoise,
        Self::OneShot,
        Self::RuneArc,
        Self::SetRfbsOnLoad,
        Self::Silent,
        Self::Hidden,
        Self::InfiniteStamina,
        Self::InfiniteFp,
        Self::InfiniteConsumables,
        Self::InfiniteArrows,
        Self::TorrentAnywhere,
        Self::TorrentNoDeath,
    ];
    fn list(player_tab: &PlayerTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item(player_tab)).collect();
        tabs_list(items, None, &player_tab.tab, TOGGLES_IDX)
    }
}

impl Stats {
    fn to_list_item(&self, stats: &PlayerStats) -> ListItem<'_> {
        let text = match self {
            Self::Vigor => format!("{} Vigor", stats.vigor),
            Self::Mind => format!("{} Mind", stats.mind),
            Self::Endurance => format!("{} Endurance", stats.endurance),
            Self::Strength => format!("{} Strength", stats.strength),
            Self::Dexterity => format!("{} Dexterity", stats.dexterity),
            Self::Intelligence => format!("{} Intelligence", stats.intelligence),
            Self::Faith => format!("{} Faith", stats.faith),
            Self::Arcane => format!("{} Arcane", stats.arcane),
            Self::Scadutree => format!("{} Scadutree", stats.scadutree),
            Self::SpiritAsh => format!("{} Spirit Ash", stats.spirit_ash),
            Self::RuneLevel => format!("{} Rune Memory", stats.rune_memory),
            Self::RuneMem => format!("{} Rune Memory", stats.rune_memory),
        };
        ListItem::from(text)
    }
    fn set_input(&self, input: &mut InputPrompt<InputRequest>) {
        input.show("Set Stat", PromptType::I32, InputRequest::Stat)
    }

    pub fn set_stat(&self, val: i32) -> Result<()> {
        match self {
            Self::Vigor => player::set_stat(PlayerGameDataOffset::Vigor.val(), val),
            Self::Mind => player::set_stat(PlayerGameDataOffset::Mind.val(), val),
            Self::Endurance => player::set_stat(PlayerGameDataOffset::Endurance.val(), val),
            Self::Strength => player::set_stat(PlayerGameDataOffset::Strength.val(), val),
            Self::Dexterity => player::set_stat(PlayerGameDataOffset::Dexterity.val(), val),
            Self::Intelligence => player::set_stat(PlayerGameDataOffset::Intelligence.val(), val),
            Self::Faith => player::set_stat(PlayerGameDataOffset::Faith.val(), val),
            Self::Arcane => player::set_stat(PlayerGameDataOffset::Arcane.val(), val),
            Self::Scadutree => player::set_dlc_stat(PlayerGameDataOffset::Scadutree.val(), val as u8),
            Self::SpiritAsh => player::set_dlc_stat(PlayerGameDataOffset::SpiritAsh.val(), val as u8),
            _ => Ok(()),
        }
    }
    fn increment_stat(&self, stats: &PlayerStats, val: i32) -> Result<()> {
        match self {
            Self::Vigor => self.set_stat(stats.vigor + val),
            Self::Mind => self.set_stat(stats.mind + val),
            Self::Endurance => self.set_stat(stats.endurance + val),
            Self::Strength => self.set_stat(stats.strength + val),
            Self::Dexterity => self.set_stat(stats.dexterity + val),
            Self::Intelligence => self.set_stat(stats.intelligence + val),
            Self::Faith => self.set_stat(stats.faith + val),
            Self::Arcane => self.set_stat(stats.arcane + val),
            Self::Scadutree => self.set_stat(stats.scadutree as i32 + val),
            Self::SpiritAsh => self.set_stat(stats.spirit_ash as i32 + val),
            _ => Ok(()),
        }
    }
    const ARRAY: &[Stats] = &[
        Self::Vigor,
        Self::Mind,
        Self::Endurance,
        Self::Strength,
        Self::Dexterity,
        Self::Intelligence,
        Self::Faith,
        Self::Arcane,
        Self::Scadutree,
        Self::SpiritAsh,
    ];
    const NO_DLC_ARRAY: &[Stats] = &[
        Self::Vigor,
        Self::Mind,
        Self::Endurance,
        Self::Strength,
        Self::Dexterity,
        Self::Intelligence,
        Self::Faith,
        Self::Arcane,
    ];
    pub fn array() -> &'static [Stats] {
        if GameState::dlc() { Self::ARRAY } else { Self::NO_DLC_ARRAY }
    }
    fn list(player_tab: &PlayerTab) -> List<'static> {
        let array = Self::array();
        let items: Vec<ListItem> = array.iter().map(|i| i.to_list_item(&player_tab.stats)).collect();
        tabs_list(items, Some("Stats"), &player_tab.tab, STATS_IDX)
    }
}

#[derive(Clone, Copy)]
enum InputRequest {
    Health,
    Runes,
    AnimationSpeed,
    Stat,
}