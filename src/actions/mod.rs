// use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

use crate::actions::game_control::{P1Control, P2Control, get_p1_movement, get_p2_movement};
// use crate::player::Player;
use crate::GameState;

mod game_control;

pub const FOLLOW_EPSILON: f32 = 5.;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_systems(
            Update,
            set_movement_actions.run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub p1_movement: Option<Vec2>,
    pub p2_movement: Option<Vec2>,
    pub p1_queen_died: bool,
    pub p2_queen_died: bool,
    pub worker_bee_died: bool,
}

pub fn set_movement_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<Input<KeyCode>>,
    // touch_input: Res<Touches>,
    // player: Query<&Transform, With<Player>>,
    // camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let p1_movement = Vec2::new(
        get_p1_movement(P1Control::Right, &keyboard_input)
            - get_p1_movement(P1Control::Left, &keyboard_input),
        get_p1_movement(P1Control::Up, &keyboard_input)
            - get_p1_movement(P1Control::Down, &keyboard_input),
    );

    let p2_movement = Vec2::new(
        get_p2_movement(P2Control::Right, &keyboard_input)
            - get_p2_movement(P2Control::Left, &keyboard_input),
        get_p2_movement(P2Control::Up, &keyboard_input)
            - get_p2_movement(P2Control::Down, &keyboard_input),
    );

    // if let Some(touch_position) = touch_input.first_pressed_position() {
    //     let (camera, camera_transform) = camera.single();
    //     if let Some(touch_position) = camera.viewport_to_world_2d(camera_transform, touch_position)
    //     {
    //         let diff = touch_position - player.single().translation.xy();
    //         if diff.length() > FOLLOW_EPSILON {
    //             player_movement = diff.normalize();
    //         }
    //     }
    // }

    if p1_movement != Vec2::ZERO {
        actions.p1_movement = Some(p1_movement.normalize());
    } else {
        actions.p1_movement = None;
    }

    if p2_movement != Vec2::ZERO {
        actions.p2_movement = Some(p2_movement.normalize());
    } else {
        actions.p2_movement = None;
    }
}
