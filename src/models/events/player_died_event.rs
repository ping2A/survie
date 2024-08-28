use bevy::prelude::Entity;

use bevy::prelude::Event;

#[derive(Event)]
pub struct PlayerDiedEvent {
    pub player_entity: Entity,
}