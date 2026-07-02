use crate::{
    app::CurrentScreen,
    common::{ListExt, block, centered_rect, controls::draw_controls},
    event::{Event, ResultExt, send_event},
    theme::{self, theme},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use gubtool_core::attached::{self, ProcessManager};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, TableState, Wrap},
};

const CONTROLS: &[(&str, &str)] = &[
    ("ctrl-k", "Kill"),
    ("Enter", "Attach"),
];

pub struct ProcessSelector {
    pub manager: ProcessManager,
    pub table: TableState,
}

impl ProcessSelector {
    pub fn new() -> Self {
        Self {
            manager: ProcessManager::new(),
            table: TableState::default().with_selected(0),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        self.manager.refresh();

        let layout = centered_rect(75, 75, frame.area());
        let block = block(Some("Process Selector"), None);
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
            self.table(),
            processes_area,
            &mut self.table
        );
        frame.render_widget(
            self.path_paragraph(),
            path_area
        );
        draw_controls(frame, layout, CONTROLS);
    }

    fn path_paragraph(&self) -> Paragraph<'static> {
        let text = {
            let processes = self.manager.valid_processes();
            if let Some(idx) = self.table.selected() && idx < processes.len() {
                format!("{}", processes[idx].exe_path.display())
            } else {
                "".to_string()
            }
        };
        Paragraph::new(text).wrap(Wrap { trim: true })
            .block(Block::new().borders(Borders::TOP))
    }

    fn table(&self) -> Table<'static> {
        let mut rows: Vec<Row> = Vec::new();
        for process in self.manager.valid_processes() {
            let comm = if attached::pid() == Some(process.pid) {
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
            Constraint::Min(28),
            Constraint::Max(10),
            Constraint::Fill(1),
        ];
        Table::new(rows, widths)
            .header(header)
            .highlight_symbol(theme::HIGHLIGHT_SYMBOL)
            .row_highlight_style(Style::from(theme().accent).bold())
    }

    pub fn handle_keys(&mut self, key: KeyEvent, current_screen: &mut CurrentScreen) {
        self.table.handle_keys(key);

        match (key.code, key.modifiers) {
            (KeyCode::Char('q') | KeyCode::Esc, _) => *current_screen = CurrentScreen::Main,
            (KeyCode::Enter, _) => {
                let processes = self.manager.valid_processes();
                if let Some(idx) = self.table.selected() && idx < processes.len() {
                    processes[idx].attach().send_error();
                    send_event(Event::Attach);
                }
            }
            (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                let processes = self.manager.valid_processes();
                if let Some(idx) = self.table.selected() && idx < processes.len() {
                    processes[idx].kill();
                }
            }
            _ => (),
        }
    }
}