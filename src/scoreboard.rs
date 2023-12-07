use crate::GameState;
use bevy::prelude::*;

#[derive(Default)]
pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.init_resource::<Score>()
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                update_score_text.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct BeeText;

#[derive(Default, Resource)]
pub struct Score {
    pub points: f32,
    pub bees: usize,
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
    commands.spawn((
        TextBundle::from_section(
            "Worker Bees: ",
            TextStyle {
                font: default(),
                font_size: 20.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        BeeText,
    ));
}

fn update_score_text(
    score: Res<Score>,
    mut score_query: Query<&mut Text, (With<ScoreText>, Without<BeeText>)>,
    mut bee_query: Query<&mut Text, (With<BeeText>, Without<ScoreText>)>,
) {
    let points = score.points;
    for mut text in &mut score_query {
        text.sections[0].value = format!("Score: {points:.0}");
    }

    let bees = score.bees;
    for mut text in &mut bee_query {
        text.sections[0].value = format!("Worker Bees: {}", bees);
    }
}
