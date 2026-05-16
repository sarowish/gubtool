mod event_tab;
mod items_tab;
mod player_tab;
mod travel_tab;
mod utility_tab;

use crate::{
    config::{Config, user::AttachConfig},
    ds2::{
        game_state::{GameStateHandler, StateFlags},
        target,
    },
    tui::{
        common::tabs_widget::TabsWidget,
        ds2::{
            event_tab::EventTab, items_tab::ItemTab, player_tab::PlayerTab, travel_tab::TravelTab, utility_tab::UtilityTab
        },
        event::ResultExt,
    },
};
use anyhow::Result;
use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};

static mut GAME_INFO: GameInfo = {
    GameInfo {
        is_loaded: false,
        state_flags: StateFlags::const_default(),
    }
};

pub struct DarkSouls2 {
    tabs_widget: TabsWidget,
    game_state: GameStateHandler,
    player: PlayerTab,
    utility: UtilityTab,
    items: ItemTab,
    travel: TravelTab,
    event: EventTab,
}

struct GameInfo {
    is_loaded: bool,
    state_flags: StateFlags,
}

impl DarkSouls2 {
    pub fn new() -> Self {
        Self {
            tabs_widget: TabsWidget {
                current_tab: 0,
                title: "Dark Souls II",
                tabs: &["Player", "Target", "Utility", "Items", "Travel", "Events"],
            },
            game_state: GameStateHandler::new(),
            player: PlayerTab::new(),
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
            "Target" => (),
            "Utility" => self.utility.draw(frame, layout),
            "Items" => self.items.draw(frame, layout),
            "Travel" => self.travel.draw(frame, layout),
            "Events" => self.event.draw(frame, layout),
            _ => (),
        }
    }

    pub fn handle_keys(&mut self, key: KeyEvent) {
        self.tabs_widget.handle_keys(key);

        match self.tabs_widget.tabs[self.tabs_widget.current_tab as usize] {
            "Player" => self.player.handle_keys(key),
            "Target" => (),
            "Utility" => self.utility.handle_keys(key),
            "Items" => self.items.handle_keys(key),
            "Travel" => self.travel.handle_keys(key),
            "Events" => self.event.handle_keys(key),
            _ => (),
        }
    }

    pub fn background_tick(&mut self) {
        self.game_state.poll().send_error();
        unsafe { GAME_INFO.is_loaded = self.game_state.loaded };
    }

    pub fn render_tick(&self) {
        unsafe {
            let game_info_ptr: *mut GameInfo = &raw mut GAME_INFO;
            (*game_info_ptr).state_flags.update().send_error();
        }
    }

    pub fn on_unattach(&self) {
    }

    pub fn on_attach(&self) -> Result<()> {
        target::install_target_hook().send_error();
        if let Ok(config) = AttachConfig::read() {
            config.dark_souls_2.apply()?;
        }
        Ok(())
    }
}

fn is_character_loaded() ->  bool {
    unsafe { GAME_INFO.is_loaded }
}

fn state_flags() ->  StateFlags {
    unsafe { GAME_INFO.state_flags }
}