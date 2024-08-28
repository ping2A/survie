use bevy::prelude::{Entity, Resource};
use bevy::prelude::Event;

#[derive(Event, Resource)]
pub struct RemoveModFromTargetEvent {
    pub mod_entity: Entity,
    pub target_entity: Entity,
}