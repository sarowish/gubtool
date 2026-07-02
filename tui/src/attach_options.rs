use crate::{
    app::CurrentScreen, common::{StrExt, centered_rect, list, stateful_list::StatefulList, tabs_widget::TabsWidget}, event::{ResultExt}, input::request_input, mutate_app, spawn_task,
};
use config::{
    Config,
    attach::{AttachConfig, AttachEntries, AttachEntry, AttachConfigManager},
};
use crossterm::event::{KeyCode, KeyEvent};
use gubtool_core::game_version::Game;
use ratatui::{
    Frame,
    widgets::{Clear, List, ListItem},
};

pub struct AttachOptions {
    pub manager: AttachConfigManager,
    game_tabs: GameTabs,
    list_state: StatefulList,
    list_identifier: usize,
}

struct GameTabs {
    ds2: TabsWidget,
    er: TabsWidget,
}

impl AttachOptions {
    pub fn new() -> Self {
        Self {
            manager: AttachConfigManager::new(),
            list_state: StatefulList::new(0),
            list_identifier: 0,
            game_tabs: GameTabs {
                ds2: TabsWidget {
                    current_tab: 0,
                    title: None,
                    tabs: &["Player", "Utility"],
                },
                er: TabsWidget {
                    current_tab: 0,
                    title: None,
                    tabs: &["Player", "Utility"],
                },
            }
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, game_screen: &Game) {
        self.manager.update();

        let layout = centered_rect(65, 65, frame.area());
        frame.render_widget(Clear, layout);

        let (list, id) = current_list(&self.game_tabs, &self.manager.entries, game_screen);
        let list_len = list.len();

        if self.list_identifier != id {
            self.list_identifier = id;
            self.list_state.select(0);
            self.list_state.size = list_len;
        }

        frame.render_stateful_widget(
            entries_to_list(&self.manager.config, list),
            layout,
            &mut self.list_state.state,
        );

        let tabs = match game_screen {
            Game::DarkSouls2 => &self.game_tabs.ds2,
            Game::EldenRing => &self.game_tabs.er,
        };
        tabs.draw_thin(frame, layout);
    }

    pub fn handle_keys(&mut self, key: KeyEvent, game_screen: &Game, current_screen: &mut CurrentScreen) {
        match game_screen {
            Game::DarkSouls2 => {
                self.game_tabs.ds2.handle_keys_arrows(key);
            }
            Game::EldenRing => {
                self.game_tabs.er.handle_keys_arrows(key);
            }
        }

        self.list_state.handle_keys(key);

        match (key.code, key.modifiers) {
            (KeyCode::Char('q') | KeyCode::Esc, _) => *current_screen = CurrentScreen::Main,
            (KeyCode::Enter, _) => {
                let Some(selected) = self.list_state.selected() else { return };
                let (list, _) = current_list(&self.game_tabs, &self.manager.entries, game_screen);

                match &list[selected] {
                    AttachEntry::Bool(val) => {
                        val.toggle(&mut self.manager.config);
                        self.manager.config.write().send_error();
                    }
                    AttachEntry::Float(_) => {
                        let game = game_screen.clone();
                        spawn_task! {
                            let new_val = request_input::<f32>(None).await;
                            mutate_app!(|app: &mut crate::app::App| {
                                let (list, _) = current_list(&app.attach_options.game_tabs, &app.attach_options.manager.entries, &game);
                                if let Some(AttachEntry::Float(target_val)) = list.get(selected) {
                                    target_val.set(&mut app.attach_options.manager.config, new_val);
                                    app.attach_options.manager.config.write().send_error();
                                }
                            });
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

fn current_list<'a>(tabs: &'a GameTabs, entries: &'a AttachEntries, game_screen: &'a Game) -> (&'a Vec<AttachEntry>, usize) {
    let list = match game_screen {
        Game::DarkSouls2 => {
            match tabs.ds2.current_tab() {
                "Player" => &entries.ds2.player,
                "Utility" => &entries.ds2.utility,
                _ => panic!("invalid tab"),
            }
        }
        Game::EldenRing => {
            match tabs.er.current_tab() {
                "Player" => &entries.er.player,
                "Utility" => &entries.er.utility,
                _ => panic!("invalid tab"),
            }
        }
    };
    let id = list.as_ptr() as usize;
    (list, id)
}

fn entries_to_list(config: &AttachConfig, entries: &Vec<AttachEntry>) -> List<'static> {
    let items: Vec<ListItem> = entries.iter().map(|x| {
        match x {
            AttachEntry::Bool(val) => {
                ListItem::from(format!("{x}").create_toggle_str(*val.get(config)))
            }
            AttachEntry::Float(val) => {
                let s = val.get(config).map_or(String::new(), |v| v.to_string());
                ListItem::from(format!("{}: {}", x, s))
            }
        }
    }).collect();
    list(items, None)
}