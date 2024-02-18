use bevy::utils::Duration;
use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    assets::Graphics,
    collision::CollisionSize,
    combat::{AttackDelay, CombatBundle, Health},
    movement::MovementSpeed,
    powerups::PowerupSpawnChance,
    state::GameState,
    survivour::Survivour,
};

pub struct ZombiesPlugin;

impl Plugin for ZombiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, zombies_walk.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub enum Zombie {
    Chaser,
    Crawler,
    Bloater,
}

#[derive(Bundle)]
pub struct ZombieBundle {
    pub sprite_bundle: SpriteBundle,
    pub zombie: Zombie,
    pub movement_speed: MovementSpeed,
    pub combat_bundle: CombatBundle,
    pub collision_size: CollisionSize,
    pub powerup_spawn_chance: PowerupSpawnChance,
}

const ZOMBIE_Z: f32 = 4.0;

impl ZombieBundle {
    pub fn chaser(pos: Vec2, graphics: &Graphics) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_xyz(pos.x, pos.y, ZOMBIE_Z),
                texture: graphics.chaser.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..Default::default()
                },
                ..default()
            },
            zombie: Zombie::Chaser,
            movement_speed: MovementSpeed { speed: 100.0 },
            combat_bundle: CombatBundle {
                health: Health(1),
                attack_delay: AttackDelay {
                    delay: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Once),
                },
            },
            // Collision size is smaller than the sprite to make it more realistic to hit
            collision_size: CollisionSize(Vec2::new(19.0, 41.0)),
            powerup_spawn_chance: PowerupSpawnChance { health: 0.1 },
        }
    }

    pub fn crawler(pos: Vec2, graphics: &Graphics) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_xyz(pos.x, pos.y, ZOMBIE_Z),
                texture: graphics.crawler.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..Default::default()
                },
                ..default()
            },
            zombie: Zombie::Crawler,
            movement_speed: MovementSpeed { speed: 80.0 },
            combat_bundle: CombatBundle {
                health: Health(3),
                attack_delay: AttackDelay {
                    delay: Timer::new(Duration::from_secs_f32(2.0), TimerMode::Once),
                },
            },
            collision_size: CollisionSize(Vec2::new(38.0, 32.0)),
            powerup_spawn_chance: PowerupSpawnChance { health: 0.12 },
        }
    }

    pub fn bloater(pos: Vec2, graphics: &Graphics) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_xyz(pos.x, pos.y, ZOMBIE_Z),
                texture: graphics.bloater.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..Default::default()
                },
                ..default()
            },
            zombie: Zombie::Bloater,
            movement_speed: MovementSpeed { speed: 60.0 },
            combat_bundle: CombatBundle {
                health: Health(5),
                attack_delay: AttackDelay {
                    delay: Timer::new(Duration::from_secs_f32(3.0), TimerMode::Once),
                },
            },
            collision_size: CollisionSize(Vec2::new(31.0, 51.0)),
            powerup_spawn_chance: PowerupSpawnChance { health: 0.15 },
        }
    }
}

fn zombies_walk(
    survivour: Query<&Transform, (With<Survivour>, Without<Zombie>)>,
    mut zombies: Query<(&mut Transform, &MovementSpeed), (With<Zombie>, Without<Survivour>)>,
    time: Res<Time>,
) {
    let Ok(survivour_transform) = survivour.get_single() else {
        return;
    };

    for (mut zombie_transform, speed) in zombies.iter_mut() {
        let mut angle = (survivour_transform.translation.y - zombie_transform.translation.y)
            .atan2(survivour_transform.translation.x - zombie_transform.translation.x);
        if angle < 0.0 {
            angle += 2.0 * PI;
        }

        zombie_transform.translation.x += **speed * angle.cos() * time.delta_seconds();
        zombie_transform.translation.y += **speed * angle.sin() * time.delta_seconds();

        zombie_transform.rotation = Quat::from_rotation_z(angle);
    }
}
