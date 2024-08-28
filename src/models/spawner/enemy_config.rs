use bevy::prelude::{Color, Handle, TextureAtlas, Vec2};
use crate::models::configurations::raw_configs::enemy_behavior::EnemyBehavior;

use crate::models::sprite_layer::SpriteLayer;

use crate::models::animation::idle_animation_component::IdleAnimation;
use crate::models::animation::walking_animation_component::MoveAnimationSide;

pub struct EnemyConfig {
    pub config_id: usize,
    pub entity_name: String,

    pub size: Vec2,
    pub texture_atlas: Handle<TextureAtlas>,
    pub texture_atlas_columns: usize,
    pub texture_atlas_rows: usize,
    
    pub sprite_layer: SpriteLayer,
    pub tint: Color,

    pub collider_weight: f32,

    pub base_damage: f32,
    pub damage_interval: f32,

    pub move_speed: f32,
    pub health: f32,

    pub behavior: EnemyBehavior,

    pub idle_animation: IdleAnimation,
    pub move_animation_side: MoveAnimationSide,
}