#![allow(clippy::unnecessary_cast)]

use crate::bees_helper::BeesHelperPlugin;
use crate::loading::TextureAssets;
use crate::GameState;
use crate::actions::{Actions, gamepad_input};
use bevy::prelude::*;
use bevy_xpbd_2d::{math::*, prelude::*};

pub struct BeesPlugin;

impl Plugin for BeesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BeesHelperPlugin)
            .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .insert_resource(SubstepCount(6))
            .insert_resource(Gravity(Vector::ZERO))
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, (
                p1_queen_movement.after(gamepad_input).run_if(in_state(GameState::Playing)),
                p2_queen_movement.after(gamepad_input).run_if(in_state(GameState::Playing)),
                p1_bee_movement.after(gamepad_input).run_if(in_state(GameState::Playing)),
                p2_bee_movement.after(gamepad_input).run_if(in_state(GameState::Playing))
            ))
            .add_systems(PostProcessCollisions, (apply_deferred.run_if(in_state(GameState::Playing)),death_collisions.run_if(in_state(GameState::Playing))).chain());
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Player1;

#[derive(Component)]
pub struct Player2;

#[derive(Component)]
struct P1Bee;

#[derive(Component)]
struct P2Bee;

#[derive(Component)]
struct Wall;

fn setup(mut commands: Commands, textures: Res<TextureAssets>) {
    // commands.spawn(Camera2dBundle::default());

    let square_sprite = Sprite {
        color: Color::rgb(0.7, 0.7, 0.8),
        custom_size: Some(Vec2::splat(50.0)),
        ..default()
    };

    let p1_color = Color::rgb(7.0, 3.0, 3.0); // The Red Team vs
    let p2_color = Color::rgb(2.0, 3.0, 7.0); // The Blue Team

    // Define the collision layers
    #[derive(PhysicsLayer)]
    enum Layer {
        Blue,
        Red,
    }

    commands.spawn((
        SpriteBundle {
            texture: textures.queen.clone(),
            transform: Transform::from_translation(Vec3::new(-350., 0., 1.)),
            sprite: Sprite {
                color: p1_color,
                ..default()
            },
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::ball(30.0 as Scalar),
        CollisionLayers::new([Layer::Red], [Layer::Blue]),
        Player,
        Player1,
    ));

    commands.spawn((
        SpriteBundle {
            texture: textures.queen.clone(),
            transform: Transform::from_translation(Vec3::new(350., 0., 1.)),
            sprite: Sprite {
                color: p2_color,
                flip_x: true,
                flip_y: false,
                ..default()
            },
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(30.0 as Scalar),
        CollisionLayers::new([Layer::Blue], [Layer::Red]),
        Player,
        Player2,
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

    let bee_radius = 7.0;
    let bee_spacing = 25.0;
    // Spawn stacks of Player 1 Bees
    for x in 0..10 {
        for y in 0..10 {
            commands.spawn((
                SpriteBundle {
                    texture: textures.bee.clone(),
                    transform: Transform::from_xyz(
                        x as f32 * -1.0 * bee_spacing - 50.0,
                        y as f32 * 1.0 * bee_spacing - 100.0,
                        0.0,
                    ),
                    sprite: Sprite {
                        color: p1_color,
                        ..default()
                    },
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::ball(bee_radius as Scalar),
                CollisionLayers::new([Layer::Red], [Layer::Blue]),
                P1Bee,
            ));
        }
    }
    
    // Spawn stacks of Player 2 Bees
    for x in 0..10 {
        for y in 0..10 {
            commands.spawn((
                SpriteBundle {
                    texture: textures.bee.clone(),
                    transform: Transform::from_xyz(
                        x as f32 * bee_spacing + 50.0,
                        y as f32 * bee_spacing - 100.0,
                        0.0,
                    ),
                    sprite: Sprite {
                        color: p2_color,
                        ..default()
                    },
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::ball(bee_radius as Scalar),
                CollisionLayers::new([Layer::Blue], [Layer::Red]),
                P2Bee,
            ));
        }
    }
}

pub fn death_collisions(
    // mut commands: Commands,
    mut actions: ResMut<Actions>,
    mut collision_event_reader: EventReader<CollisionStarted>,
    p1_queen_query: Query<Entity, With<Player1>>,
    p2_queen_query: Query<Entity, With<Player2>>,
    // mut time: ResMut<Time<Physics>>,
) {
    
    // actions.p1_queen_died = false; // maybe set this to false on game/round restart
    // actions.p2_queen_died = false; // maybe set this to false on game/round restart
    actions.worker_bee_died = false;

    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {

        let mut queen_just_died: bool = false;
        
        if let Ok(_) = p1_queen_query.get(*entity1) {
            actions.p1_queen_died = true;
            queen_just_died = true;
            println!("Player 2 Wins!!!");
            // time.pause();
        } else if let Ok(_) = p1_queen_query.get(*entity2) {
            actions.p1_queen_died = true;
            queen_just_died = true;
            println!("Player 2 Wins!!!");
            // time.pause();
        }
        
        if let Ok(_) = p2_queen_query.get(*entity1) {
            actions.p2_queen_died = true;
            queen_just_died = true;
            println!("Player 1 Wins!!!");
            // time.pause();
        } else if let Ok(_) = p2_queen_query.get(*entity2) {
            actions.p2_queen_died = true;
            queen_just_died = true;
            println!("Player 1 Wins!!!");
            // time.pause();
        }

        if !queen_just_died {
            // worker bees dying
            // commands.entity(*entity1).despawn();
            // commands.entity(*entity2).despawn();
            actions.worker_bee_died = true;
        }

    }
}

const QUEEN_MOVEMENT_SCALING_X: f32 = 700.0;
const QUEEN_MOVEMENT_SCALING_Y: f32 = 700.0;
const BEE_MOVEMENT_SCALING_X: f32 = 1000.0;
const BEE_MOVEMENT_SCALING_Y: f32 = 1000.0;

fn p1_queen_movement(
    time: Res<Time>,
    actions: Res<Actions>,
    mut bees: Query<&mut LinearVelocity, With<Player1>>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();

    for mut linear_velocity in &mut bees {
        linear_velocity.x += actions.p1_movement.x * QUEEN_MOVEMENT_SCALING_X * delta_time;
        linear_velocity.y += actions.p1_movement.y * QUEEN_MOVEMENT_SCALING_Y * delta_time;
    }
}

fn p2_queen_movement(
    time: Res<Time>,
    actions: Res<Actions>,
    mut bees: Query<&mut LinearVelocity, With<Player2>>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();

    for mut linear_velocity in &mut bees {
        linear_velocity.x += actions.p2_movement.x * QUEEN_MOVEMENT_SCALING_X * delta_time;
        linear_velocity.y += actions.p2_movement.y * QUEEN_MOVEMENT_SCALING_Y * delta_time;
    }
}

fn p1_bee_movement(
    time: Res<Time>,
    actions: Res<Actions>,
    mut bees: Query<&mut LinearVelocity, With<P1Bee>>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();

    for mut linear_velocity in &mut bees {
        linear_velocity.x += actions.p1_bee_movement.x * BEE_MOVEMENT_SCALING_X * delta_time;
        linear_velocity.y += actions.p1_bee_movement.y * BEE_MOVEMENT_SCALING_Y * delta_time;
    }
}

fn p2_bee_movement(
    time: Res<Time>,
    actions: Res<Actions>,
    mut bees: Query<&mut LinearVelocity, With<P2Bee>>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();

    for mut linear_velocity in &mut bees {
        linear_velocity.x += actions.p2_movement.x * BEE_MOVEMENT_SCALING_X * delta_time;
        linear_velocity.y += actions.p2_movement.y * BEE_MOVEMENT_SCALING_Y * delta_time;
    }
}
