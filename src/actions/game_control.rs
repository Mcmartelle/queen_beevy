use bevy::prelude::{Input, KeyCode, Res};

pub enum P1Control {
    Up,
    Down,
    Left,
    Right,
}

pub enum P2Control {
    Up,
    Down,
    Left,
    Right,
}

impl P1Control {
    pub fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            P1Control::Up => {
                keyboard_input.pressed(KeyCode::W)
            }
            P1Control::Down => {
                keyboard_input.pressed(KeyCode::S)
            }
            P1Control::Left => {
                keyboard_input.pressed(KeyCode::A)
            }
            P1Control::Right => {
                keyboard_input.pressed(KeyCode::D)
            }
        }
    }
}

impl P2Control {
    pub fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            P2Control::Up => {
                keyboard_input.pressed(KeyCode::Up)
            }
            P2Control::Down => {
                keyboard_input.pressed(KeyCode::Down)
            }
            P2Control::Left => {
                keyboard_input.pressed(KeyCode::Left)
            }
            P2Control::Right => {
                keyboard_input.pressed(KeyCode::Right)
            }
        }
    }
}

pub fn get_p1_movement(control: P1Control, input: &Res<Input<KeyCode>>) -> f32 {
    if control.pressed(input) {
        1.0
    } else {
        0.0
    }
}

pub fn get_p2_movement(control: P2Control, input: &Res<Input<KeyCode>>) -> f32 {
    if control.pressed(input) {
        1.0
    } else {
        0.0
    }
}
