use bevy::prelude::*;

use crate::assets::Graphics;

pub struct PowerupsPlugin;

impl Plugin for PowerupsPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub enum PowerUp {
    Health,
}

#[derive(Component)]
pub struct PowerupSpawnChance {
    pub health: f32,
}

#[derive(Component, Deref, DerefMut)]
pub struct PowerupTimer(pub Timer);
