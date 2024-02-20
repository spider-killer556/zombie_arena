use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world_camera);
    }
}

#[derive(Component)]
pub struct GameCamera;

fn spawn_world_camera(mut cmds: Commands) {
    cmds.spawn((GameCamera, Camera2dBundle::default()));
}
