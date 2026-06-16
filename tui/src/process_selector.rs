use crate::{
    app::CurrentScreen,
    common::{ListExt, block, centered_rect, controls::draw_controls},
    event::ResultExt,
    theme::{self, theme},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use gubtool_core::attached::{self, GameProcess};
use nix::{
    sys::signal::{self, Signal},
    unistd::Pid,
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, TableState, Wrap},
};

const CONTROLS: &[(&str, &str)] = &[
    ("ctrl-k", "Kill"),
    ("Enter", "Attach"),
    ("q", "Exit"),
];

pub struct ProcessSelector {
    pub table: TableState,
    available_processes: Vec<GameProcess>,
}

impl ProcessSelector {
    pub fn new() -> Self {
        Self {
            table: TableState::default().with_selected(0),
            available_processes: Vec::new(),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        self.update_processes();

        let layout = centered_rect(75, 75, frame.area());
        let block = block(Some("Valid Processes"), None);
        frame.render_widget(Clear, layout);
        frame.render_widget(&block, layout);

        let [processes_area, path_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Length(4),
            ])
            .areas(block.inner(layout));

        frame.render_stateful_widget(
            Self::table(&self.available_processes),
            processes_area,
            &mut self.table
        );
        frame.render_widget(
            Self::path_paragraph(&self.available_processes, self.table.selected()),
            path_area
        );
        draw_controls(frame, layout, CONTROLS);
    }

    fn path_paragraph(processes: &[GameProcess], selected: Option<usize>) -> Paragraph<'static> {
        let text = {
            if let Some(idx) = selected && idx < processes.len() {
                format!("{}", processes[idx].exe_path.display())
            } else {
                "".to_string()
            }
        };
        Paragraph::new(text).wrap(Wrap { trim: true })
            .block(Block::new().borders(Borders::TOP))
    }

    fn table(processes: &[GameProcess]) -> Table<'static> {
        let mut rows: Vec<Row> = Vec::new();
        for process in processes {
            let comm = if attached::pid() == process.pid {
                    format!("*{}", process.comm)
                } else {
                    format!(" {}", process.comm)
                };
            let row = Row::new(vec![
                Cell::from(comm),
                Cell::from(process.pid.to_string()),
                Cell::from(format!("{}", process.game_version)),
            ]);
            rows.push(row);
        }
        let header = Row::new(vec![
            Cell::from("Name"),
            Cell::from("PID"),
            Cell::from("Game Version"),
        ]).bold();
        let widths = [
            Constraint::Max(20),
            Constraint::Max(10),
            Constraint::Fill(1),
        ];
        Table::new(rows, widths)
            .header(header)
            .highlight_symbol(theme::HIGHLIGHT_SYMBOL)
            .row_highlight_style(Style::from(theme().accent).bold())
    }

    pub fn handle_keys(&mut self, key: KeyEvent, current_screen: &mut CurrentScreen) -> Option<GameProcess> {
        match (key.code, key.modifiers) {
            (KeyCode::Char('q') | KeyCode::Esc, _) => *current_screen = CurrentScreen::Main,
            (KeyCode::Enter, _) => {
                if let Some(selected) = self.table.selected() {
                    let mut processes = attached::get_processes();
                    if selected < processes.len() {
                        return Some(processes.remove(selected))
                    }
                }
            }
            (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                if let Some(selected) = self.table.selected() {
                    Self::kill_process(self.available_processes[selected].pid).send_error();
                    return None;
                }
            }
            _ => (),
        }
        self.table.handle_keys(key);
        None
    }

    fn kill_process(pid: Pid) -> Result<(), std::io::Error> {
        Ok(signal::kill(pid, Signal::SIGKILL)?)
    }

    pub fn update_processes(&mut self)  {
        self.available_processes = attached::get_processes()
    }
}