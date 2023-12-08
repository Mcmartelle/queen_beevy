#![allow(clippy::unnecessary_cast)]

use crate::actions::{gamepad_system, Actions};
use crate::bee_spawner::BeeSpawnerPlugin;
use crate::loading::TextureAssets;
use crate::scoreboard::Score;
use crate::GameState;
use bevy::prelude::*;
use bevy_xpbd_2d::{math::*, prelude::*};

pub struct BeesPlugin;

impl Plugin for BeesPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_plugins(BeeSpawnerPlugin)
            .add_plugins(PhysicsPlugins::default())
            .insert_resource(ClearColor(Color::rgb(0.161, 0.678, 1.0)))
            .insert_resource(SubstepCount(6))
            .insert_resource(Gravity(Vector::ZERO))
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                (
                    queen_bee_movement
                        .after(gamepad_system)
                        .run_if(in_state(GameState::Playing)),
                    worker_bee_movement
                        .after(gamepad_system)
                        .run_if(in_state(GameState::Playing)),
                ),
            )
            .add_systems(
                PostProcessCollisions,
                flower_collision.run_if(in_state(GameState::Playing)),
            );
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

#[derive(Component)]
pub struct Flower;

fn setup(
    mut commands: Commands,
    textures: Res<TextureAssets>,
) {
    // commands.spawn(Camera2dBundle::default());

    let square_sprite = Sprite {
        color: Color::rgb(0.7, 0.7, 0.8),
        custom_size: Some(Vec2::splat(50.0)),
        ..default()
    };

    // Queen Bee
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

    // Flower
    commands.spawn((
        SpriteBundle {
            texture: textures.flower.clone(),
            transform: Transform::from_translation(Vec3::new(350., 5., 1.)),
            ..Default::default()
        },
        RigidBody::Kinematic,
        Collider::ball(20.0 as Scalar),
        AngularVelocity(1.5),
        Flower,
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
            transform: Transform::from_xyz(0.0, -53.0 * 6.0, 0.0)
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
            transform: Transform::from_xyz(-44.0 * 9.5, 0.0, 0.0)
                .with_scale(Vec3::new(1.0, 12.0, 1.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(50.0, 50.0),
    ));
    // Right wall
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite,
            transform: Transform::from_xyz(44.0 * 9.5, 0.0, 0.0)
                .with_scale(Vec3::new(1.0, 12.0, 1.0)),
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
    mut bees: Query<(&mut LinearVelocity, &mut Sprite), With<QueenBee>>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();

    for (mut linear_velocity, mut sprite) in &mut bees {
        linear_velocity.x += actions.p1_movement.x * QUEEN_MOVEMENT_SCALING_X * delta_time;
        linear_velocity.y += actions.p1_movement.y * QUEEN_MOVEMENT_SCALING_Y * delta_time;
        sprite.flip_x = actions.p1_movement.x < 0.0;
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

pub fn flower_collision(
    mut actions: ResMut<Actions>,
    mut score: ResMut<Score>,
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut queen_query: Query<(&mut Production, Entity), With<QueenBee>>,
    mut flower_query: Query<(&mut Transform, Entity), With<Flower>>,
    time: Res<Time>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        let mut flower_gotten: bool = false;

        if let Ok(_) = queen_query.get(*entity1) {
            if let Ok(_) = flower_query.get(*entity2) {
                flower_gotten = true;
            }
        } else if let Ok(_) = queen_query.get(*entity2) {
            if let Ok(_) = flower_query.get(*entity1) {
                flower_gotten = true;
            }
        }

        if flower_gotten {
            score.points = score.points + 1.0;
            actions.flower_gotten = true;
            for (mut transform, _) in &mut flower_query {
                let randomish = 10000.0 * time.elapsed_seconds();
                transform.translation.x =
                    randomish % 370.0 * (-(randomish % 2.0) * 2.0 + 1.0).clamp(-1.0, 1.0);
                transform.translation.y =
                    randomish % 210.0 * (-(randomish % 3.0) * 2.0 + 3.0).clamp(-1.0, 1.0);
            }
            for (mut production, _) in &mut queen_query {
                production.0 += 1.0;
            }
            break;
        }
    }
}
