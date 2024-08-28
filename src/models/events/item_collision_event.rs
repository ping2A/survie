use bevy::prelude::Entity;
use bevy::prelude::Event;

#[derive(Event)]
pub struct ItemCollisionEvent {
    pub source_entity: Entity,
    pub target_entity: Entity,
}