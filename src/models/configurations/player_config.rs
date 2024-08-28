use bevy::prelude::*;
use serde::Deserialize;

use crate::models::animation::idle_animation_component::IdleAnimation;
use crate::models::animation::walking_animation_component::MoveAnimationSide;

#[derive(Default, Deserialize, Debug, Clone)]
pub struct PlayerConfig {
    pub sprite_path: String,
    pub sprite_custom_size_x: f32,
    pub sprite_custom_size_y: f32,
    pub texture_atlas_grid_size: Vec2,
    pub texture_atlas_columns: usize,
    pub texture_atlas_rows: usize,
    pub move_speed: f32,
    pub damage: f32,
    pub health: f32,
    pub reload: f32,

    pub idle_animation: IdleAnimation,
    pub move_animation_side: MoveAnimationSide,
}