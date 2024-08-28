use bevy::app::Plugin;
use bevy::prelude::*;

use crate::AppState;

use crate::animation::animation_move_down_state_handle_system::animation_move_down_state_handle_system;
use crate::animation::animation_move_idle_state_handle_system::animation_move_idle_state_handle_system;
use crate::animation::animation_move_side_state_handle_system::animation_move_side_state_handle_system;
use crate::animation::animation_move_up_state_handle_system::animation_move_up_state_handle_system;
use crate::animation::fade_animation_system::{fade_animation_sprite_system, fade_animation_texture_atlas_system};
use crate::animation::idle_animation_system::idle_animation_system;
use crate::animation::movement_animation_down_system::movement_animation_down_system;
use crate::animation::movement_animation_side_system::movement_animation_side_system;
use crate::animation::movement_animation_up_system::movement_animation_up_system;
use crate::animation::teleport_animation_system::teleport_animation_system;
use crate::animation::attack_animation_system::attack_animation_system;
use crate::animation::animation_move_attack_state_handle_system::animation_move_attack_state_handle_system;


mod movement_animation_side_system;
mod idle_animation_system;
mod movement_animation_down_system;
mod movement_animation_up_system;
mod teleport_animation_system;
mod fade_animation_system;
mod animation_move_side_state_handle_system;
mod animation_move_down_state_handle_system;
mod animation_move_up_state_handle_system;
mod animation_move_idle_state_handle_system;
mod attack_animation_system;
mod animation_move_attack_state_handle_system;

pub struct AnimationPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct AnimationSystemSet;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            AnimationSystemSet
                .run_if(in_state(AppState::InGame))
        );

        app
            .add_systems(Update,movement_animation_side_system.in_set(AnimationSystemSet))
            .add_systems(Update,movement_animation_up_system.in_set(AnimationSystemSet))
            .add_systems(Update,movement_animation_down_system.in_set(AnimationSystemSet))
            .add_systems(Update,idle_animation_system.in_set(AnimationSystemSet))
            .add_systems(Update,attack_animation_system.in_set(AnimationSystemSet))
            .add_systems(Update,teleport_animation_system.in_set(AnimationSystemSet))
            .add_systems(Update,fade_animation_sprite_system.in_set(AnimationSystemSet))
            .add_systems(Update,fade_animation_texture_atlas_system.in_set(AnimationSystemSet))
            .add_systems(Update,animation_move_idle_state_handle_system.in_set(AnimationSystemSet))
            .add_systems(Update,animation_move_attack_state_handle_system.in_set(AnimationSystemSet))
            .add_systems(Update,animation_move_down_state_handle_system.in_set(AnimationSystemSet))
            .add_systems(Update,animation_move_side_state_handle_system.in_set(AnimationSystemSet))
            .add_systems(Update,animation_move_up_state_handle_system.in_set(AnimationSystemSet));
    }
}