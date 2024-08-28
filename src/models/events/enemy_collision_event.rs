use bevy::prelude::Entity;
use bevy::prelude::Event;

#[derive(Event, Debug)]
pub struct EnemyCollisionEvent {
    pub target_entity: Entity,
    pub source_entity: Entity,
}