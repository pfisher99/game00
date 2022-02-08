use bevy::prelude::*;

const AI_MOVEMENT_SPEED: i32 = 10;

pub struct Spawner;

fn direction_spritesheet_row(direction: Direction) -> i32 {
    use self::Direction::*;
    match direction {
        Up => 3,
        Down => 0,
        Left => 1,
        Right => 2,
    }
}

/// Create animation frames for the standard character spritesheet
pub fn character_spritesheetbundler(texture_handle: Handle<Image>) -> SpriteSheetBundle {
    //SpriteSheetBundle {}

    
}

pub fn create_player() {

}