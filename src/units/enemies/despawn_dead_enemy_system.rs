use bevy::prelude::*;

use crate::models::events::target_died_event::TargetDiedEvent;
use crate::models::resources::counter::CounterInformation;

pub fn despawn_dead_enemy_system(
    mut commands: Commands,
    mut enemy_died_event: EventReader<TargetDiedEvent>,
    mut counter_info: ResMut<CounterInformation>,
) {    
    for event in enemy_died_event.read() {
        counter_info.increase_kill();
        commands.entity(event.target_entity).despawn_recursive();
    }
}