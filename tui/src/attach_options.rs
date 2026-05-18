use crate::{
    app::CurrentScreen,
    common::{StrExt, centered_rect, list, stateful_list::StatefulList},
    event::ResultExt,
    send_input_event,
};
use config::{
    Config,
    user::{AttachConfig, ds2_attach::Ds2Attach, er_attach::ErAttach},
};
use crossterm::event::{KeyCode, KeyEvent};
use engine::Game;
use ratatui::{
    Frame,
    widgets::{Clear, List, ListItem},
};

pub struct AttachOptions {
    ds2_list: StatefulList,
    er_list: StatefulList,
    attach_config: AttachConfig,
}

impl AttachOptions {
    pub fn new() -> Self {
        Self {
            ds2_list: StatefulList::new(Ds2Options::ARRAY.len()),
            er_list: StatefulList::new(ErOptions::ARRAY.len()),
            attach_config: AttachConfig::read().unwrap_or_default(),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, game_screen: &Game) {
        self.attach_config = AttachConfig::read().unwrap_or_default();

        let layout = centered_rect(75, 75, frame.area());
        frame.render_widget(Clear, layout);

        match game_screen {
            Game::DarkSoulsII => {
                frame.render_stateful_widget(
                    Ds2Options::list(&self.attach_config.dark_souls_2),
                    layout,
                    &mut self.ds2_list.state
                );
            }
            Game::EldenRing => {
                frame.render_stateful_widget(
                    ErOptions::list(&self.attach_config.elden_ring),
                    layout,
                    &mut self.er_list.state
                );
            }
        }
    }

    pub fn handle_keys(&mut self, key: KeyEvent, game_screen: &Game, current_screen: &mut CurrentScreen) {
        match game_screen {
            Game::DarkSoulsII => {
                self.ds2_list.handle_keys(key);
            }
            Game::EldenRing => {
                self.er_list.handle_keys(key);
            }
        }
        match (key.code, key.modifiers) {
            (KeyCode::Char('q') | KeyCode::Esc, _) => *current_screen = CurrentScreen::Game,
            (KeyCode::Enter, _) => {
                match game_screen {
                    Game::DarkSoulsII => {
                        if let Some(selected) = self.ds2_list.selected() {
                            Ds2Options::ARRAY[selected].execute();
                        }
                    }
                    Game::EldenRing => {
                        if let Some(selected) = self.er_list.selected() {
                            ErOptions::ARRAY[selected].execute();
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

enum Ds2Options {
    NoDeath,
    GauntletSkip,
    DisableLoyce,
    SkipCredits,
    FastQuitout,
    StartEventLogger,
}

enum ErOptions {
    FpsCap,
    NoDeath,
    NoDamage,
    SetRfbsOnLoad,
    InfinitePoise,
    MuteMusic,
    RemoveLogo,
    DisableAreaTitleCards,
    StutterFix,
    MapAnywhere,
    TravelAnywhere,
}

impl Ds2Options {
    fn execute(&self) {
        match self {
            Self::NoDeath => {
                Ds2Attach::update(|c| c.no_death = !c.no_death).send_error()
            }
            Self::SkipCredits => {
                Ds2Attach::update(|c| c.skip_credits = !c.skip_credits).send_error()
            }
            Self::GauntletSkip => {
                Ds2Attach::update(|c| c.gauntlet_skip = !c.gauntlet_skip).send_error()
            }
            Self::DisableLoyce => {
                Ds2Attach::update(|c| c.disable_loyce = !c.disable_loyce).send_error()
            }
            Self::FastQuitout => {
                Ds2Attach::update(|c| c.fast_quitout = !c.fast_quitout).send_error()
            }
            Self::StartEventLogger => {
                Ds2Attach::update(|c| c.start_logger = !c.start_logger).send_error()
            }
        }
    }
    fn to_list_item(&self, options: &Ds2Attach) -> ListItem<'_> {
        let text = match self {
            Self::NoDeath => {
                "No Death".create_toggle_str(options.no_death)
            }
            Self::GauntletSkip => {
                "Skip Ivory King Gauntlet".create_toggle_str(options.gauntlet_skip)
            }
            Self::DisableLoyce => {
                "Disable Loyce Knights".create_toggle_str(options.disable_loyce)
            }
            Self::FastQuitout => {
                "Fast Quitout".create_toggle_str(options.fast_quitout)
            }
            Self::SkipCredits => {
                "Skip Credits".create_toggle_str(options.skip_credits)
            }
            Self::StartEventLogger => {
                "Start Event Logger".create_toggle_str(options.start_logger)
            }
        };
        ListItem::new(text)
    }
    const ARRAY: &[Ds2Options] = &[
        Self::NoDeath,
        Self::FastQuitout,
        Self::SkipCredits,
        Self::GauntletSkip,
        Self::DisableLoyce,
        Self::StartEventLogger,
    ];
    fn list(ds2: &Ds2Attach) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item(ds2)).collect();
        list(items, Some("Attach Options"))
    }
}

impl ErOptions {
    fn execute(&self) {
        match self {
            Self::FpsCap => {
                send_input_event!(text, _app, {
                    if let Ok(v) = text.parse() {
                        ErAttach::update(|c| {
                            c.fps = Some(v);
                        })
                        .send_error();
                    } else if text.is_empty() {
                        ErAttach::update(|c| {
                            c.fps = None;
                        })
                        .send_error();
                    }
                })
            }
            Self::NoDeath => {
                ErAttach::update(|c| c.no_death = !c.no_death).send_error()
            }
            Self::NoDamage => {
                ErAttach::update(|c| c.no_damage = !c.no_damage).send_error()
            }
            Self::SetRfbsOnLoad => {
                ErAttach::update(|c| c.rfbs_on_load = !c.rfbs_on_load).send_error()
            }
            Self::InfinitePoise => {
                ErAttach::update(|c| c.infinite_poise = !c.infinite_poise).send_error()
            }
            Self::MuteMusic => {
                ErAttach::update(|c| c.mute_music = !c.mute_music).send_error()
            }
            Self::RemoveLogo => {
                ErAttach::update(|c| c.remove_logo = !c.remove_logo).send_error()
            }
            Self::StutterFix => {
                ErAttach::update(|c| c.stutter_fix = !c.stutter_fix).send_error()
            }
            Self::DisableAreaTitleCards => {
                ErAttach::update(|c| c.disable_area_target_cards = !c.disable_area_target_cards).send_error()
            }
            Self::MapAnywhere => {
                ErAttach::update(|c| c.map_in_combat = !c.map_in_combat).send_error()
            }
            Self::TravelAnywhere => {
                ErAttach::update(|c| c.travel_in_dungeon = !c.travel_in_dungeon).send_error()
            }
        }
    }
    fn to_list_item(&self, options: &ErAttach) -> ListItem<'_> {
        let text = match self {
            Self::FpsCap => {
                format!("FPS Cap: {}", options.fps.map_or("".to_string(), |v| v.to_string()))
            }
            Self::NoDeath => {
                "No Death".create_toggle_str(options.no_death)
            }
            Self::NoDamage => {
                "No Damage".create_toggle_str(options.no_damage)
            }
            Self::SetRfbsOnLoad => {
                "Set RFBS on load".create_toggle_str(options.rfbs_on_load)
            }
            Self::InfinitePoise => {
                "Infinite Poise".create_toggle_str(options.infinite_poise)
            }
            Self::MuteMusic => {
                "Mute Music".create_toggle_str(options.mute_music)
            }
            Self::RemoveLogo => {
                "Remove Logos".create_toggle_str(options.remove_logo)
            }
            Self::StutterFix => {
                "Stutter Fix".create_toggle_str(options.stutter_fix)
            }
            Self::DisableAreaTitleCards => {
                "Disable Area Title Cards".create_toggle_str(options.disable_area_target_cards)
            }
            Self::MapAnywhere => {
                "Allow Map In Combat".create_toggle_str(options.map_in_combat)
            }
            Self::TravelAnywhere => {
                "Allow Travel In Dungeons".create_toggle_str(options.travel_in_dungeon)
            }
        };
        ListItem::new(text)
    }
    const ARRAY: &[ErOptions] = &[
        Self::FpsCap,
        Self::NoDeath,
        Self::NoDamage,
        Self::SetRfbsOnLoad,
        Self::InfinitePoise,
        Self::MuteMusic,
        Self::RemoveLogo,
        Self::DisableAreaTitleCards,
        Self::StutterFix,
        Self::MapAnywhere,
        Self::TravelAnywhere,
    ];
    fn list(er: &ErAttach) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item(er)).collect();
        list(items, Some("Attach Options"))
    }
}