use crate::{
    event::KeyContext,
    panes::{TableController, TablePane, TableView},
    popup::{Popup, PopupState, centered_popup},
    screen::Screen,
    ui_state::UiState,
};
use config::Config;
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{BorderType, Row},
};
use ratatui_themes::{ThemeName, ThemePalette};
use std::sync::RwLock;

pub const HIGHLIGHT_SYMBOL: &'static str = "> ";
pub const BORDER_TYPE: BorderType = BorderType::Rounded;

static GLOBAL_THEME: RwLock<GlobalTheme> = RwLock::new(GlobalTheme::new());

struct GlobalTheme {
    palette: ThemePalette,
    version: usize,
}

impl GlobalTheme {
    const fn new() -> Self {
        Self { palette: ThemeName::TokyoNight.palette(), version: 0 }
    }
}

pub fn theme() -> ThemePalette {
    GLOBAL_THEME.read().unwrap().palette
}

pub fn set_theme(theme: ThemeName) {
    let mut global_theme = GLOBAL_THEME.write().unwrap();
    global_theme.palette = theme.palette();
    global_theme.version = global_theme.version.wrapping_add(1);
}

pub fn get_theme_version() -> usize {
    GLOBAL_THEME.read().unwrap().version
}

pub struct ThemeSelector {
    list: TablePane,
    popup: PopupState,
}

impl Popup for ThemeSelector {
    fn screen(&mut self) -> &mut dyn Screen {
        &mut self.list
    }
    fn popup_state(&mut self) -> &mut PopupState {
        &mut self.popup
    }
    fn popup_rect(&self, frame: &mut Frame) -> Rect {
        centered_popup(60, 60, frame.area())
    }
}

struct ThemeList;
impl TableController for ThemeList {
    fn make_table_view(&self) -> TableView {
        let selected_theme = theme();
        let rows = ThemeName::all().iter()
            .map(|theme| {
                let name = if selected_theme == theme.palette() {
                    format!("*{}", theme.display_name())
                } else {
                    format!(" {}", theme.display_name())
                };
                Row::new([name])
            })
            .collect();
        TableView::new(rows)
    }
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
        if ctx.key_enter() {
            let theme = ThemeName::all()[selected];
            UiState::update(|c| { c.global.theme = theme; }).ok();
            set_theme(theme);
        }
    }
}

impl ThemeSelector {
    pub fn new() -> Self {
        Self {
            list: TablePane::new_static(&ThemeList)
                .with_title("Themes"),
            popup: PopupState::default(),
        }
    }
}