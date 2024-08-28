use bevy::prelude::*;

use crate::{App, AppState};
use crate::models::resources::world::active_stage::ActiveStage;

use crate::world::spawner::spawn_scheduler_system::spawn_scheduler_system;
use crate::world::spawner::spawn_worker_system::spawn_worker_system;
use crate::world::spawner::spawn_particle_system::spawn_particle_system;

mod spawn_scheduler_system;
mod spawn_worker_system;
mod spawn_particle_system;

pub struct SpawnerPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct SpawnerSystemSet;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            SpawnerSystemSet
                .run_if(in_state(AppState::InGame))
        );

        app
            .add_systems(Update, spawn_scheduler_system.in_set(SpawnerSystemSet).run_if(resource_exists::<ActiveStage>()))
            .add_systems(Update, spawn_worker_system.in_set(SpawnerSystemSet))
            .add_systems(Update, spawn_particle_system.in_set(SpawnerSystemSet));
    }
}