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
            .add_systems(Update, button_system.run_if(in_state(GameState::MainMenu)))
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu);
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
    Settings,
    Quit,
}

impl MainMenuButtons {
    fn to_string(&self) -> String {
        match self {
            MainMenuButtons::Play => "Play".to_string(),
            MainMenuButtons::Settings => "Settings".to_string(),
            MainMenuButtons::Quit => "Quit".to_string(),
        }
    }
}

fn main_menu(mut commands: Commands, graphics: Res<Graphics>, fonts: Res<Fonts>) {
    commands.spawn((Camera2dBundle::default(), MainMenu));

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
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|ui| {
            ui.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(35.0),
                    height: Val::Percent(35.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            })
            .with_children(|ui| {
                MainMenuButtons::iter().for_each(|button| {
                    ui.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(150.0),
                                height: Val::Px(65.0),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            border_color: BorderColor(Color::BLACK),
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        },
                        button,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            button.to_string(),
                            TextStyle {
                                font: fonts.zombiecontrol.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ));
                    });
                });
            });
        });
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
                    MainMenuButtons::Settings => {
                        println!("Settings button pressed");
                        next_state.set(GameState::Settings);
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
