#![allow(unused)]
use gtk::{glib, gdk, gio, prelude::*};

mod ui;
mod config;
mod entries;
mod keys;
mod search;

const APP_ID: &str = "dev.topeko.waylauncher";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("waylauncher.gresource")
        .expect("Failed to register resources.");

    let app = gtk::Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_startup(|_| config::load_css());
    app.connect_activate(|app| ui::build_ui(app));

    app.run()
}
