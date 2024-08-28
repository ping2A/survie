use bevy::prelude::{Entity, EventReader, EventWriter, Query, With};

use crate::models::enemy::Enemy;
use crate::models::events::debug_command_event::DebugCommandEvent;
use crate::models::events::debug_command_info_event::DebugCommandInfoEvent;
use crate::models::modifications::descriptors::mod_name::ModName;
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::player::Player;

const KEY: &str = "list_mod";

pub fn list_mod_command(
    mut debug_command_events: EventReader<DebugCommandEvent>,
    mut debug_command_info_event: EventWriter<DebugCommandInfoEvent>,
    mod_entities: Query<(Entity, &ModName), With<Modification>>,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for debug_command_event in debug_command_events.read() {
        if debug_command_event.key != KEY {
            continue;
        }
        for (mod_entity, mod_name) in mod_entities.iter() {
            debug_command_info_event.send(DebugCommandInfoEvent { debug_command: format!("Mod [{:?}] available", mod_name.mod_name.to_lowercase().replace(" ", "")) });
        }
    }
}

const HELP_TEXT: &str = "list_mod";

pub fn push_list_mod_info(
    mut debug_command_events: EventReader<DebugCommandEvent>,
    mut debug_command_info_event: EventWriter<DebugCommandInfoEvent>,
) {
    for debug_command_event in debug_command_events.read() {
        if debug_command_event.key != "help" {
            continue;
        }

        debug_command_info_event.send(
            DebugCommandInfoEvent { debug_command: HELP_TEXT.to_string() }
        );
    }
}