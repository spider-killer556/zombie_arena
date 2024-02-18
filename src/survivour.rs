use crate::assets::Sounds;
use crate::collision::CollisionSize;
use crate::combat::{AttackDelay, CombatBundle, Health};
use crate::map::MapBounds;
use crate::movement::MovementSpeed;
use crate::{assets::Graphics, state::GameState, world_camera::WorldCamera};
use bevy::prelude::*;
use bevy::utils::Duration;
use bevy::window::PrimaryWindow;
use bevy_kira_audio::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct SurvivourPlugin;

impl Plugin for SurvivourPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<SurvivourActions>::default())
            .add_plugins(AudioPlugin);

        app.init_resource::<MouseWorldCoords>();

        app.add_systems(OnEnter(GameState::Playing), (spawn_cursor, spawn_survivour))
            .add_systems(
                Update,
                (survivour_walks, update_camera)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    update_mouse_world_coords,
                    update_cursor,
                    look_at_cursor,
                    shoot_bullet,
                    update_bullet,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

const SURVIVOUR_Z: f32 = 2.0;
const BULLET_Z: f32 = 2.5;

#[derive(Component)]
pub struct Survivour;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum SurvivourActions {
    Up,
    Down,
    Left,
    Right,
    Shoot,
}

#[derive(Bundle)]
struct SurvivourBundle {
    survivour: Survivour,
    input_manager: InputManagerBundle<SurvivourActions>,
}

#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
    pub start_position: Vec2,
}

impl SurvivourBundle {
    fn default_input_map() -> InputMap<SurvivourActions> {
        // This allows us to replace `ArpgAction::Up` with `Up`,
        // significantly reducing boilerplate
        use SurvivourActions::*;
        let mut input_map = InputMap::<SurvivourActions>::default();

        // Movement
        input_map.insert(KeyCode::W, Up);
        input_map.insert(GamepadButtonType::DPadUp, Up);

        input_map.insert(KeyCode::S, Down);
        input_map.insert(GamepadButtonType::DPadDown, Down);

        input_map.insert(KeyCode::A, Left);
        input_map.insert(GamepadButtonType::DPadLeft, Left);

        input_map.insert(KeyCode::D, Right);
        input_map.insert(GamepadButtonType::DPadRight, Right);

        input_map.insert(MouseButton::Left, Shoot);

        input_map
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct MouseWorldCoords(Vec2);

#[derive(Component)]
pub struct GameCursor;

fn spawn_cursor(mut commands: Commands, graphics: Res<Graphics>, mut window: Query<&mut Window>) {
    let mut window = window.single_mut();
    window.cursor.visible = false;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            texture: graphics.aim.clone(),
            transform: Transform::from_xyz(0., 0., 100.),
            ..default()
        },
        GameCursor,
    ));

    println!("Cursor spawned");
}

fn update_mouse_world_coords(
    cam_q: Query<(&Camera, &GlobalTransform)>,
    mut coords: ResMut<MouseWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok((cam, cam_tf)) = cam_q.get_single() else {
        return;
    };
    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| cam.viewport_to_world(cam_tf, cursor))
        .map(|ray| ray.origin.truncate())
    {
        coords.0 = world_position;
    }
}

fn update_cursor(
    mut game_cursor: Query<&mut Transform, With<GameCursor>>,
    coords: Res<MouseWorldCoords>,
) {
    let Ok(mut tf) = game_cursor.get_single_mut() else {
        return;
    };

    tf.translation.x = coords.x;
    tf.translation.y = coords.y;
}

fn spawn_survivour(mut cmds: Commands, graphics: Res<Graphics>) {
    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0., SURVIVOUR_Z),
            texture: graphics.player.clone(),
            ..default()
        },
        InputManagerBundle {
            input_map: SurvivourBundle::default_input_map(),
            ..default()
        },
        Survivour,
        CombatBundle {
            health: Health(3),
            attack_delay: AttackDelay {
                delay: Timer::new(Duration::from_secs_f32(0.6), TimerMode::Once),
            },
        },
        MovementSpeed { speed: 10.0 },
        CollisionSize(Vec2::new(32.0, 32.0)),
    ));
}

