# Waylauncher

GTK4 based Wayland fuzzy search application launcher written in Rust.
I am a Rust beginner and this is my first GTK app.

## Features
- Fuzzy search
- XDG desktop entries

## Example config
~/.config/waylauncher/config.toml
```toml
terminal = "foot"

[window]
top = 300
width = 800
height = 500

[keys]
exit = ["Escape"]
action = ["Return"]
next = ["C-j", "C-n", "Down"]
previous = ["C-k", "C-p", "Up"]
```

## Custom CSS
~/.config/waylauncher/style.css
