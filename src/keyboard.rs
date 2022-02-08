use bevy::{
    
    prelude::{Query, EventReader, KeyCode, With},
    input::{keyboard::KeyboardInput, ElementState},
};

use crate::components::{Direction, Velocity, KeyboardControlled};

const PLAYER_MOVEMENT_SPEED: i32 = 1;

    pub fn keyboard_controller(

        mut keyboard_input: EventReader<KeyboardInput>,
        mut query: Query<&mut Velocity, With<KeyboardControlled>>,
    )

        {
            let mut vel = query.single_mut();
            
            for ev in keyboard_input.iter() {
                match ev.state {
                    ElementState::Pressed => {
                        match ev.key_code.unwrap() {
                            KeyCode::Up => 
                            {
                                vel.speed = PLAYER_MOVEMENT_SPEED;
                                vel.direction = Direction::Up;
                            }
                            KeyCode::Down => 
                            {
                                vel.speed = PLAYER_MOVEMENT_SPEED;
                                vel.direction = Direction::Down;
                            }
                            KeyCode::Left => 
                            {
                                vel.speed = PLAYER_MOVEMENT_SPEED;
                                vel.direction = Direction::Left;
                            }
                            KeyCode::Right => 
                            {
                                vel.speed = PLAYER_MOVEMENT_SPEED;
                                vel.direction = Direction::Right;
                            }
                            _ => {}
                        }
                    }
                    ElementState::Released => {
                        match ev.key_code.unwrap() {
                            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {vel.speed = 0;}
                            _ => {}
                        } 
                    }
                }
            }
            /* Old
            //Up, Down, Left, Right
            if keyboard_input.pressed(KeyCode::Up) {
                vel.speed = PLAYER_MOVEMENT_SPEED;
                vel.direction = Direction::Up;
            }
            if keyboard_input.pressed(KeyCode::Down) {
                vel.speed = PLAYER_MOVEMENT_SPEED;
                vel.direction = Direction::Down;
            }
            if keyboard_input.pressed(KeyCode::Left) {
                vel.speed = PLAYER_MOVEMENT_SPEED;
                vel.direction = Direction::Left;
            }
            if keyboard_input.pressed(KeyCode::Right) {
                vel.speed = PLAYER_MOVEMENT_SPEED;
                vel.direction = Direction::Right;
            }

            //RELEASED - Up, Down, Left, Right
                        if keyboard_input.just_released(KeyCode::Up) {
                            vel.speed = 0;
                        }
                        if keyboard_input.just_released(KeyCode::Down) {
                            vel.speed = 0;
                        }
                        if keyboard_input.just_released(KeyCode::Left) {
                            vel.speed = 0;
                        }
                        if keyboard_input.just_released(KeyCode::Right) {
                            vel.speed = 0;
                        }
            */
        }