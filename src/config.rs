use std::{env, fs};
use gtk::CssProvider;
use gtk::gdk::{Key, ModifierType, Display};
use serde_derive::Deserialize;
use toml;
use toml::Table;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub terminal: String,
    pub keys: Keys,
    pub window: Window
}

#[derive(Deserialize, Clone, Debug)]
pub struct Keys {
    pub exit: Vec<String>,
    pub next: Vec<String>,
    pub previous: Vec<String>,
    pub action: Vec<String>
}

#[derive(Deserialize, Clone, Debug)]
pub struct Window {
    pub top: i32,
    pub width: i32,
    pub height: i32
}

pub fn get_config() -> Config {
    if let Some(home) = env::var_os("HOME") {
        let home_string = home.to_owned().to_str().unwrap_or("").to_string();
        let config_path = home_string + "/.config/waylauncher/config.toml";

        let config_data = fs::read_to_string(config_path).unwrap_or("".to_string());
        if !config_data.is_empty() {
            toml::from_str(&config_data).unwrap_or_else(|e| panic!("Could not load config:\n{e}"))
        } else {
            println!("Using default config");
            // Default config
            Config {
                terminal: "foot".to_string(),
                window: Window {
                    top: 300,
                    width: 800,
                    height: 500
                },
                keys: Keys {
                    exit: vec!["Escape".to_string()],
                    action: vec!["Return".to_string()] ,
                    next: vec!["C-j".to_string()],
                    previous: vec!["C-k".to_string()]
                }
            }
        }
    } else {
        panic!("Could not find home path");
    }
}

pub fn load_css() {
    if let Some(home) = env::var_os("HOME") {
        let display = Display::default().expect("Could not get default display");
        let css_provider = CssProvider::new();

        let home_string = home.to_owned().to_str().unwrap_or("").to_string();
        let css_path = home_string + "/.config/waylauncher/style.css";
        let css_data = fs::read_to_string(css_path).unwrap_or("".to_string());
        if !css_data.is_empty() {
            css_provider.load_from_string(&css_data);
        } else {
            println!("Using default stylesheet");
            css_provider.load_from_resource("/dev/topeko/waylauncher/style.css");
        }
        gtk::style_context_add_provider_for_display(&display, &css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    } else {
        panic!("Could not find home path");
    }
}
