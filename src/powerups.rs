use bevy::prelude::*;

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
