use bevy::prelude::Event;

#[derive(Event)]
pub struct DebugCommandInfoEvent {
    pub debug_command: String,
}