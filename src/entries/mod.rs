pub mod desktop;

pub trait Action {
    fn launch(&self, terminal_cmd: String);
}

#[derive(Debug, Clone)]
pub enum LauncherAction {
    DesktopEntry(desktop::DesktopEntry)
}

impl Action for LauncherAction {
    fn launch(&self, terminal_cmd: String) {
        match self {
            LauncherAction::DesktopEntry(entry) => entry.launch(terminal_cmd),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LauncherEntry {
    pub name: String,
    pub keywords: String,
    pub action: LauncherAction
}

impl LauncherEntry {
    pub fn new(name: String, keywords: String, action: LauncherAction) -> LauncherEntry {
        LauncherEntry {
            name,
            keywords,
            action
        }
    }
    pub fn launch(&self, terminal_cmd: String) {
        self.action.launch(terminal_cmd);
    }
}

pub fn get_entries() -> Vec<LauncherEntry> {
    let mut entries: Vec<LauncherEntry> = Vec::new();
    entries.extend(desktop::get_desktop_entries());
    entries
}
