use bevy::prelude::*;
use crate::GameState;

#[derive(Default)]
pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, update_score_text.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
struct ScoreText;

#[derive(Default, Resource)]
pub struct Score{ 
    pub points: f32,
}

fn setup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Score: ",
            TextStyle {
                font: default(),
                font_size: 20.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        ScoreText,
    ));
}

fn update_score_text(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    let points = score.points;
    for mut text in &mut query {
        text.sections[0].value = format!("Score: {points:.0}");    
    }
}