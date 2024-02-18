use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct MovementSpeed {
    pub speed: f32,
}