fn survivour_walks(
    mut survivour_actions: Query<(
        &ActionState<SurvivourActions>,
        &mut Transform,
        &MovementSpeed,
    )>,
    map_bounds: Query<&MapBounds>,
    time: Res<Time>,
) {
    let Ok((actions, mut tf, speed)) = survivour_actions.get_single_mut() else {
        return;
    };
    let map_bounds = map_bounds.single();

    let mut delta = Vec2::splat(0.0);

    if actions.pressed(SurvivourActions::Up) {
        delta += Vec2::Y;
    }

    if actions.pressed(SurvivourActions::Down) {
        delta += -Vec2::Y;
    }

    if actions.pressed(SurvivourActions::Left) {
        delta += -Vec2::X;
    }

    if actions.pressed(SurvivourActions::Right) {
        delta += Vec2::X;
    }

    delta *= 15.0;

    tf.translation += delta.extend(0.) * time.delta_seconds() * **speed;

    // Clamp the player to the map bounds
    tf.translation.x = tf
        .translation
        .x
        .clamp(-map_bounds.x * 32.0 + 16.0, map_bounds.x * 32.0 - 16.0);
    tf.translation.y = tf
        .translation
        .y
        .clamp(-map_bounds.y * 32.0 + 16.0, map_bounds.y * 32.0 - 16.0);
}

fn look_at_cursor(
    mut survivour_tf: Query<&mut Transform, (With<Survivour>, Without<WorldCamera>)>,
    coords: Res<MouseWorldCoords>,
) {
    let Ok(mut survivour_tf) = survivour_tf.get_single_mut() else {
        return;
    };

    let direction = (coords.0 - survivour_tf.translation.truncate()).normalize();

    let angle = direction.y.atan2(direction.x);

    survivour_tf.rotation = Quat::from_rotation_z(angle);
}

fn update_camera(
    survivour_tf: Query<&Transform, (With<Survivour>, Without<WorldCamera>)>,
    mut world_cam_tf: Query<&mut Transform, (With<WorldCamera>, Without<Survivour>)>,
    window: Query<&Window>,
    map_bounds: Query<&MapBounds>,
) {
    let Ok(survivour_tf) = survivour_tf.get_single() else {
        return;
    };
    let Ok(mut cam_tf) = world_cam_tf.get_single_mut() else {
        return;
    };
    let window = window.single();
    let map_bounds = map_bounds.single();
    let window_width = window.width();
    let window_height = window.height();

    let (x_diff_pos, x_diff_neg) = (
        map_bounds.x * 32.0 - survivour_tf.translation.x,
        -map_bounds.x * 32.0 - survivour_tf.translation.x,
    );
    let (y_diff_pos, y_diff_neg) = (
        map_bounds.y * 32.0 - survivour_tf.translation.y,
        -map_bounds.y * 32.0 - survivour_tf.translation.y,
    );

    let x_collision =
        x_diff_pos - window_width / 2.0 > 0.0 && x_diff_neg - -(window_width / 2.0) < 0.0;

    let y_collision =
        y_diff_pos - window_height / 2.0 > 0.0 && y_diff_neg - -(window_height / 2.0) < 0.0;

    if x_collision {
        cam_tf.translation.x = survivour_tf.translation.x;
    }
    if y_collision {
        cam_tf.translation.y = survivour_tf.translation.y;
    }
}

fn shoot_bullet(
    mut cmds: Commands,
    mut survivour_tf: Query<(&Transform, &ActionState<SurvivourActions>, &mut AttackDelay)>,
    time: Res<Time>,
    graphics: Res<Graphics>,
    sounds: Res<Sounds>,
    audio: Res<Audio>,
) {
    let Ok((survivour_tf, survivour_actions, mut shoot_delay)) = survivour_tf.get_single_mut()
    else {
        return;
    };

    shoot_delay.tick(time.delta());
    if shoot_delay.finished() && survivour_actions.pressed(SurvivourActions::Shoot) {
        let bullet_start_pos = survivour_tf.translation.truncate()
            + survivour_tf.rotation.mul_vec3(Vec3::X * 30.0).truncate();

        cmds.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    ..default()
                },
                transform: Transform {
                    translation: bullet_start_pos.extend(BULLET_Z),
                    rotation: survivour_tf.rotation,
                    ..default()
                },
                texture: graphics.bullet.clone(),
                ..default()
            },
            Bullet {
                speed: 700.0,
                start_position: bullet_start_pos,
            },
            CollisionSize(Vec2::new(16.0, 16.0)),
        ));

        audio.play(sounds.shoot.clone());
        shoot_delay.reset();
    }
}

fn update_bullet(
    mut cmds: Commands,
    mut bullets: Query<(Entity, &Bullet, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, bullet, mut tf) in bullets.iter_mut() {
        if (tf.translation.truncate() - bullet.start_position).length() > 2000.0 {
            // Remove bullet if it's too far from the player
            cmds.entity(entity).despawn_recursive();
        } else {
            let direction = tf.rotation.mul_vec3(Vec3::X);
            tf.translation += direction * bullet.speed * time.delta_seconds();
        }
    }
}
