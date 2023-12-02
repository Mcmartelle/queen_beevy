use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_xpbd_2d::prelude::*;
use crate::GameState;

#[derive(Default)]
pub struct BeesHelperPlugin;

impl Plugin for BeesHelperPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default())
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, update_fps_text.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
struct FpsText;

fn setup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "FPS: ",
            TextStyle {
                font: default(),
                font_size: 20.0,
                color: Color::TOMATO,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        FpsText,
    ));
}

fn update_fps_text(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[0].value = format!("FPS: {value:.2}");
            }
        }
    }
}