use bevy::prelude::*;

use crate::AppState;
use crate::models::events::player_died_event::PlayerDiedEvent;

use crate::models::enemy::Enemy;


pub fn player_died_system(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    mut player_died_event: EventReader<PlayerDiedEvent>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for event in player_died_event.read() {
        // Kill all enemies
        for enemy_entity in enemy_query.iter() {
            commands.entity(enemy_entity).despawn();
        }
        
        // Kill all things under the player
        commands.entity(event.player_entity).despawn_recursive();

        // Move to GameOver state
        next_state.set(AppState::GameOver);
    }

}