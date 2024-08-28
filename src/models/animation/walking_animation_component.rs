use bevy::ecs::component::Component;

use serde::Deserialize;

#[derive(Component, Default, Clone,  Debug, Deserialize)]
pub struct MoveAnimationSide {
    pub progress: f32,
    pub sprite_row_length: usize,
    pub framecount: usize,
    pub atlas_row: usize,
    pub gameframes_until_loop: i32,
}

#[derive(Component)]
pub struct MoveAnimationUp {
    pub progress: f32,
    pub sprite_row_length: usize,
    pub framecount: usize,
    pub atlas_row: usize,
    pub gameframes_until_loop: i32,
}

#[derive(Component)]
pub struct MoveAnimationDown {
    pub progress: f32,
    pub sprite_row_length: usize,
    pub framecount: usize,
    pub atlas_row: usize,
    pub gameframes_until_loop: i32,
}

impl MoveAnimationSide {
    pub fn new(progress: f32, sprite_row_length:usize, framecount: usize, atlas_row: usize, gameframes: i32) -> Self {
        Self { progress: progress, sprite_row_length: sprite_row_length, framecount: framecount, atlas_row: atlas_row, gameframes_until_loop: gameframes }
    }
}

impl MoveAnimationUp {
    pub fn new(progress: f32, sprite_row_length:usize, framecount: usize, atlas_row: usize, gameframes: i32) -> Self {
        Self { progress: progress, sprite_row_length: sprite_row_length, framecount: framecount, atlas_row: atlas_row, gameframes_until_loop: gameframes }
    }
}

impl MoveAnimationDown {
    pub fn new(progress: f32, sprite_row_length:usize, framecount: usize, atlas_row: usize, gameframes: i32) -> Self {
        Self { progress: progress, sprite_row_length: sprite_row_length,  framecount: framecount, atlas_row: atlas_row, gameframes_until_loop: gameframes }
    }
}