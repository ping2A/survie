use bevy::prelude::{Entity, Resource};
use bevy::prelude::Event;

#[derive(Event, Resource)]
pub struct ProjectileShotEvent {
    pub entity: Entity,
}