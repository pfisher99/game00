use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Debug)]
pub struct CharacterAssets {
    pub texture_Bardo: Handle<TextureAtlas>,
}

#[derive(Component, Debug, Default)]
pub struct AIControlled;

#[derive(Component, Debug, Default)]
pub struct KeyboardControlled;

/// The current position of a given entity
#[derive(Component, Debug)]
pub struct Position(pub Vec3);

/// The current speed and direction of a given entity
#[derive(Component, Debug)]
pub struct Velocity {
    pub speed: i32,
    pub direction: Direction,
    pub last_dir: Direction,
    pub last_anim: u64,
    
}

///Game Ticks / Half Second
#[derive(Component, Debug)]
pub struct Ticks {
    pub total_ticks: u64,
    //pub last_anim: u64,
    pub last_spawn: u64,
    pub last_ai: u64,
}


#[derive(Component, Debug)]
pub struct MovementAnimation {
    // The current frame in the animation of the direction this entity is moving in
    pub sprite_number: usize,
    //pub current_frame: usize,
    //pub up_frames: Vec<usize>,
    //pub down_frames: Vec<usize>,
    //pub left_frames: Vec<usize>,
    //pub right_frames: Vec<usize>,
}
