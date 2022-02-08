use bevy::{
    sprite::collide_aabb::{collide, Collision},
    prelude::*,
};

use crate::components::{Direction, Velocity};

    pub fn physics_system(mut query: Query<(&Velocity, &mut Transform)>) {

        use self::Direction::*;

        for (vel, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        
            match vel.direction {
                Left => {
                    translation.x += -vel.speed as f32;
                },
                Right => {
                    translation.x += vel.speed as f32;
                },
                Up => {
                    translation.y += vel.speed as f32;
                },
                Down => {
                    translation.y += -vel.speed as f32;
                },
            }
        }
    }

    pub fn collision_system(mut query: Query<(&Velocity, &mut Transform)>) {

    }