use bevy::prelude::{Res, ResMut, Resource};
use bevy_kira_audio::{Audio, AudioChannel, AudioControl, MainTrack};

use crate::models::audio::sound_channel::SoundChannel;
use crate::models::audio::sound_handle_channel::SoundHandleChannel;

#[derive(Default, Resource)]
pub struct SoundManager {
    channel_vector_sound: Vec<AudioChannel<MainTrack>>,
    channel_vector_background: Vec<AudioChannel<MainTrack>>,

    sound_handle_vector: Vec<SoundHandleChannel>,
}

impl SoundManager {
    pub fn set_volume_for_channel(&self, volume: f32, channel: SoundChannel, audio: Res<Audio>) {
        match channel {
            SoundChannel::Sound => {
                for channel in self.channel_vector_sound.iter() {
                    audio.set_volume(volume as f64);
                }
            }
            SoundChannel::Background => {
                for channel in self.channel_vector_background.iter() {
                    audio.set_volume(volume as f64);
                }
            }
        }
    }

    pub fn sound_manager_system(
        mut sound_manager: ResMut<SoundManager>,
        audio: Res<Audio>,
    ) {
        while !sound_manager.sound_handle_vector.is_empty() {
            let channel_identifier = sound_manager.sound_handle_vector.remove(0);

            match channel_identifier {
                SoundHandleChannel::Sound(handle) => {
                    let current_channel = sound_manager.channel_vector_sound.remove(0);
                    //audio.stop();
                    audio.play(handle.clone());
                    sound_manager.channel_vector_sound.push(current_channel);
                }
                SoundHandleChannel::Background(handle) => {
                    let current_channel = sound_manager.channel_vector_background.remove(0);
                    audio.stop();
                    audio.play(handle.clone()).looped();
                    sound_manager.channel_vector_background.push(current_channel);
                }
            }
        }
    }

    pub fn new() -> Self {
        let mut channel_vector = SoundManager::default();

        for _ in 1..2 {
            channel_vector.channel_vector_background.push(AudioChannel::default());
        }

        for _ in 1..3 {
            channel_vector.channel_vector_sound.push(AudioChannel::default());
        }

        channel_vector
    }

    pub fn queue_sound(
        &mut self,
        handle_enum: SoundHandleChannel,
    ) {
        self.sound_handle_vector.push(handle_enum)
    }
}