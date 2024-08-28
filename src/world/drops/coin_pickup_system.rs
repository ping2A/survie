use bevy::prelude::*;

use crate::assets_handling::preload_audio_system::SoundHandles;
use crate::audio::sound_manager::SoundManager;
use crate::models::audio::sound_handle_channel::SoundHandleChannel;
use crate::models::events::item_collision_event::ItemCollisionEvent;
use crate::models::resources::counter::CounterInformation;
use crate::models::items::descriptor::gold_value::GoldValue;

use crate::models::items::descriptor::item::Item;
use crate::ui::world_text::SpawnWorldText;

pub fn coin_pickup_system(
    mut commands: Commands,
    item_query: Query<&GoldValue>,
    sound_handles: Res<SoundHandles>,
    mut sound_manager: ResMut<SoundManager>,
    mut item_pickup_events: EventReader<ItemCollisionEvent>,
    mut counter_info: ResMut<CounterInformation>,
    coin_query: Query<&Transform, With<Item>>,
    mut ev_spawn_world_text: EventWriter<SpawnWorldText>,
) {
    for event in item_pickup_events.read() {
        let gold_value = match item_query.get(event.source_entity) {
            Ok(value) => value,
            Err(_) => continue
        };

        let item_position = match coin_query.get(event.source_entity) {
            Ok(transform) => transform.translation,
            Err(_) => continue,
        };

        ev_spawn_world_text.send(SpawnWorldText {
            pos: item_position,
            content: format!("+{}", gold_value.gold_value),
            ..default()
        });

        counter_info.increase_highscore(gold_value.gold_value as u32);

        sound_manager.queue_sound(SoundHandleChannel::Sound(sound_handles.coin_pickup_sound.clone()));
        commands.entity(event.source_entity).despawn_recursive();
    }
}