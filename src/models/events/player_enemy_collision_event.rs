use bevy::prelude::Entity;
use bevy::prelude::Event;

#[derive(Event, Debug)]
pub struct PlayerEnemyCollisionEvent {
    pub player_entity: Entity,
    pub enemy_entity: Entity,
}