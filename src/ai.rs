use bevy::prelude::*;
use rand::prelude::*;

use crate::components::{Direction, Velocity, *};

const TICKS_TO_SPAWN: u64 = 10;

const AI_MOVEMENT_SPEED: i32 = 1;
const BRAIN_MOV_TICKS: u64 = 1;

pub fn ai_system(
    mut assets_q: Query<&mut CharacterAssets>,
    mut commands: Commands,
    mut ticks_q: Query<&mut Ticks>,
    mut vel_q: Query<(Option<&mut Velocity>, With<AIControlled>)>,

)
{
    let mut ticks = ticks_q.single_mut();
    let mut assets = assets_q.single_mut();
    let mut rng = rand::thread_rng();

    //Spawn New Every 10 Ticks, 5 Seconds
    if ticks.last_spawn + TICKS_TO_SPAWN <= ticks.total_ticks {
        spawn_ai(commands, assets.texture_Bardo.clone());
        info!["Spawned! Last Spawn Ticks: {}, Total Ticks: {}", ticks.last_spawn, ticks.total_ticks];
        ticks.last_spawn = ticks.total_ticks.clone();
    }


    if ticks.last_ai + BRAIN_MOV_TICKS <= ticks.total_ticks {
    for (mut vel, _) in vel_q.iter_mut() {
        if let Some(mut vel) = vel {
            vel.speed = AI_MOVEMENT_SPEED;
            let random: i32 = rng.gen_range(0..2);

            match random {
                0 => {/*info!["0!"];*/}
                1 => {vel.direction = change_direction();}
                _ => {info!["What? _"]}
            }
            
            
        }
    }
    ticks.last_ai = ticks.total_ticks.clone();
}




    
}

fn spawn_ai(mut commands: Commands, texture_atlas_handle: Handle<TextureAtlas>) {

    

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(100.0, 100.0, 1.0),
            ..Default::default()
        })
        .insert(AIControlled)
        .insert(Velocity {speed: AI_MOVEMENT_SPEED, direction: Direction::Right, last_dir: Direction::Right, last_anim: 0})
        .insert(Position(Vec3::new(50.0, 50.0, 1.0)));
}

fn change_direction() -> Direction {

    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();

    if x <= 0.25 {
        Direction::Up
    }
    else if x <= 0.5 {
        Direction::Left
    }
    else if x <= 0.75 {
        Direction::Right
    }
    else 
    {        
        Direction::Down
}
}