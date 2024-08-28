
extern crate core;

use bevy::app::App;

use bevy::prelude::*;

use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_particle_systems::ParticleSystemPlugin;

use bevy_kira_audio::AudioPlugin;

use crate::animation::AnimationPlugin;
use crate::audio::CustomAudioPlugin;
use crate::assets_handling::AssetHandlingPlugin;
use crate::collision::CollisionPlugin;
use crate::ui::UiPlugin;
use crate::models::resources::ResourcePlugin;
use crate::models::events::EventsPlugin;
use crate::units::UnitPlugin;
use crate::input::InputPlugin;
use crate::scheduling::SchedulingPlugin;
use crate::world::WorldPlugin;

mod animation;
mod assets_handling;
mod audio;
mod collision;
mod input;
mod scheduling;
mod util;
mod ui;
mod units;
mod models;
mod world;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum AssetState {
    #[default]
    ConfigLoading,
    AssetLoading
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Loading,
    InGame,
    GameOver,
    Paused,
    Shop,
    GameWon,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum ConsoleState {
    #[default]
    Hidden,
    Shown,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum MainMenuState {
    #[default]
    Splash,
    Entry,
    MainMenu,
    ChooseStage,
    Options,
}

fn main() {
    info!("Starting Survie Engine");

    App::new()
        .add_plugins((
            DefaultPlugins,
            AudioPlugin))
        .add_plugins(EguiPlugin)
        
        .add_plugins(ParticleSystemPlugin)

        .add_state::<AppState>()
        .add_state::<ConsoleState>()
        .add_state::<MainMenuState>()
        .add_state::<AssetState>()
        
        .add_plugins(WorldInspectorPlugin::new().run_if(in_state(ConsoleState::Shown)))

        .add_plugins(UiPlugin)
        .add_plugins(AssetHandlingPlugin)
        .add_plugins(CustomAudioPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(SchedulingPlugin)
        .add_plugins(ResourcePlugin)
        .add_plugins(EventsPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(UnitPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(WorldPlugin)

        .run()
}