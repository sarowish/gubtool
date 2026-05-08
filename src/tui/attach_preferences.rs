use crate::tui::common::{stateful_list::StatefulList, tabs_widget::TabsWidget};

pub struct AttachPreferences {
    tabs: TabsWidget,
    list_state: StatefulList,
}