use bevy::prelude::*;

use crate::AppState;

use crate::world::drops::coin_pickup_system::coin_pickup_system;
use crate::world::drops::drop_chance_system::drop_chance_system;

mod drop_chance_system;
mod coin_pickup_system;

/// this plugin manages the spawning and collection of items during the game.
///
/// [ basic_drop_system ] controls probability and collection of possible drops
/// whenever an enemy dies
///
/// [ coin_pickup_system ] and [ barrel_pickup_system ]
/// handle the event when an item is picked up and execute the responding action.
pub struct DropsPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct DropsEnterShopSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct DropsUpdateSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct DropsPreUpdateSystemSet;

impl Plugin for DropsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            OnEnter(AppState::InGame),
            DropsEnterShopSystemSet
        );

        app.configure_sets(
            Update, 
            DropsUpdateSystemSet
                .run_if(in_state(AppState::InGame))
        );

        app.configure_sets(
            PreUpdate,
            DropsPreUpdateSystemSet
                .run_if(in_state(AppState::InGame))
        );


        app
            .add_systems(Update, drop_chance_system.in_set(DropsUpdateSystemSet));

        app
            .add_systems(PreUpdate, coin_pickup_system.in_set(DropsPreUpdateSystemSet));
    }
}