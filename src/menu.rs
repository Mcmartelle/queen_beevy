use crate::actions::{Actions, InputDevice};
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(Update, click_play_button.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

#[derive(Component)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.3, 0.3, 0.3),
        }
    }
}

#[derive(Component)]
struct Menu;

fn setup_menu(
    mut commands: Commands,
    textures: Res<TextureAssets>,
) {
    info!("menu");
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            children.spawn(TextBundle::from_section(
                "Objective: Get Flowers with the Queen Bee",
                TextStyle {
                    font: default(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
                
            ).with_style(Style {
                margin: UiRect {
                    left: Val::Px(10.0),
                    right: Val::Px(10.0),
                    top: Val::Px(10.0),
                    bottom: Val::Px(5.0),
                },
                ..default()
            }),);
            children.spawn(TextBundle::from_section(
                "WARNING: Flowers increase spawn rate of worker bees!",
                TextStyle {
                    font: default(),
                    font_size: 22.0,
                    color: Color::rgb(0.7, 0.0, 0.0),
                },
                
            ).with_style(Style {
                margin: UiRect {
                    left: Val::Px(5.0),
                    right: Val::Px(10.0),
                    top: Val::Px(10.0),
                    bottom: Val::Px(50.0),
                },
                ..default()
            }),);
            let button_colors = ButtonColors::default();
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(400.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: button_colors.normal.into(),
                        ..Default::default()
                    },
                    button_colors,
                    ChangeState(GameState::Playing),
                    ChangeInput(InputDevice::Gamepad),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play with Controller",
                        TextStyle {
                            font_size: 32.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
            children.spawn(TextBundle::from_section(
                "Left Stick: Queen Bee, Right Stick: Worker Bees",
                TextStyle {
                    font: default(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ).with_style(Style {
                margin: UiRect {
                    left: Val::Px(10.0),
                    right: Val::Px(10.0),
                    top: Val::Px(10.0),
                    bottom: Val::Px(30.0),
                },
                ..default()
            }),);
            let button_colors = ButtonColors::default();
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(400.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: button_colors.normal.into(),
                        ..Default::default()
                    },
                    button_colors,
                    ChangeState(GameState::Playing),
                    ChangeInput(InputDevice::Keyboard),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play with Keyboard",
                        TextStyle {
                            font_size: 32.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
            children.spawn(TextBundle::from_section(
                "WASD: Queen Bee, Arrow Keys: Worker Bees",
                TextStyle {
                    font: default(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ).with_style(Style {
                margin: UiRect {
                    left: Val::Px(10.0),
                    right: Val::Px(10.0),
                    top: Val::Px(10.0),
                    bottom: Val::Px(30.0),
                },
                ..default()
            }),);
        });
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    bottom: Val::Px(5.),
                    width: Val::Percent(100.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(170.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::SpaceAround,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(5.)),
                            ..Default::default()
                        },
                        background_color: Color::NONE.into(),
                        ..Default::default()
                    },
                    ButtonColors {
                        normal: Color::NONE,
                        ..default()
                    },
                    OpenLink("https://bevyengine.org"),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Made with Bevy",
                        TextStyle {
                            font_size: 15.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                    parent.spawn(ImageBundle {
                        image: textures.bevy.clone().into(),
                        style: Style {
                            width: Val::Px(32.),
                            ..default()
                        },
                        ..default()
                    });
                });
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(170.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::SpaceAround,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(5.)),
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..Default::default()
                    },
                    ButtonColors {
                        normal: Color::NONE,
                        hovered: Color::rgb(0.25, 0.25, 0.25),
                    },
                    OpenLink("https://github.com/NiklasEi/bevy_game_template"),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Open source",
                        TextStyle {
                            font_size: 15.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                    parent.spawn(ImageBundle {
                        image: textures.github.clone().into(),
                        style: Style {
                            width: Val::Px(32.),
                            ..default()
                        },
                        ..default()
                    });
                });
        });
}

#[derive(Component)]
struct ChangeState(GameState);

#[derive(Component)]
struct ChangeInput(InputDevice);

#[derive(Component)]
struct OpenLink(&'static str);

fn click_play_button(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangeState>,
            Option<&OpenLink>,
            Option<&ChangeInput>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut actions: ResMut<Actions>,
) {
    for (interaction, mut color, button_colors, change_state, open_link, change_input) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    if let Some(device) = change_input {
                        match device.0 {
                            InputDevice::Gamepad => actions.input_device = InputDevice::Gamepad,
                            InputDevice::Keyboard => actions.input_device = InputDevice::Keyboard,
                        }
                    }
                    next_state.set(state.0.clone());
                } else if let Some(link) = open_link {
                    if let Err(error) = webbrowser::open(link.0) {
                        warn!("Failed to open link {error:?}");
                    }
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn cleanup_menu(
    mut commands: Commands,
    menu: Query<Entity, With<Menu>>,
) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
