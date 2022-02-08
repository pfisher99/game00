use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    core::FixedTimestep,
    math::Quat,
    prelude::*,
};

//mod spawner;
mod components;
mod keyboard;
mod physics;
mod animator;
mod ai;

use crate::keyboard::*;
use crate::physics::*;
use crate::animator::*;
use crate::ai::*;
use components::{Direction, *};

//const TIME_STEP: f32 = 1.0 / 60.0;
const TICK_SPEED_SECS: f32 = 0.5;

    fn main() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_startup_system(setup)
            .add_system(tick_system)
            .add_system_set(
                SystemSet::new()
                    //.with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(keyboard_controller.label("Keyboard"))
                    .with_system(physics_system.after("Keyboard"))
                    .with_system(animation_system.after("Keyboard"))
                    .with_system(ai_system)
                    .with_system(collision_system)
                    //.with_system(ball_movement_system),
            )
            .add_system(bevy::input::system::exit_on_esc_system)
            //.add_system(hello_world)
            //.add_system(greet_people)
            .run();
    }

    fn setup(mut commands: Commands, assets: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {

    
    // Spawns the camera, timer, and ticker
    commands
    .spawn()
    .insert_bundle(OrthographicCameraBundle::new_2d())
    .insert(Timer::from_seconds(TICK_SPEED_SECS, true))
    .insert(Ticks {total_ticks: 0, last_spawn: 0, last_ai: 0})
    .insert(Transform::from_xyz(0.0, 0.0, 1000.0));
    
    //Loads only Sprite for now
    let texture_handle: Handle<Image> = assets.load("bardo.png");

    //Refactor Later
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(26.0, 36.0), 3, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    
    commands.spawn().insert(CharacterAssets {texture_Bardo: texture_atlas_handle.clone()});

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert(KeyboardControlled)
        .insert(Velocity {speed: 0, direction: Direction::Right, last_dir: Direction::Right, last_anim: 0})
        .insert(Position(Vec3::new(0.0, 0.0, 1.0)));


    
    }
fn tick_system(
    mut query: Query<(&mut Timer, &mut Ticks)>,
    time: Res<Time>
)
{
    let (mut timer, mut ticks) = query.single_mut();

    timer.tick(time.delta());

    if timer.just_finished() {
        ticks.total_ticks += 1;
    }
}