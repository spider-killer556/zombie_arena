use bevy::{app::AppExit, prelude::*};
use strum::*;

use crate::{
    assets::{Fonts, Graphics},
    state::GameState,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), main_menu)
            .add_systems(
                Update,
                (button_system, enter_to_play).run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu);

        app.add_systems(OnEnter(GameState::GameOver), game_over_ui)
            .add_systems(
                Update,
                update_game_over.run_if(in_state(GameState::GameOver)),
            )
            .add_systems(OnExit(GameState::GameOver), cleanup_game_over);
    }
}

// All the stuff related to the main menu
#[derive(Component)]
pub struct MainMenu;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component, EnumIter, Copy, Clone)]
enum MainMenuButtons {
    Play,
    Quit,
}

impl std::fmt::Display for MainMenuButtons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MainMenuButtons::Play => "Play",
                MainMenuButtons::Quit => "Quit",
            }
        )
    }
}

fn main_menu(mut commands: Commands, graphics: Res<Graphics>, fonts: Res<Fonts>) {
    commands
        .spawn((
            MainMenu,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|ui| {
            ui.spawn(ImageBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                image: graphics.background.clone().into(),
                ..default()
            });
        });

    commands
        .spawn((
            MainMenu,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|ui| {
            ui.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Press Enter to start!".to_string(),
                        style: TextStyle {
                            font: fonts.zombiecontrol.clone(),
                            font_size: 100.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
        });
}

fn enter_to_play(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        next_state.set(GameState::Playing);
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &MainMenuButtons, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit: EventWriter<AppExit>,
) {
    for (interaction, button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                match button {
                    MainMenuButtons::Play => {
                        println!("Play button pressed");
                        next_state.set(GameState::Playing);
                    }
                    MainMenuButtons::Quit => {
                        println!("Quit button pressed");
                        app_exit.send(AppExit);
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// All the stuff related to the game over screen
#[derive(Component)]
pub struct GameOver;

fn game_over_ui(mut commands: Commands, graphics: Res<Graphics>, fonts: Res<Fonts>) {
    commands
        .spawn((
            GameOver,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|ui| {
            ui.spawn(ImageBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                image: graphics.background.clone().into(),
                ..default()
            });
        });

    commands
        .spawn((
            GameOver,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|ui| {
            ui.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            })
            .with_children(|ui| {
                ui.spawn(TextBundle::from_section(
                    "Game Over",
                    TextStyle {
                        font: fonts.zombiecontrol.clone(),
                        font_size: 72.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
                ui.spawn(TextBundle::from_section(
                    "Press Space to restart!",
                    TextStyle {
                        font: fonts.zombiecontrol.clone(),
                        font_size: 72.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
        });
}

fn update_game_over(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<GameOver>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
