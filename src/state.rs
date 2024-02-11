use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>();
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq, States, Hash)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    Settings,
    Playing,
    Paused,
}
