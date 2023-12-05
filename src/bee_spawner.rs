use bevy::prelude::*;

use crate::GameState;
use crate::bees::{Production, QueenBee, WorkerBee};
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
    queen_query: Query<(&Transform, &Production), With<QueenBee>>,
    textures: Res<TextureAssets>,
) {

    for (transform, production) in &queen_query {
        if (time.elapsed_seconds() - *time_produced) > (1.0 / production.0) {
            *time_produced = time.elapsed_seconds();
            commands.spawn((
                SpriteBundle {
                    texture: textures.bee.clone(),
                    transform: transform.clone(),
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::ball(7.0 as Scalar),
                WorkerBee,
            ));
        }
    } 

}