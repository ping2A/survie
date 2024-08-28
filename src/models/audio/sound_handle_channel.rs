use bevy::asset::Handle;
use bevy_kira_audio::AudioSource;

pub enum SoundHandleChannel {
    Sound(Handle<AudioSource>),
    Background(Handle<AudioSource>),
}