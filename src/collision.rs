use std::ops::Mul;

use bevy::prelude::*;
use bevy::sprite::collide_aabb;
use bevy_kira_audio::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, *};

use crate::{
    assets::{Graphics, Sounds},
    combat::{CombatBundle, Health},
    powerups::{PowerUp, PowerupSpawnChance, PowerupTimer},
    state::GameState,
    survivour::{Bullet, Survivour},
    waves::ZombieCount,
    zombies::Zombie,
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TweeningPlugin);

        app.add_systems(
            Update,
            (collision_zombies_bullets, survivour_health_pickup).run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (despawn_blood, despawn_powerup).run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct CollisionSize(pub Vec2);

#[derive(Component, Deref, DerefMut)]
pub struct BloodTimer(pub Timer);

fn collision_zombies_bullets(
    mut commands: Commands,
    mut zombies: Query<(
        Entity,
        &Transform,
        &CollisionSize,
        &mut Health,
        &Zombie,
        &PowerupSpawnChance,
    )>,
    bullets: Query<(Entity, &Transform, &CollisionSize, &Bullet)>,
    graphics: Res<Graphics>,
    sounds: Res<Sounds>,
    audio: Res<Audio>,
    mut zombie_count: ResMut<ZombieCount>,
) {
    for (
        zombie_entity,
        zombie_transform,
        zombie_size,
        mut zombie_health,
        zombie,
        power_spawn_chance,
    ) in zombies.iter_mut()
    {
        for (bullet_entity, bullet_transform, bullet_size, _) in bullets.iter() {
            let collision = collide_aabb::collide(
                zombie_transform.translation,
                zombie_size.0,
                bullet_transform.translation,
                bullet_size.0,
            );

            if collision.is_none() {
                continue;
            }
            audio.play(sounds.splat.clone());
            commands.entity(bullet_entity).despawn();
            zombie_health.0 -= 1;

            if zombie_health.0 > 0 {
                continue;
            }
            zombie_count.decrease_count(zombie);
            health_pickup_spawn(
                &mut commands,
                &graphics,
                power_spawn_chance,
                bullet_transform,
                zombie_transform,
            );
            // Replace zombie sprite with blood and then despawn it after a delay
            commands
                .entity(zombie_entity)
                .remove::<(Zombie, CollisionSize, CombatBundle)>()
                .insert((
                    SpriteBundle {
                        texture: graphics.blood.clone(),
                        transform: Transform::from_translation(
                            zombie_transform.translation.truncate().extend(1.5),
                        ),
                        ..default()
                    },
                    BloodTimer(Timer::from_seconds(10.0, TimerMode::Once)),
                ));
        }
    }
}

fn health_pickup_spawn(
    commands: &mut Commands,
    graphics: &Res<Graphics>,
    power_spawn_chance: &PowerupSpawnChance,
    bullet_transform: &Transform,
    zombie_transform: &Transform,
) {
    if rand::random::<f32>() < power_spawn_chance.health {
        let zombie_pos = zombie_transform.translation;

        let direction = bullet_transform
            .rotation
            .mul(Vec3::X)
            .normalize()
            .truncate();

        let tween = Tween::new(
            EaseFunction::ExponentialOut,
            std::time::Duration::from_secs_f32(0.5),
            TransformPositionLens {
                start: Vec3::new(zombie_pos.x, zombie_pos.y, 1.75),
                end: Vec3::new(
                    zombie_pos.x + direction.x * 25.0,
                    zombie_pos.y + direction.y * 25.0,
                    1.75,
                ),
            },
        );

        commands.spawn((
            SpriteBundle {
                texture: graphics.health_pickup.clone(),
                transform: Transform::from_translation(zombie_pos.truncate().extend(1.75)),
                ..default()
            },
            PowerUp::Health,
            Animator::new(tween),
            PowerupTimer(Timer::from_seconds(60.0, TimerMode::Once)),
            CollisionSize(Vec2::new(32.0, 26.0)),
        ));
    }
}

fn survivour_health_pickup(
    mut commands: Commands,
    mut survivour: Query<(&Transform, &CollisionSize, &mut Health), With<Survivour>>,
    powerups: Query<(Entity, &Transform, &CollisionSize, &PowerUp)>,
    audio: Res<Audio>,
    sounds: Res<Sounds>,
) {
    for (sv_tf, player_size, mut health) in survivour.iter_mut() {
        for (entity, powerup_transform, powerup_size, powerup) in powerups.iter() {
            let collision = collide_aabb::collide(
                sv_tf.translation,
                player_size.0,
                powerup_transform.translation,
                powerup_size.0,
            );

            if collision.is_none() {
                continue;
            }
            match powerup {
                PowerUp::Health => {
                    health.0 += if health.0 < 3 { 1 } else { 0 };
                    audio.play(sounds.pickup.clone());
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

fn despawn_blood(
    mut commands: Commands,
    mut blood: Query<(Entity, &mut BloodTimer)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in blood.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn despawn_powerup(
    mut commands: Commands,
    mut powerups: Query<(Entity, &mut PowerupTimer)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in powerups.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
