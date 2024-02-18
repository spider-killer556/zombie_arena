use bevy::prelude::*;

pub struct WorldCameraPlugin;

impl Plugin for WorldCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world_camera);
    }
}

#[derive(Component)]
pub struct WorldCamera;

fn spawn_world_camera(mut cmds: Commands) {
    cmds.spawn((WorldCamera, Camera2dBundle::default()));
}
