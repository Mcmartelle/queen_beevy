#![allow(clippy::unnecessary_cast)]

use crate::bee_spawner::BeeSpawnerPlugin;
use crate::loading::TextureAssets;
use crate::GameState;
use crate::actions::{Actions, gamepad_system};
use bevy::prelude::*;
use bevy_xpbd_2d::{math::*, prelude::*};

pub struct BeesPlugin;

impl Plugin for BeesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BeeSpawnerPlugin)
            .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .insert_resource(SubstepCount(6))
            .insert_resource(Gravity(Vector::ZERO))
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, (
                queen_bee_movement.after(gamepad_system).run_if(in_state(GameState::Playing)),
                worker_bee_movement.after(gamepad_system).run_if(in_state(GameState::Playing)),
            ));
            // .add_systems(PostProcessCollisions, (apply_deferred.run_if(in_state(GameState::Playing)),death_collisions.run_if(in_state(GameState::Playing))).chain());
    }
}

#[derive(Component)]
pub struct QueenBee;

#[derive(Component)]
pub struct Production(pub f32);

#[derive(Component)]
pub struct WorkerBee;

#[derive(Component)]
pub struct Wall;


fn setup(mut commands: Commands, textures: Res<TextureAssets>) {
    // commands.spawn(Camera2dBundle::default());

    let square_sprite = Sprite {
        color: Color::rgb(0.7, 0.7, 0.8),
        custom_size: Some(Vec2::splat(50.0)),
        ..default()
    };


    commands.spawn((
        SpriteBundle {
            texture: textures.queen.clone(),
            transform: Transform::from_translation(Vec3::new(-350., 0., 1.)),
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::ball(30.0 as Scalar),
        LockedAxes::ROTATION_LOCKED,
        Production(1.0),
        QueenBee,
    ));

    // Ceiling
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite.clone(),
            transform: Transform::from_xyz(0.0, 50.0 * 6.0, 0.0)
                .with_scale(Vec3::new(20.0, 1.0, 1.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(50.0, 50.0),
    ));
    // Floor
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite.clone(),
            transform: Transform::from_xyz(0.0, -50.0 * 6.0, 0.0)
                .with_scale(Vec3::new(20.0, 1.0, 1.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(50.0, 50.0),
    ));
    // Left wall
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite.clone(),
            transform: Transform::from_xyz(-50.0 * 9.5, 0.0, 0.0)
                .with_scale(Vec3::new(1.0, 11.0, 1.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(50.0, 50.0),
    ));
    // Right wall
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite,
            transform: Transform::from_xyz(50.0 * 9.5, 0.0, 0.0)
                .with_scale(Vec3::new(1.0, 11.0, 1.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(50.0, 50.0),
    ));
    
}


const QUEEN_MOVEMENT_SCALING_X: f32 = 700.0;
const QUEEN_MOVEMENT_SCALING_Y: f32 = 700.0;
const BEE_MOVEMENT_SCALING_X: f32 = 1000.0;
const BEE_MOVEMENT_SCALING_Y: f32 = 1000.0;

fn queen_bee_movement(
    time: Res<Time>,
    actions: Res<Actions>,
    mut bees: Query<&mut LinearVelocity, With<QueenBee>>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();

    for mut linear_velocity in &mut bees {
        linear_velocity.x += actions.p1_movement.x * QUEEN_MOVEMENT_SCALING_X * delta_time;
        linear_velocity.y += actions.p1_movement.y * QUEEN_MOVEMENT_SCALING_Y * delta_time;
    }
}

fn worker_bee_movement(
    time: Res<Time>,
    actions: Res<Actions>,
    mut bees: Query<&mut LinearVelocity, With<WorkerBee>>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();

    for mut linear_velocity in &mut bees {
        linear_velocity.x += actions.p1_bee_movement.x * BEE_MOVEMENT_SCALING_X * delta_time;
        linear_velocity.y += actions.p1_bee_movement.y * BEE_MOVEMENT_SCALING_Y * delta_time;
    }
}

// pub fn death_collisions(
//     // mut commands: Commands,
//     mut actions: ResMut<Actions>,
//     mut collision_event_reader: EventReader<CollisionStarted>,
//     queen_query: Query<Entity, With<QueenBee>>,
//     // mut time: ResMut<Time<Physics>>,
// ) {
    
//     // actions.p1_queen_died = false; // maybe set this to false on game/round restart
//     // actions.p2_queen_died = false; // maybe set this to false on game/round restart
//     actions.worker_bee_died = false;

//     for CollisionStarted(entity1, entity2) in collision_event_reader.read() {

//         let mut queen_just_died: bool = false;
        
//         if let Ok(_) = p1_queen_query.get(*entity1) {
//             actions.p1_queen_died = true;
//             queen_just_died = true;
//             println!("Player 2 Wins!!!");
//             // time.pause();
//         } else if let Ok(_) = p1_queen_query.get(*entity2) {
//             actions.p1_queen_died = true;
//             queen_just_died = true;
//             println!("Player 2 Wins!!!");
//             // time.pause();
//         }
        
//         if let Ok(_) = p2_queen_query.get(*entity1) {
//             actions.p2_queen_died = true;
//             queen_just_died = true;
//             println!("Player 1 Wins!!!");
//             // time.pause();
//         } else if let Ok(_) = p2_queen_query.get(*entity2) {
//             actions.p2_queen_died = true;
//             queen_just_died = true;
//             println!("Player 1 Wins!!!");
//             // time.pause();
//         }

//         if !queen_just_died {
//             // worker bees dying
//             // commands.entity(*entity1).despawn();
//             // commands.entity(*entity2).despawn();
//             actions.worker_bee_died = true;
//         }

//     }
// }

