use gtk::{glib, gdk, gio, prelude::*};
use gdk::{Key, ModifierType};
use crate::entries::LauncherEntry;
use crate::config::Config;

pub fn handle_key(config: Config, key: Key, modifiers: ModifierType, results: Option<&Vec<LauncherEntry>>, list_box: &gtk::ListBox) -> glib::Propagation {
    // Get currently selected item index from list box
    let selection_index = list_box.selected_row().map(|row| row.index()).unwrap_or(-1);
  
    // Handle moving between items
    let handle_next = || {
        let next_index = selection_index + 1;
        if let Some(_) = list_box.row_at_index(next_index) {
            list_box.select_row(list_box.row_at_index(next_index).as_ref());
        }
    };
    let handle_previous = || {
        if selection_index > 0 {
            list_box.select_row(list_box.row_at_index(selection_index - 1).as_ref());
        }
    };
    // Handle interacting with selected item
    let handle_action = || {
        if let Some(results) = results {
            if selection_index >= 0 {
                let selected_entry = &results[selection_index as usize];
                selected_entry.launch(config.terminal);
            }
        }
    };

    // Fomat key input
    let mut input_string = key.name().unwrap().to_string();
    if modifiers.contains(ModifierType::CONTROL_MASK) {
        input_string = format!("C-{}", input_string);
    }
    if modifiers.contains(ModifierType::ALT_MASK) {
        input_string = format!("A-{}", input_string);
    }
    if modifiers.contains(ModifierType::SHIFT_MASK) {
        input_string = format!("S-{}", input_string);
    }

    // Check if keybind
    let input_match = |keys: &Vec<String>| -> bool {
        keys.iter().any(|k| k == &input_string)
    };

    if input_match(&config.keys.exit) {
        std::process::exit(0);
    } else if input_match(&config.keys.action) {
        handle_action();
    } else if input_match(&config.keys.next) {
        handle_next();
        glib::Propagation::Stop;
    } else if input_match(&config.keys.previous) {
        handle_previous();
        glib::Propagation::Stop;
    }
    glib::Propagation::Proceed
}
