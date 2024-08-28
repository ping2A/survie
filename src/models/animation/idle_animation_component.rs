use bevy::ecs::component::Component;

use serde::Deserialize;

#[derive(Default, Debug, Deserialize, Clone, Component)]
pub struct IdleAnimation {
    pub progress: f32,
    pub sprite_row_length: usize,
    pub framecount: usize,
    pub atlas_row: usize,
    pub duration: f32,
}

impl IdleAnimation {
    pub fn new(progress: f32, sprite_row_length:usize, framecount: usize, atlas_row: usize, duration: f32) -> IdleAnimation {
        IdleAnimation { progress: progress, sprite_row_length, framecount, atlas_row, duration }
    }
}