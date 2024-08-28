use bevy::prelude::*;

use crate::{App, AppState};

use crate::units::projectile::projectile_check_stop_system::projectile_check_stop_system;
use crate::units::projectile::projectile_despawn_system::projectile_despawn_system;

mod projectile_check_stop_system;
mod projectile_despawn_system;

/// This plugin manages the [Projectile] systems and how they get applied.
///
/// [projectile_hit_system] gets run in the [on_pre_update] stack because it is a system that
/// reacts directly to the collision systems
///
/// [projectile_check_stop_system] gets run in the [on_update] stack
///
/// [projectile_despawn_system] gets run in the [on_last] stack because the app panics if
/// you try access a despawned entity
///
/// All system get only used in the [AppState::InGame].
pub struct ProjectilePlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ProjectileUpdateSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ProjectileCleanupSystemSet;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            ProjectileUpdateSystemSet
                .run_if(in_state(AppState::InGame))
        );

        app.configure_sets(
            Last,
            ProjectileCleanupSystemSet
                .run_if(in_state(AppState::InGame))
        );

        app.add_systems(Update, projectile_check_stop_system.in_set(ProjectileUpdateSystemSet));

        app.add_systems(Last, projectile_despawn_system.in_set(ProjectileCleanupSystemSet));
    }
}