#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

use bevy::prelude::*;

mod assets;
mod collision;
mod combat;
mod game_conf;
mod map;
mod movement;
mod powerups;
mod state;
mod survivour;
mod ui;
mod waves;
mod camera;
mod zombies;

use assets::AssetsPlugin;
use collision::CollisionPlugin;
use game_conf::GameConfPlugin;
use map::MapPlugin;
use state::StatePlugin;
use survivour::SurvivourPlugin;
use ui::UiPlugin;
use waves::WavesPlugin;
use camera::CameraPlugin;
use zombies::ZombiesPlugin;

fn main() {
    App::new()
        .add_plugins((
            GameConfPlugin,
            StatePlugin,
            CameraPlugin,
            AssetsPlugin,
            UiPlugin,
            MapPlugin,
            SurvivourPlugin,
            ZombiesPlugin,
            WavesPlugin,
            CollisionPlugin,
        ))
        .run();
}
