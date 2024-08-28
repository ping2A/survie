use bevy::prelude::*;

use crate::AppState;

use crate::units::player::player_died_system::player_died_system;
use crate::units::player::player_hit_system::player_hit_system;
use crate::units::player::setup_player_system::setup_player_system;
use setup_player_health_bar_system::setup_player_health_bar_system;

pub mod setup_player_system;
pub mod setup_player_health_bar_system;
pub mod player_hit_system;
pub mod player_died_system;

use crate::util::helper_systems::despawn_recursive_system::despawn_recursive_system;
use crate::models::player::Player;

/// This plugin manages the everything related to [Player] systems and how they get applied.
///
/// The setup of the player happens in the main menu, because other setup-systems depend on having
/// a player already.
///
/// The [player_hit_system] is called in the [PreUpdate] because it reacts to the [CollisionPlugin].
///
/// The [player_died_system] is called in the [Last] because it despawns the entity as a very
/// last step so that other systems can still react to the [PlayerDiedEvent].
pub struct PlayerPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PlayerSetupSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PlayerUpdateSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PlayerDiedSystemSet;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PreUpdate,
            PlayerUpdateSystemSet
                .run_if(in_state(AppState::InGame))
        );

        app.configure_sets(
            Last,
            PlayerDiedSystemSet
                .run_if(in_state(AppState::InGame))
        );

        app
            .add_systems(OnExit(AppState::MainMenu), 
                (setup_player_system, apply_deferred, setup_player_health_bar_system).chain());

        app.add_systems(PreUpdate, player_hit_system.in_set(PlayerUpdateSystemSet));

        app.add_systems(Last, player_died_system.in_set(PlayerDiedSystemSet));

    //    app.add_systems(OnEnter(AppState::GameOver), despawn_recursive_system::<Player>);
    }
}