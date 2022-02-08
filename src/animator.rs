use bevy::prelude::*;

use crate::components::{Direction, Velocity, *};

const TICKS_TO_ANIMATE: u64 = 1;

    pub fn animation_system(
        mut query: Query<(&mut Velocity, &mut TextureAtlasSprite)>,
        mut ticks_q: Query<&mut Ticks>,
    ) {
        for (mut vel, mut sprite) in query.iter_mut() {

            use self::Direction::*;

            let mut dir_change = false;

            let mut animate = false;

            let mut ticks = ticks_q.single_mut();

            if vel.direction != vel.last_dir {dir_change = true; animate = true;}

            else if vel.last_anim + TICKS_TO_ANIMATE <= ticks.total_ticks {animate = true;}

            else {animate = false;}
            
            if animate {
            match vel.speed {
                0 => {}
                _ => {
                    match vel.direction {
                    Left => {
                        if dir_change == true {sprite.index = 3}

                        else if sprite.index == 5 || sprite.index < 3  {sprite.index = 3; }
                        else {sprite.index += 1;}

                        vel.last_anim = ticks.total_ticks.clone();

                    },
                    Right => {
                        if dir_change == true {sprite.index = 6}

                        else if sprite.index == 8 || sprite.index < 6 {sprite.index = 6; }
                        else {sprite.index += 1;}

                        vel.last_anim = ticks.total_ticks.clone();
                    },
                    Up => {
                        if dir_change == true {sprite.index = 9}

                        if sprite.index == 11 || sprite.index < 9 {sprite.index = 9; }
                        else {sprite.index += 1;}

                        vel.last_anim = ticks.total_ticks.clone();
                    },
                    Down => {
                        if dir_change == true {sprite.index = 0}

                        if sprite.index == 2 {sprite.index = 0; }
                        else {sprite.index += 1;}

                        vel.last_anim = ticks.total_ticks.clone();
                        },

                };

            }
            }
        }

        vel.last_dir = vel.direction.clone();
    }

        }
