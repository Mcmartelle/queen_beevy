mod game_control;

use bevy::{
    input::gamepad::{
        GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadButtonInput,
        GamepadConnectionEvent, GamepadEvent, GamepadConnection
    },
    prelude::*,
};

use crate::actions::game_control::{P1Control, P2Control, get_p1_movement, get_p2_movement};
// use crate::player::Player;
use crate::GameState;


pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_systems(
            Update,
            gamepad_events.run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub p1_movement: Option<Vec2>,
    pub p1_bee_movement: Option<Vec2>,
    pub p2_movement: Option<Vec2>,
    pub p1_queen_died: bool,
    pub p2_queen_died: bool,
    pub worker_bee_died: bool,
}

#[derive(Resource)]
struct P1Gamepad(Gamepad);

#[derive(Resource)]
struct P2Gamepad(Gamepad);

fn gamepad_events(
    mut connection_events: EventReader<GamepadConnectionEvent>,
    mut axis_changed_events: EventReader<GamepadAxisChangedEvent>,
    // Handles the continuous measure of how far a button has been pressed down, as measured
    // by `Axis<GamepadButton>`. Whenever that value changes, this event is emitted.
    mut button_changed_events: EventReader<GamepadButtonChangedEvent>,
    // Handles the boolean measure of whether a button is considered pressed or unpressed, as
    // defined by the thresholds in `GamepadSettings::button_settings` and measured by
    // `Input<GamepadButton>`. When the threshold is crossed and the button state changes,
    // this event is emitted.
    mut button_input_events: EventReader<GamepadButtonInput>,
    mut commands: Commands,
    mut actions: ResMut<Actions>,
    p1_gamepad: Option<Res<P1Gamepad>>,
    p2_gamepad: Option<Res<P2Gamepad>>,
) {
    for connection_event in connection_events.read() {
        info!("{:?}", connection_event);
        let id = connection_event.gamepad;
        match &connection_event.connection {
            GamepadConnection::Connected(info) => {
                println!("New gamepad connected with ID: {:?}, name: {}", id, info.name);

                // if we don't have any gamepad yet, use this one
                if p1_gamepad.is_none() {
                    commands.insert_resource(P1Gamepad(id));
                } else if p2_gamepad.is_none() {
                    commands.insert_resource(P2Gamepad(id));
                }
            }
            GamepadConnection::Disconnected => {
                println!("Lost gamepad connection with ID: {:?}", id);

                // if it's the one we previously associated with the player,
                // disassociate it:
                if let Some(P1Gamepad(old_id)) = p1_gamepad.as_deref() {
                    if *old_id == id {
                        commands.remove_resource::<P1Gamepad>();
                    }
                }
                if let Some(P2Gamepad(old_id)) = p2_gamepad.as_deref() {
                    if *old_id == id {
                        commands.remove_resource::<P2Gamepad>();
                    }
                }
            }
            // other events are irrelevant
            _ => {}
        }
    }
    for axis_changed_event in axis_changed_events.read() {
        info!(
            "{:?} of {:?} is changed to {}",
            axis_changed_event.axis_type, axis_changed_event.gamepad, axis_changed_event.value
        );
        if let Some(P1Gamepad(p1_gamepad_id)) = p1_gamepad.as_deref() {
            if *p1_gamepad_id == axis_changed_event.gamepad {
                match axis_changed_event.axis_type {
                    GamepadAxisType::LeftStickX => {
                        match actions.p1_movement {
                            Some(mut vec_2) => vec_2.x = axis_changed_event.value,
                            None => {
                                actions.p1_movement = Some(Vec2::new(axis_changed_event.value, 0.0))
                            },
                        }
                    },
                    GamepadAxisType::LeftStickY => {
                        match actions.p1_movement {
                            Some(mut vec_2) => vec_2.y = axis_changed_event.value,
                            None => {
                                actions.p1_movement = Some(Vec2::new(0.0, axis_changed_event.value))
                            },
                        }
                    },
                    GamepadAxisType::LeftZ => {

                    },
                    GamepadAxisType::RightStickX => {
                        match actions.p1_bee_movement {
                            Some(mut vec_2) => vec_2.x = axis_changed_event.value,
                            None => {
                                actions.p1_bee_movement = Some(Vec2::new(axis_changed_event.value, 0.0))
                            },
                        }
                    },
                    GamepadAxisType::RightStickY => {
                        match actions.p1_bee_movement {
                            Some(mut vec_2) => vec_2.y = axis_changed_event.value,
                            None => {
                                actions.p1_bee_movement = Some(Vec2::new(0.0, axis_changed_event.value))
                            },
                        }
                    },
                    GamepadAxisType::RightZ => {

                    },
                    GamepadAxisType::Other(u8) => {

                    },
                }
            }
        }
    }

    // if let Some(p1_movement) = actions.p1_movement {
    //     actions.p1_movement = Some(p1_movement.normalize()); // Assuming double normalizing won't hurt keyboard input
    // }

    // if let Some(p1_bee_movement) = actions.p1_bee_movement {
    //     actions.p1_bee_movement = Some(p1_bee_movement.normalize()); // Assuming double normalizing won't hurt keyboard input
    // }


    for button_changed_event in button_changed_events.read() {
        info!(
            "{:?} of {:?} is changed to {}",
            button_changed_event.button_type,
            button_changed_event.gamepad,
            button_changed_event.value
        );
    }
    for button_input_event in button_input_events.read() {
        info!("{:?}", button_input_event);
    }
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
