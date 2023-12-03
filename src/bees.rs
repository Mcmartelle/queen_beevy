#![allow(clippy::unnecessary_cast)]

use crate::bees_helper::BeesHelperPlugin;
use crate::loading::TextureAssets;
use crate::GameState;
use crate::actions::Actions;
use bevy::prelude::*;
use bevy_xpbd_2d::{math::*, prelude::*};

pub struct BeesPlugin;

impl Plugin for BeesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BeesHelperPlugin)
            .insert_resource(ClearColor(Color::rgb(0.46, 0.71, 0.996)))
            .insert_resource(SubstepCount(6))
            .insert_resource(Gravity(Vector::NEG_Y * 100.0))
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, (
                p1_queen_movement.run_if(in_state(GameState::Playing)),
                p2_queen_movement.run_if(in_state(GameState::Playing)),
                p1_bee_movement.run_if(in_state(GameState::Playing)),
                p2_bee_movement.run_if(in_state(GameState::Playing))
            ));
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
        Player,
        Player1,
    ));

    commands.spawn((
        SpriteBundle {
            texture: textures.queen.clone(),
            transform: Transform::from_translation(Vec3::new(350., 0., 1.)),
            sprite: Sprite {
                flip_x: true,
                flip_y: false,
                ..default()
            },
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(30.0 as Scalar),
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

    let bee_radius = 5.0;

    // Spawn stacks of Player 1 Bees
    for x in 0..10 {
        for y in 0..10 {
            commands.spawn((
                SpriteBundle {
                    texture: textures.bee.clone(),
                    transform: Transform::from_xyz(
                        x as f32 * -5.0 * bee_radius - 50.0,
                        y as f32 * 5.0 * bee_radius - 100.0,
                        0.0,
                    ),
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::ball(bee_radius as Scalar),
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
                        x as f32 * 5.0 * bee_radius + 50.0,
                        y as f32 * 5.0 * bee_radius -100.0,
                        0.0,
                    ),
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::ball(bee_radius as Scalar),
                P2Bee,
            ));
        }
    }
}

fn p1_queen_movement(
    time: Res<Time>,
    actions: Res<Actions>,
    mut bees: Query<&mut LinearVelocity, With<Player1>>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();

    for mut linear_velocity in &mut bees {
        linear_velocity.x += actions.p1_movement.unwrap_or(Vec2::new(0.0,0.0)).x * 200.0 * delta_time;
        linear_velocity.y += actions.p1_movement.unwrap_or(Vec2::new(0.0,0.0)).y * 450.0 * delta_time;
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
        linear_velocity.x += actions.p2_movement.unwrap_or(Vec2::new(0.0,0.0)).x * 200.0 * delta_time;
        linear_velocity.y += actions.p2_movement.unwrap_or(Vec2::new(0.0,0.0)).y * 450.0 * delta_time;
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
        linear_velocity.x += actions.p1_movement.unwrap_or(Vec2::new(0.0,0.0)).x * 500.0 * delta_time;
        linear_velocity.y += actions.p1_movement.unwrap_or(Vec2::new(0.0,0.0)).y * 500.0 * delta_time;
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
        linear_velocity.x += actions.p2_movement.unwrap_or(Vec2::new(0.0,0.0)).x * 500.0 * delta_time;
        linear_velocity.y += actions.p2_movement.unwrap_or(Vec2::new(0.0,0.0)).y * 500.0 * delta_time;
    }
}
