use gtk::{glib, gdk, gio, prelude::*};
use gtk4_layer_shell::{Layer, LayerShell, Edge, KeyboardMode};
use std::cell::RefCell;
use std::rc::Rc;

use crate::config;
use crate::entries;
use crate::keys;
use crate::search;

pub fn build_ui(app: &gtk::Application) {
    let builder = gtk::Builder::from_resource("/dev/topeko/waylauncher/window.ui");
   
    // Load config
    let app_config = config::get_config();

    // Build window
    let window = builder
        .object::<gtk::ApplicationWindow>("window")
        .expect("Could not get window");
    window.set_application(Some(app));
    window.set_decorated(false);
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(KeyboardMode::OnDemand);
    window.set_default_size(app_config.window.width, app_config.window.height);
    window.set_margin(Edge::Top, app_config.window.top);
    let anchors = [
        (Edge::Left, false),
        (Edge::Right, false),
        (Edge::Top, true),
        (Edge::Bottom, false),
    ];
    for (anchor, state) in anchors {
        window.set_anchor(anchor, state);
    }

    // GTK objects
    let input = builder.object::<gtk::Entry>("input").expect("Could not get input");
    let list_box = builder.object::<gtk::ListBox>("result_list").expect("Could not get ListBox");
    let scrolled_window = builder.object::<gtk::ScrolledWindow>("result_window").expect("Could not get ScrolledWindow");

    let input_buffer = input.buffer();

    // Get all entries
    let launcher_entries = entries::get_entries();
    // Vector for storing search results
    let search_results = Rc::new(RefCell::new(Vec::new()));

    // Handle entry input
    let search_results_clone = search_results.clone();
    let list_box_clone = list_box.clone();
    let scrolled_window_clone = scrolled_window.clone();
    input.connect_changed(move |_| {
        let search_query = &input_buffer.text();
        search_results_clone.replace(search::handle_search(search_query, &launcher_entries));

        // Update list box to show search results
        list_box_clone.remove_all();
        for result in search_results_clone.borrow().iter() {
            let label = gtk::Label::new(Some(&result.name));
            label.set_xalign(0.0);
            let row = gtk::ListBoxRow::new();
            row.set_child(Some(&label));
            row.set_can_focus(false);
            list_box_clone.append(&row);
        }

        if (search_query.is_empty()) {
            //TODO: Do not show results if empty
        } else {
            //Update selection
            list_box_clone.select_row(list_box_clone.row_at_index(0).as_ref());
        }
    });

    // Handle keybinds
    let list_box_clone = list_box.clone();
    let event_controller = gtk::EventControllerKey::new();
    event_controller.connect_key_pressed(move |_, keyval, _, state| {
        keys::handle_key(app_config.clone(), keyval, state, Some(&search_results.borrow()), &list_box_clone)
    });
    event_controller.set_propagation_phase(gtk::PropagationPhase::Capture);
    window.add_controller(event_controller);

    window.present();
}
