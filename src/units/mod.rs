use bevy::prelude::*;

use crate::{App, AppState};

use crate::units::player::PlayerPlugin;
use crate::units::sprite_flip_system::{sprite_atlas_flip_system, sprite_flip_system};
use crate::units::move_unit_system::move_unit_system;
use crate::units::layerable_system::layerable_system;
use crate::units::behaviors::BehaviorPlugin;
use crate::units::enemies::EnemiesPlugin;
use crate::units::health_bar_update_system::healthbar_update_system;
use crate::units::apply_damaged_component_system::apply_damage_component_system;
use crate::units::sprite_aim_rotate_system::sprite_aim_rotate_system;
use crate::units::sprite_move_rotate_system::sprite_move_rotate_system;
use crate::units::knock_back_system::knock_back_system;
use crate::units::rotate_unit_system::rotate_unit_system;
use crate::units::guns::GunPlugin;
use crate::units::projectile::ProjectilePlugin;
use crate::units::hit_system::hit_system;
use crate::units::modifications::UnitModificationsPlugin;
use crate::units::time_alive_system::time_alive_system;
use crate::units::unit_push_system::unit_push_system;
use crate::units::apply_hit_effect_system::{apply_hit_effect_sprite_atlas_system, apply_hit_effect_sprite_system};
use crate::units::clear_damaged_entities_system::clear_damaged_entities_system;
use crate::units::mirror_aim_to_move_direction_system::mirror_aim_to_move_direction_system;
use crate::units::unit_size_change_system::{unit_size_sprite_change_system, unit_size_texture_atlas_sprite_change_system};

mod behaviors;
mod player;
mod sprite_move_rotate_system;
mod sprite_aim_rotate_system;
mod sprite_flip_system;
mod move_unit_system;
mod layerable_system;
mod enemies;
mod health_bar_update_system;
mod apply_damaged_component_system;
mod apply_hit_effect_system;
mod knock_back_system;
mod rotate_unit_system;
mod projectile;
mod hit_system;
mod modifications;
mod time_alive_system;
mod unit_push_system;
mod clear_damaged_entities_system;
mod mirror_aim_to_move_direction_system;
mod unit_size_change_system;

pub mod guns;

/// This plugin manages the everything related to [Unit] systems and how they get applied.
///
/// The [PlayerPlugin] is for systems specific to all player.
/// The [EnemiesPlugin] is for systems specific to all enemies.
///
/// every system related to units overall get called in the [Update] of the [AppState::InGame].
pub struct UnitPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct UnitHitSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct UnitUpdateSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct UnitMovementSystemSet;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {        
        app.configure_sets(
            PreUpdate,
            UnitHitSystemSet
                .run_if(in_state(AppState::InGame))
        );

        app.configure_sets(
            Update,
            UnitUpdateSystemSet
                .run_if(in_state(AppState::InGame))
        );

        app.configure_sets(
            Last,
            UnitMovementSystemSet
                .run_if(in_state(AppState::InGame))
        );

        app
            .add_plugins(UnitModificationsPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(ProjectilePlugin)
            .add_plugins(EnemiesPlugin)
            .add_plugins(BehaviorPlugin)
            .add_plugins(GunPlugin);

        app.add_systems(PreUpdate, hit_system.in_set(UnitHitSystemSet));

        app
            .add_systems(Update, rotate_unit_system.in_set(UnitUpdateSystemSet))
            .add_systems(Update, sprite_flip_system.in_set(UnitUpdateSystemSet))
            .add_systems(Update, sprite_aim_rotate_system.in_set(UnitUpdateSystemSet))
            .add_systems(Update, sprite_atlas_flip_system.in_set(UnitUpdateSystemSet))
            .add_systems(Update, sprite_move_rotate_system.in_set(UnitUpdateSystemSet))
            .add_systems(Update, healthbar_update_system.in_set(UnitUpdateSystemSet))
            .add_systems(Update, apply_damage_component_system.in_set(UnitUpdateSystemSet))
            .add_systems(Update, apply_hit_effect_sprite_system.in_set(UnitUpdateSystemSet))
            .add_systems(Update, apply_hit_effect_sprite_atlas_system.in_set(UnitUpdateSystemSet))
            .add_systems(Update, clear_damaged_entities_system.in_set(UnitUpdateSystemSet))
            .add_systems(Update, mirror_aim_to_move_direction_system.in_set(UnitUpdateSystemSet))
            .add_systems(Update, unit_push_system.in_set(UnitUpdateSystemSet))
            .add_systems(Update, unit_size_sprite_change_system.in_set(UnitUpdateSystemSet))
            .add_systems(Update, unit_size_texture_atlas_sprite_change_system.in_set(UnitUpdateSystemSet))
            .add_systems(Update, time_alive_system.in_set(UnitUpdateSystemSet))
            .add_systems(Update, knock_back_system.in_set(UnitUpdateSystemSet));

        app
            .add_systems(Last, move_unit_system.in_set(UnitMovementSystemSet))
            .add_systems(Last, layerable_system.in_set(UnitMovementSystemSet));
    }
}