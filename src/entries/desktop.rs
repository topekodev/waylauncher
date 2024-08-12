use std::{fs, env};
use std::process::{Command, Stdio, exit};
use ini::Ini;
use crate::entries::{LauncherEntry, LauncherAction, Action};

#[derive(Debug, Clone)]
pub struct DesktopEntry {
    pub exec: String,
    pub terminal: String
}

impl DesktopEntry {
    pub fn new(exec: String, terminal: String) -> Self {
        DesktopEntry { exec, terminal }
    }
}

impl Action for DesktopEntry {
    fn launch(&self, terminal_cmd: String) {
        // Handle GUI and terminal apps
        if self.terminal != "true" {
            Command::new("sh")
                .arg("-c")
                .arg(&self.exec)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("Failed to launch");
        } else {
            Command::new(terminal_cmd)
                .arg("--")
                .arg(&self.exec)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("Failed to launch");
        }
        exit(0);
    }
}

pub fn get_desktop_entries() -> Vec<LauncherEntry> {
    let mut desktop_entries: Vec<LauncherEntry> = Vec::new();

    // Get application desktop files from all directories included in XDG_DATA_DIRS
    if let Some(xdg_data_dirs) = env::var_os("XDG_DATA_DIRS") {
        let paths: Vec<_> = env::split_paths(&xdg_data_dirs)
            .map(|path| path.join("applications"))
            .filter(|path| {
                if let Ok(metadata) = fs::metadata(path) {
                    metadata.is_dir()
                } else {
                    false
                }
            })
            .collect();

        for path in paths {
            let files = fs::read_dir(path)
                .unwrap()
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|path| path.extension().map_or(false, |ext| ext == "desktop"))
                .collect::<Vec<_>>();

            for file in files {
                let desktop = Ini::load_from_file(&file).unwrap();
                let entry = desktop.section(Some("Desktop Entry")).unwrap();
                if let Some(no_display) = entry.get("NoDisplay") {
                    if no_display == "true" {
                        continue;
                    }
                }
                
                // Get required information from desktop file
                let name = entry.get("Name").unwrap_or("Unknown");
                let generic_name = entry.get("GenericName").unwrap_or("");
                let mut keywords = entry.get("Keywords").unwrap_or("")
                    .replace(";", " ")
                    .to_string();
                let categories = entry.get("Categories").unwrap_or("")
                    .replace(";", " ")
                    .to_string();
                let exec = entry.get("Exec").unwrap_or("").split_whitespace()
                    .filter(|arg| !arg.contains('%'))
                    .collect::<Vec<&str>>()
                    .join(" ");
                let terminal = entry.get("Terminal").unwrap_or("false");
                keywords.push_str(name);
                keywords.push_str(generic_name);
                keywords.push_str(&categories);

                // Create entry for application
                let application = LauncherEntry::new(
                    name.to_string(),
                    keywords,
                    LauncherAction::DesktopEntry(
                        DesktopEntry::new(exec, terminal.to_string())
                    )
                );
                desktop_entries.push(application);
            }
        }
    } else {
        panic!("XDG_DATA_DIRS not set");
    }

    desktop_entries
}
