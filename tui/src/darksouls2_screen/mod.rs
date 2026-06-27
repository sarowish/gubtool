mod event_tab;
mod items_tab;
mod player_tab;
mod target_tab;
mod travel_tab;
mod utility_tab;

use crate::{
    common::tabs_widget::TabsWidget,
    darksouls2_screen::{
        event_tab::EventTab, items_tab::ItemTab, player_tab::PlayerTab, target_tab::TargetTab,
        travel_tab::TravelTab, utility_tab::UtilityTab,
    },
};
use anyhow::Result;
use crossterm::event::KeyEvent;
use darksouls2::{
    bonfire,
    chr_ctrl::{ChrCtrl, ChrCtrlExt},
    game_state::{GameStateHandler, StateFlags},
    player, target, utility,
};
use ratatui::{Frame, layout::Rect};

pub struct DarkSouls2 {
    tabs_widget: TabsWidget,
    game_state: GameStateHandler,
    player: PlayerTab,
    target: TargetTab,
    utility: UtilityTab,
    items: ItemTab,
    travel: TravelTab,
    event: EventTab,
}

static mut GAME_STATE: GameState = {
    GameState {
        is_loaded: false,
        state_flags: StateFlags::const_default(),
        player_ctrl: Ok(0),
        target_ctrl: Ok(0),
    }
};

struct GameState {
    is_loaded: bool,
    state_flags: StateFlags,
    player_ctrl: ChrCtrl,
    target_ctrl: ChrCtrl,
}

impl GameState {
    pub fn loaded() -> bool {
        unsafe { GAME_STATE.is_loaded }
    }
    pub fn state_flags() ->  StateFlags {
        unsafe { GAME_STATE.state_flags }
    }
    pub fn target_ctrl() ->  &'static ChrCtrl {
        unsafe { &*std::ptr::addr_of!(GAME_STATE.target_ctrl) }
    }
    pub fn player_ctrl() ->  &'static ChrCtrl {
        unsafe { &*std::ptr::addr_of!(GAME_STATE.player_ctrl) }
    }
}

impl DarkSouls2 {
    pub fn new() -> Self {
        Self {
            tabs_widget: TabsWidget {
                current_tab: 0,
                title: Some("Dark Souls II"),
                tabs: &["Player", "Target", "Utility", "Items", "Travel", "Events"],
            },
            game_state: GameStateHandler::new(),
            player: PlayerTab::new(),
            target: TargetTab::new(),
            utility: UtilityTab::new(),
            items: ItemTab::new(),
            travel: TravelTab::new(),
            event: EventTab::new(),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect) {
        let layout = self.tabs_widget.draw(frame, layout);

        match self.tabs_widget.tabs[self.tabs_widget.current_tab as usize] {
            "Player" => self.player.draw(frame, layout),
            "Target" => self.target.draw(frame, layout),
            "Utility" => self.utility.draw(frame, layout),
            "Items" => self.items.draw(frame, layout),
            "Travel" => self.travel.draw(frame, layout),
            "Events" => self.event.draw(frame, layout),
            _ => (),
        }
    }

    pub fn handle_keys(&mut self, key: KeyEvent, block_inputs: bool) {
        match self.tabs_widget.tabs[self.tabs_widget.current_tab as usize] {
            "Player" => self.player.handle_keys(key),
            "Target" => self.target.handle_keys(key),
            "Utility" => self.utility.handle_keys(key),
            "Items" => self.items.handle_keys(key),
            "Travel" => self.travel.handle_keys(key),
            "Events" => self.event.handle_keys(key),
            _ => (),
        }

        if block_inputs { return; }

        self.tabs_widget.handle_keys(key);
    }

    pub fn background_tick(&mut self) {
        let _ = self.game_state.poll();
        unsafe {
            GAME_STATE.is_loaded = self.game_state.loaded;
            GAME_STATE.player_ctrl = player::player_ctrl();
        };
    }

    pub fn render_tick(&self) {
        unsafe {
            let game_state_ptr: *mut GameState = &raw mut GAME_STATE;
            let _ = (*game_state_ptr).state_flags.update();
        }
    }

    pub fn on_unattach(&self) {
    }

    pub fn on_attach(&self) -> Result<()> {
        target::install_target_hook()?;
        Ok(())
    }
}

pub fn dbg_lines() -> Vec<String> {
    vec![
        format!("area id: {:#X}", utility::get_area_id().unwrap_or_default()),
        format!("player coords: {:?}", player::player_position().unwrap_or_default()),
        format!("player quaternion: {:?}", player::player_ctrl().rot_quaternion().unwrap_or_default()),
        format!("last rested bonfire id: {}", bonfire::get_last_bonfire_id().unwrap_or_default()),
        format!("stats: {:?}", player::Stats::read()),
    ]
}