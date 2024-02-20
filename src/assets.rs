use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::prelude::{AudioSource, *};

use crate::state::GameState;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin);
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::MainMenu)
                .load_collection::<Graphics>()
                .load_collection::<Sounds>()
                .load_collection::<Fonts>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct Graphics {
    #[asset(path = "graphics/ammo_icon.png")]
    pub ammo_icon: Handle<Image>,
    #[asset(path = "graphics/ammo_pickup.png")]
    pub ammo_pickup: Handle<Image>,
    #[asset(path = "graphics/background.png")]
    pub background: Handle<Image>,
    #[asset(path = "graphics/bloater.png")]
    pub bloater: Handle<Image>,
    #[asset(path = "graphics/blood.png")]
    pub blood: Handle<Image>,
    #[asset(path = "graphics/bullet.png")]
    pub bullet: Handle<Image>,
    #[asset(path = "graphics/chaser.png")]
    pub chaser: Handle<Image>,
    #[asset(path = "graphics/crawler.png")]
    pub crawler: Handle<Image>,
    #[asset(path = "graphics/crosshair.png")]
    pub crosshair: Handle<Image>,
    #[asset(path = "graphics/health_pickup.png")]
    pub health_pickup: Handle<Image>,
    #[asset(path = "graphics/map.png")]
    pub map: Handle<Image>,
    #[asset(path = "graphics/player.png")]
    pub player: Handle<Image>,
    #[asset(path = "graphics/heart.png")]
    pub heart: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct Sounds {
    #[asset(path = "sound/hit.wav")]
    pub hit: Handle<AudioSource>,
    #[asset(path = "sound/pickup.wav")]
    pub pickup: Handle<AudioSource>,
    #[asset(path = "sound/powerup.wav")]
    pub powerup: Handle<AudioSource>,
    #[asset(path = "sound/reload_failed.wav")]
    pub reload_failed: Handle<AudioSource>,
    #[asset(path = "sound/reload.wav")]
    pub reload: Handle<AudioSource>,
    #[asset(path = "sound/shoot.wav")]
    pub shoot: Handle<AudioSource>,
    #[asset(path = "sound/splat.wav")]
    pub splat: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct Fonts {
    #[asset(path = "fonts/zombiecontrol.ttf")]
    pub zombiecontrol: Handle<Font>,
}
