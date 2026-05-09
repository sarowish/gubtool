use crate::{
    config::{Config, ui_state::UiState},
    core::{
        attach::{self, ATTACHED_PROCESS, Game, GameProcess, game, module_handle, pid, version},
        sys,
    },
    tui::{
        attach_options::AttachOptions,
        ds2::DarkSouls2,
        er::EldenRing,
        event::{Event, ResultExt, send_event, start_event_loop_thread},
        fuzzy_finder::FuzzyFinder,
        game_screen_selector::GameScreenSelector,
        help,
        input::Input,
        process_selector::ProcessSelector,
        theme::{THEME, ThemeSelector, theme},
    },
};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Clear, Paragraph},
};
use ratatui_themes::ThemeName;
use std::{sync::RwLock, thread, time::Duration};

pub struct App {
    running: bool,
    current_screen: CurrentScreen,
    pub game_screen: Game,
    attached: bool,
    show_err: bool,
    show_input: bool,
    err_message: String,

    pub theme: ThemeName,
    theme_selector: ThemeSelector,
    input: Input,
    input_enter_fn: fn(String, &mut App),
    pub fuzzy_finder: FuzzyFinder,
    fuzzy_picker: fn(&mut App),
    process_selector: ProcessSelector,
    game_screen_selector: GameScreenSelector,
    attach_options: AttachOptions,

    pub elden_ring: EldenRing,
    pub dark_souls_2: DarkSouls2,
}

