use bevy::prelude::*;

use crate::models::animation::animation_state::{AnimationState, CurrentAnimationState};
use crate::models::animation::attack_animation_component::AttackAnimation;

pub fn attack_animation_system(
    time: Res<Time>,
    mut movers_query: Query<(&mut AttackAnimation, &CurrentAnimationState, &mut TextureAtlasSprite)>,
) {
    for (mut animation_data, state, mut sprite) in movers_query.iter_mut() {        
        if !matches!(state.state, AnimationState::Attack) {
            continue;
        }
        
        animation_data.progress += (time.delta_seconds() * animation_data.animation_frame_count as f32) / animation_data.duration;
        sprite.index = (animation_data.progress as usize % animation_data.animation_frame_count) + (animation_data.sprite_row_length * animation_data.atlas_row);
    }
}