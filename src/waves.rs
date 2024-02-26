use crate::zombies::ZombieBundle;
use crate::{
    assets::{Fonts, Graphics},
    map::MapBounds,
    state::GameState,
    zombies::Zombie,
};
use bevy::prelude::*;
use rand::Rng;

pub struct WavesPlugin;

impl Plugin for WavesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), init_wave_stats);
        app.add_systems(
            Update,
            update_wave_text
                .run_if(in_state(GameState::Playing).and_then(resource_changed::<Wave>())),
        )
        .add_systems(
            Update,
            update_zombies_remaining.run_if(in_state(GameState::Playing)),
        )
        .add_systems(Update, update_score.run_if(in_state(GameState::Playing)))
        .add_systems(
            Update,
            generate_wave
                .run_if(in_state(GameState::Playing).and_then(resource_equals(ZombieCount::ZERO))),
        );
    }
}

// Manage the wave in a single resource
#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct Wave {
    pub count: i32,
    pub timer: Timer,
}

impl Default for Wave {
    fn default() -> Self {
        Self {
            count: 1,
            timer: Timer::from_seconds(5.0, TimerMode::Once),
        }
    }
}

#[derive(Resource, Default)]
pub struct Score(pub i32);

impl Score {
    pub fn increase(&mut self, zombie: &Zombie) {
        match zombie {
            Zombie::Chaser => self.0 += 1,
            Zombie::Crawler => self.0 += 3,
            Zombie::Bloater => self.0 += 5,
        }
    }
}

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct WaveText;

#[derive(Resource, Default, PartialEq, Eq, Debug, Clone)]
pub struct ZombieCount {
    pub chaser: i32,
    pub crawler: i32,
    pub bloater: i32,
}

impl ZombieCount {
    pub const ZERO: Self = Self {
        chaser: 0,
        crawler: 0,
        bloater: 0,
    };

    pub fn total(&self) -> i32 {
        self.chaser + self.crawler + self.bloater
    }

    pub fn decrease_count(&mut self, zombie: &Zombie) {
        match zombie {
            Zombie::Chaser => self.chaser -= 1,
            Zombie::Crawler => self.crawler -= 1,
            Zombie::Bloater => self.bloater -= 1,
        }
    }
}

fn init_wave_stats(mut cmds: Commands, fonts: Res<Fonts>) {
    cmds.insert_resource(Wave::default());
    cmds.insert_resource(ZombieCount::default());
    cmds.insert_resource(FirstWave::default());
    cmds.insert_resource(Score::default());

    // Spawn the game HUD
    cmds.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect {
                left: Val::Px(20.0),
                right: Val::Px(20.0),
                top: Val::Px(20.0),
                bottom: Val::Px(20.0),
            },
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                format!("Wave {}", 1),
                TextStyle {
                    font: fonts.zombiecontrol.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ),
            WaveText,
        ));
        parent.spawn((
            TextBundle::from_section(
                "Score: 0",
                TextStyle {
                    font: fonts.zombiecontrol.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ),
            ScoreText,
        ));
        parent.spawn((
            TextBundle::from_section(
                format!("Zombies Left {}", 0),
                TextStyle {
                    font: fonts.zombiecontrol.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ),
            ZombieLeftText,
        ));
    });
}

#[derive(Resource)]
struct FirstWave(pub bool);

impl Default for FirstWave {
    fn default() -> Self {
        Self(true)
    }
}

fn generate_wave(
    mut commands: Commands,
    mut wave: ResMut<Wave>,
    map_bounds: Query<&MapBounds>,
    graphics: Res<Graphics>,
    mut first_wave: ResMut<FirstWave>,
    time: Res<Time>,
) {
    if !wave.timer.tick(time.delta()).finished() {
        return;
    }
    let map_bounds = map_bounds.single();

    let mut rng = rand::thread_rng();
    let chaser_count = rng.gen_range(1..=wave.count * 3);
    let crawler_count = rng.gen_range(1..=wave.count * 2);
    let bloater_count = rng.gen_range(1..=wave.count);
    commands.insert_resource(ZombieCount {
        chaser: chaser_count,
        crawler: crawler_count,
        bloater: bloater_count,
    });

    let x_spawn_range = -map_bounds.x * 32.0..map_bounds.x * 32.0;
    let y_spawn_range = -map_bounds.y * 32.0..map_bounds.y * 32.0;

    // Spawn chaser zombies
    for _ in 0..chaser_count {
        let x = rng.gen_range(x_spawn_range.clone());
        let y = rng.gen_range(y_spawn_range.clone());
        commands.spawn(ZombieBundle::chaser(Vec2::new(x, y), &graphics));
    }

    // Spawn crawler zombies
    for _ in 0..crawler_count {
        let x = rng.gen_range(x_spawn_range.clone());
        let y = rng.gen_range(y_spawn_range.clone());
        commands.spawn(ZombieBundle::crawler(Vec2::new(x, y), &graphics));
    }

    // Spawn bloater zombies
    for _ in 0..bloater_count {
        let x = rng.gen_range(x_spawn_range.clone());
        let y = rng.gen_range(y_spawn_range.clone());
        commands.spawn(ZombieBundle::bloater(Vec2::new(x, y), &graphics));
    }

    // Check if this is the first wave
    if first_wave.0 {
        first_wave.0 = false;
    } else {
        wave.count += 1;
    }
    wave.timer.reset();
}

#[derive(Component)]
pub struct ZombieLeftText;

fn update_wave_text(wave: Res<Wave>, mut wave_text: Query<&mut Text, With<WaveText>>) {
    for mut text in wave_text.iter_mut() {
        text.sections[0].value = format!("Wave {}", wave.count);
    }
}

fn update_zombies_remaining(
    zombies: Res<ZombieCount>,
    mut zombie_left_text: Query<&mut Text, With<ZombieLeftText>>,
) {
    let zombies_count = zombies.total();
    for mut text in zombie_left_text.iter_mut() {
        text.sections[0].value = format!("Zombies Left: {}", zombies_count);
    }
}

fn update_score(score: Res<Score>, mut score_text: Query<&mut Text, With<ScoreText>>) {
    for mut text in score_text.iter_mut() {
        text.sections[0].value = format!("Score: {}", score.0);
    }
}