impl App {
    pub fn new() -> App {
        App {
            running: true,
            game_screen: Game::EldenRing,
            current_screen: CurrentScreen::Game,
            attached: false,
            show_err: false,
            show_input: false,
            err_message: "".to_string(),

            theme: ThemeName::default(),
            theme_selector: ThemeSelector::new(),
            input: Input::default(),
            input_enter_fn: |_,_| {},
            fuzzy_finder: FuzzyFinder::default(),
            fuzzy_picker: |_| {},
            process_selector: ProcessSelector::new(),
            game_screen_selector: GameScreenSelector::new(),
            attach_options: AttachOptions::new(),

            elden_ring: EldenRing::new(),
            dark_souls_2: DarkSouls2::new(),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        UiState::apply(&mut self);
        THEME.set(RwLock::new(self.theme.palette())).unwrap();
        let rx = start_event_loop_thread();

        self.try_attach(None).send_error();

        while self.running {
            terminal.draw(|frame| Self::draw(&mut self, frame))?;

            match rx.recv()? {
                Event::Key(key) => {
                    Self::handle_keys(&mut self, key)
                }
                Event::Error(err) => {
                    self.err_message = err;
                    self.show_err = true;
                }
                Event::BackgroundTick => {
                    if !self.attached {
                        self.try_attach(None).send_error()
                    } else if !attach::is_pid_valid() {
                        self.detach()
                    }
                    if self.attached && game() == self.game_screen {
                        match self.game_screen {
                            Game::EldenRing => self.elden_ring.background_tick(),
                            Game::DarkSoulsII => self.dark_souls_2.background_tick(),
                        }
                    }
                }
                Event::RenderTick => {
                    if self.attached && game() == self.game_screen {
                        match self.game_screen {
                            Game::EldenRing => self.elden_ring.render_tick(),
                            Game::DarkSoulsII => self.dark_souls_2.render_tick(),
                        }
                    }
                }
                Event::Search((list, f)) => {
                    self.fuzzy_finder.entries = Some(list);
                    self.fuzzy_finder.update_matches();
                    self.fuzzy_picker = f;
                    self.current_screen = CurrentScreen::Search
                }
                Event::Input(f) => {
                    self.input_enter_fn = f;
                    self.show_input = true;
                }
                Event::ApplyAttach => {
                    match game() {
                        Game::EldenRing => self.elden_ring.on_attach(),
                        Game::DarkSoulsII => self.dark_souls_2.on_attach(),
                    }.send_error()
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let background = Block::default().bg(theme().bg);
        frame.render_widget(background, frame.area());

        let constraints = if self.show_err || self.show_input {
            vec![
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ]
        } else {
            vec![
                Constraint::Length(1),
                Constraint::Fill(1),
            ]
        };

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(frame.area());

        let [pid_area, version_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Max(25),
                Constraint::Fill(1)
            ])
            .areas(layout[0]);

        frame.render_widget(self.pid_paragraph(), pid_area);
        frame.render_widget(self.version_paragraph(), version_area);

        if self.show_err {
            let err_paragraph = Paragraph::new(self.err_message.to_string()).style(theme().error);
            frame.render_widget(err_paragraph, layout[2]);
        } else if self.show_input {
            let input = Paragraph::new(self.input.to_string()).style(theme().fg);
            self.input.set_cursor(frame, layout[2]);
            frame.render_widget(input, layout[2]);
        }

        match self.game_screen {
            Game::EldenRing => self.elden_ring.draw(frame, layout[1]),
            Game::DarkSoulsII => self.dark_souls_2.draw(frame, layout[1]),
        }

        match self.current_screen {
            CurrentScreen::Search => {
                self.fuzzy_finder.draw(frame)
            }
            CurrentScreen::ProcessSelection => {
                self.process_selector.draw(frame)
            }
            CurrentScreen::ThemeSelection => {
                self.theme_selector.draw(frame, &self.theme)
            }
            CurrentScreen::GameScreenSelection => {
                self.game_screen_selector.draw(frame)
            }
            CurrentScreen::AttachOptions => {
                self.attach_options.draw(frame, &self.game_screen)
            }
            CurrentScreen::Help => {
                help::draw(frame)
            }
            CurrentScreen::Debug => {
                frame.render_widget(Clear, frame.area());
                frame.render_widget(dbg_paragraph(), frame.area());
            }
            _ => (),
        }
    }

    fn handle_keys(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char('c') &&
        key.modifiers == KeyModifiers::CONTROL {
            self.running = false
        }
        if self.show_err {
            self.show_err = false;
        }
        if self.show_input {
            match key.code {
                KeyCode::Enter => {
                    let text = self.input.text.clone();
                    (self.input_enter_fn)(text, self);
                    self.input.set_text("");
                    self.show_input = false
                }
                KeyCode::Esc => {
                    self.input.set_text("");
                    self.show_input = false
                }
                _ => {
                    self.input.handle_keys(key);
                }
            }
            return;
        }
        match self.current_screen {
            CurrentScreen::ProcessSelection => {
                if let Some(process) = self.process_selector.handle_keys(key, &mut self.current_screen) {
                    self.try_attach(Some(process)).send_error()
                }
            },
            CurrentScreen::GameScreenSelection => {
                self.game_screen_selector.handle_keys(key, &mut self.game_screen, &mut self.current_screen)
            },
            CurrentScreen::ThemeSelection => {
                self.theme_selector.handle_keys(key, &mut self.theme, &mut self.current_screen)
            },
            CurrentScreen::AttachOptions => {
                self.attach_options.handle_keys(key, &self.game_screen, &mut self.current_screen)
            },
            CurrentScreen::Help | CurrentScreen::Debug => {
                self.current_screen = CurrentScreen::Game
            },
            CurrentScreen::Search => {
                match key.code {
                    KeyCode::Enter => {
                        (self.fuzzy_picker)(self);
                        self.fuzzy_finder.reset();
                        self.current_screen = CurrentScreen::Game;
                    }
                    KeyCode::Esc => {
                        self.fuzzy_finder.reset();
                        self.current_screen = CurrentScreen::Game;
                    }
                    _ => {
                        self.fuzzy_finder.handle_keys(key)
                    }
                }
            }
            CurrentScreen::Game => {
                match self.game_screen {
                    Game::EldenRing => self.elden_ring.handle_keys(key),
                    Game::DarkSoulsII => self.dark_souls_2.handle_keys(key),
                }
            }
        }
        match (key.code, key.modifiers) {
            (KeyCode::Char('a'), _) => self.current_screen = CurrentScreen::AttachOptions,
            (KeyCode::F(1), _) => self.current_screen = CurrentScreen::Help,
            (KeyCode::Char('p'), _) => self.current_screen = {
                self.process_selector.update_processes();
                CurrentScreen::ProcessSelection
            },
            (KeyCode::Char('o'), _) => self.current_screen = CurrentScreen::GameScreenSelection,
            (KeyCode::F(12), KeyModifiers::CONTROL) => self.current_screen = CurrentScreen::Debug,
            (KeyCode::F(12), _) => self.current_screen = CurrentScreen::ThemeSelection,
            _ => ()
        }
    }

    fn try_attach(&mut self, process: Option<GameProcess>) -> anyhow::Result<()> {
        let mut result = Ok(());
        if let Some(process) = process {
            if let Err(err) = attach::attach_to_process(process) {
                result = Err(err)
            }
        } else {
            match attach::auto_attach() {
                Ok(val) => if !val { return Ok(()) },
                Err(err) => result = Err(err),
            }
        }

        self.attached = true;
        self.game_screen = game();
        let _ = UiState::update(|c| c.global.game_screen = game() );

        let time_to_wait = 6.0 - sys::get_process_uptime(pid()).unwrap_or_default();
        if time_to_wait > 0.0 {
            thread::spawn(move || {
                thread::sleep(Duration::from_secs_f64(time_to_wait));
                send_event(Event::ApplyAttach);
            });
        } else {
            send_event(Event::ApplyAttach);
        }
        result
    }

    fn detach(&mut self) {
        unsafe {
            ATTACHED_PROCESS = GameProcess::detached()
        }
        match game() {
            Game::EldenRing => self.elden_ring.on_unattach(),
            Game::DarkSoulsII => self.dark_souls_2.on_unattach(),
        }
        self.attached = false;
    }

    fn pid_paragraph(&self) -> Paragraph<'static> {
        if self.attached {
            Paragraph::new(format!("Process ID: {}", pid()))
        } else {
            Paragraph::new("Scanning for game...")
        }.style(theme().fg)
    }
    fn version_paragraph(&self) -> Paragraph<'static> {
        if self.attached {
            Paragraph::new(format!("{}", version()))
        } else {
            Paragraph::new("")
        } .style(theme().fg)
            .alignment(Alignment::Right)
    }
}

#[derive(PartialEq)]
pub enum CurrentScreen {
    Game,
    Search,
    Help,
    ProcessSelection,
    ThemeSelection,
    GameScreenSelection,
    AttachOptions,
    Debug,
}

fn dbg_paragraph() -> Paragraph<'static> {

    let debug_info = [
        format!("Module Handle: {:#X}", module_handle()),
        format!("Process Uptime: {:.1}", sys::get_process_uptime(pid()).unwrap_or_default()),
    ];

    let lines: Vec<Line> = debug_info.iter().map(|f| Line::raw(f.to_string())).collect();
    Paragraph::new(Text::from(lines))
}