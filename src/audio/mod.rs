use bevy::app::App;
use bevy::prelude::*;

use bevy_kira_audio::prelude::*;

use crate::MainMenuState;

//use crate::assets_handling::preload_audio_system::preload_audio_system;
use crate::audio::background_music_system::play_background_music_system;
use crate::audio::sound_manager::SoundManager;

mod background_music_system;
pub mod sound_manager;

pub struct CustomAudioPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct AudioSystemSet;


impl Plugin for CustomAudioPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PostUpdate,
            AudioSystemSet
        );

        app.insert_resource(SoundManager::new());

        app
            .add_systems(PostUpdate, SoundManager::sound_manager_system.in_set(AudioSystemSet))
            .add_systems(OnEnter(MainMenuState::Splash), play_background_music_system.in_set(AudioSystemSet));
    }
}

