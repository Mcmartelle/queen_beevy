mod game_control;

use bevy::{
    input::gamepad::{
        GamepadButton,
    },
    prelude::*,
};

// use crate::player::Player;
use crate::GameState;


pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_systems(
            Update,
            (
                gamepad_system.run_if(in_state(GameState::Playing)),
            ),
        );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub p1_movement: Vec2,
    pub p1_bee_movement: Vec2,
    pub p2_movement: Vec2,
    pub p1_queen_died: bool,
    pub p2_queen_died: bool,
    pub worker_bee_died: bool,
}

pub fn gamepad_system(
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    button_axes: Res<Axis<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
) {
    for gamepad in gamepads.iter() {
        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
            info!("{:?} just pressed South", gamepad);
        } else if button_inputs.just_released(GamepadButton::new(gamepad, GamepadButtonType::South))
        {
            info!("{:?} just released South", gamepad);
        }

        let right_trigger = button_axes
            .get(GamepadButton::new(
                gamepad,
                GamepadButtonType::RightTrigger2,
            ))
            .unwrap();
        if right_trigger.abs() > 0.01 {
            info!("{:?} RightTrigger2 value is {}", gamepad, right_trigger);
        }

        let left_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        if left_stick_x.abs() > 0.01 {
            info!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
        }
        let left_stick_y = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            .unwrap();
        if left_stick_y.abs() > 0.01 {
            info!("{:?} LeftStickX value is {}", gamepad, left_stick_y);
        }
    }
}