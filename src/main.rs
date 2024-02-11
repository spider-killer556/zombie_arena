#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

mod assets;
mod map;
mod player;
mod settings;
mod state;
mod ui;

use assets::AssetsPlugin;
use settings::SettingsPlugin;
use state::StatePlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins((SettingsPlugin, StatePlugin, AssetsPlugin, UiPlugin))
        .run();
}
