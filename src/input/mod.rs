use bevy::prelude::*;

use crate::AppState;
use crate::input::cmd::CmdCommandsPlugin;
use crate::input::player_control_aim_system::player_control_aim_system;
use crate::input::player_control_movement_system::player_control_movement_system;
use crate::input::setup_camera_systems::{setup_camera_system, on_zoom_changed_system};
use crate::input::toggle_pause_system::{StateTimer, toggle_pause_system};
use crate::input::update_camera_zoom_system::update_camera_zoom_system;

use crate::models::main_camera::{Zoom, ZOOM_UPPER_BOUND};

mod player_control_movement_system;
pub mod setup_camera_systems;
mod player_control_aim_system;
mod toggle_pause_system;
mod update_camera_zoom_system;
mod cmd;

/// The [StateTimer] is for the [toggle_pause_system] so that it does not trigger x times per click.
/// [toggle_pause_system] is registered in every [AppState] for now.
///
/// [setup_camera_system] is called on the exit of the [AppState::MainMenu] to handle setup timings.
///
/// Every input system is handled in the update of the [AppState::InGame]
pub struct InputPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct InGameInputSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct InputSystemSet;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Zoom(ZOOM_UPPER_BOUND));
        
        app.configure_sets(
            Update,
            InGameInputSystemSet
                .run_if(in_state(AppState::InGame))
        );

        app.configure_sets(
            Update,
            InputSystemSet
        );

        app.add_plugins(CmdCommandsPlugin);

        app.init_resource::<StateTimer>();

       // app.add_systems(Startup, setup_camera_system);
        app.add_systems(OnEnter(AppState::MainMenu), setup_camera_system);

        app.add_systems(Update, on_zoom_changed_system);
        app.add_systems(Update, update_camera_zoom_system);

        app
            .add_systems(Update, player_control_movement_system)
            .add_systems(Update, player_control_aim_system);

        app.add_systems(Update, toggle_pause_system);
    }
}