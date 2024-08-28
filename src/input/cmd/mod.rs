use bevy::prelude::*;

use crate::ConsoleState;

use crate::input::cmd::add_gold_command::{add_gold_command, push_gold_info};
use crate::input::cmd::apply_mod_command::{apply_mod_command, push_apply_mod_info};
use crate::input::cmd::god_mode_command::{god_mode_command, push_god_mode_info};
use crate::input::cmd::open_shop_command::{open_shop_command, push_open_shop_info};
use crate::input::cmd::remove_mod_command::{push_remove_mod_info, remove_mod_command};
use crate::input::cmd::toggle_spawner_command::{push_toggle_spawner_info, toggle_spawner_command};
use crate::input::cmd::list_mod_command::{push_list_mod_info, list_mod_command};
use crate::models::events::debug_command_event::DebugCommandEvent;

mod apply_mod_command;
mod god_mode_command;
mod remove_mod_command;
mod open_shop_command;
mod add_gold_command;
mod toggle_spawner_command;
mod list_mod_command;

pub struct CmdCommandsPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CmdCommandsSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CmdInputSystemSet;

impl Plugin for CmdCommandsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            CmdCommandsSystemSet
                .run_if(on_event::<DebugCommandEvent>())
        );

        app.configure_sets(
            Update,
            CmdInputSystemSet
                .run_if(in_state(ConsoleState::Shown))
        );

        app
            .add_systems(Update, apply_mod_command.in_set(CmdCommandsSystemSet))
            .add_systems(Update, remove_mod_command.in_set(CmdCommandsSystemSet))
            .add_systems(Update, god_mode_command.in_set(CmdCommandsSystemSet))
            .add_systems(Update, open_shop_command.in_set(CmdCommandsSystemSet))
            .add_systems(Update, add_gold_command.in_set(CmdCommandsSystemSet))
            .add_systems(Update, toggle_spawner_command.in_set(CmdCommandsSystemSet))
            .add_systems(Update, list_mod_command.in_set(CmdCommandsSystemSet));

        app
            .add_systems(Update, push_gold_info.in_set(CmdCommandsSystemSet))
            .add_systems(Update, push_apply_mod_info.in_set(CmdCommandsSystemSet))
            .add_systems(Update, push_god_mode_info.in_set(CmdCommandsSystemSet))
            .add_systems(Update, push_open_shop_info.in_set(CmdCommandsSystemSet))
            .add_systems(Update, push_remove_mod_info.in_set(CmdCommandsSystemSet))
            .add_systems(Update, push_toggle_spawner_info.in_set(CmdCommandsSystemSet))
            .add_systems(Update, push_list_mod_info.in_set(CmdCommandsSystemSet));


    }
}