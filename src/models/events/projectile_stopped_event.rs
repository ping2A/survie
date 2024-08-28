use bevy::prelude::Entity;

use bevy::prelude::Event;

#[derive(Event)]
pub struct ProjectileStoppedEvent {
    pub projectile_entity: Entity,
}