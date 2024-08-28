use bevy::prelude::*;

use crate::AppState;
use crate::world::background::BackgroundPlugin;
use crate::world::drops::DropsPlugin;
use crate::world::game_time_system::game_time_system;
use crate::world::spawner::SpawnerPlugin;

pub mod background;
pub mod drops;
pub mod spawner;
mod game_time_system;

pub struct WorldPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct WorldSystemSet;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            WorldSystemSet
                .run_if(in_state(AppState::InGame))
        );

        app
            .add_plugins(DropsPlugin)
            .add_plugins(BackgroundPlugin)
            .add_plugins(SpawnerPlugin);

        app.add_systems(Update, game_time_system.in_set(WorldSystemSet));
    }
}