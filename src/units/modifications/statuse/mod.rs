use bevy::prelude::*;

use crate::{App, AppState};
use crate::scheduling::BaseSets;
use crate::units::modifications::statuse::burn_system::burn_system;
use crate::units::modifications::statuse::magnet_system::magnet_system;

mod burn_system;
mod helper;
pub mod magnet_system;

pub struct StatusPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct StatusSystemSet;

impl Plugin for StatusPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PreUpdate,
            StatusSystemSet
                .run_if(in_state(AppState::InGame))
        );

        app
            .add_systems(PreUpdate, burn_system.in_set(StatusSystemSet))
            .add_systems(PreUpdate, magnet_system.in_set(StatusSystemSet));
    }
}
