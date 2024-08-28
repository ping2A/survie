extern crate core;

use bevy::asset::{AssetServer, Handle};
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(Default, Resource)]
pub struct SoundHandles {
    pub coin_pickup_sound: Handle<AudioSource>,
    pub background_music: Handle<AudioSource>,
    pub shoot_sound: Handle<AudioSource>,
    pub hit_sound: Handle<AudioSource>,
    pub teleport_sound: Handle<AudioSource>,
}

pub fn preload_audio_system(
    mut sound_handles: ResMut<SoundHandles>,
    asset_server: Res<AssetServer>,
) {
    info!("preload_audio_system");
}