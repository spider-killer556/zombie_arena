use std::ops::SubAssign;

use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Health(pub i32);

#[derive(Component, Deref, DerefMut)]
pub struct AttackDelay {
    pub delay: Timer,
}

#[derive(Bundle)]
pub struct CombatBundle {
    pub health: Health,
    pub attack_delay: AttackDelay,
}
