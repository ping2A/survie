use bevy::prelude::Entity;
use bevy::prelude::Event;

#[derive(Event)]
pub struct TargetDiedEvent {
    pub target_entity: Entity,
}