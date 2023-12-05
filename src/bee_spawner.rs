use bevy::prelude::*;

use crate::GameState;
use crate::bees::{Production, Player, TeamColors, Layer, P1Bee};
use crate::loading::TextureAssets;
use bevy_xpbd_2d::{math::*, prelude::*};

pub struct BeeSpawnerPlugin;

impl Plugin for BeeSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_worker_bee.run_if(in_state(GameState::Playing)));    
    }
}

fn spawn_worker_bee(
    mut commands: Commands,
    mut time_produced: Local<f32>,
    time: Res<Time>,
    queen_query: Query<(&Transform, &Production), With<Player>>,
    textures: Res<TextureAssets>,
    team_colors: Res<TeamColors>
) {

    for (transform, production) in &queen_query {
        if (time.elapsed_seconds() - *time_produced) > (1.0 / production.0) {
            *time_produced = time.elapsed_seconds();
            commands.spawn((
                SpriteBundle {
                    texture: textures.bee.clone(),
                    transform: transform.clone(),
                    sprite: Sprite {
                        color: team_colors.p1_color,
                        ..default()
                    },
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::ball(7.0 as Scalar),
                CollisionLayers::new([Layer::Red], [Layer::Blue]),
                P1Bee,
            ));
        }
    } 

}