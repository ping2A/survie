use bevy::prelude::*;

use crate::AssetState;

use crate::models::spawner::enemy_config_handle::EnemyConfigHandles;
use crate::assets_handling::preload_enemy_system::preload_enemy_system;

use crate::assets_handling::preload_animation_system::AtlasHandles;
use crate::assets_handling::preload_game_system::{DebugConfigHandles, preload_game_system};
use crate::assets_handling::preload_audio_system::{preload_audio_system, SoundHandles};
use crate::assets_handling::preload_player_system::{preload_player_system, PlayerConfigHandles};
use crate::assets_handling::preload_texture_system::{preload_texture_system, TextureHandles};
use crate::assets_handling::preload_world_grid_config_system::preload_world_grid_config_system;
use crate::util::entity_builder::EntityBuilderPlugin;
use crate::assets_handling::preload_stage_spawn_system::preload_stage_spawn_behavior_system;
use crate::assets_handling::preload_projectile_system::{preload_projectile_system, ProjectileConfigHandles};
use crate::assets_handling::preload_mod_system::preload_mod_system;
use crate::assets_handling::preload_item_system::{ItemConfigHandles, preload_item_system};
use crate::assets_handling::preload_window_system::WindowConfigHandles;

pub mod preload_animation_system;
pub mod preload_audio_system;
pub mod preload_player_system;
pub mod preload_game_system;
pub mod preload_mod_system;
pub mod preload_texture_system;
pub mod preload_world_grid_config_system;
pub mod preload_enemy_system;
pub mod preload_item_system;
pub mod preload_stage_spawn_system;
pub mod preload_projectile_system;
pub mod preload_window_system;

/// This plugin serves as a Preloader for all [ Assets ].
///
/// Currently it is divided into different data types like
/// [ preload_texture_system ] for sprites or
/// [ preload_item_system ] for items
///
/// The systems are run in the custom Startupstage [ AssetSetup ] in
/// order to have them ready when the game starts
pub struct AssetHandlingPlugin;

impl Plugin for AssetHandlingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EntityBuilderPlugin)
            .init_resource::<WindowConfigHandles>()
            .init_resource::<DebugConfigHandles>()
            .init_resource::<PlayerConfigHandles>()
            .init_resource::<TextureHandles>()
            .init_resource::<EnemyConfigHandles>()
            .init_resource::<ItemConfigHandles>()
            .init_resource::<ProjectileConfigHandles>()
            .init_resource::<SoundHandles>()
            .init_resource::<AtlasHandles>();

        app
            .add_systems(PreStartup, preload_game_system)
            .add_systems(PreStartup, preload_audio_system.after(preload_game_system));

        app
            .add_systems(OnEnter(AssetState::AssetLoading), preload_texture_system)
            .add_systems(OnEnter(AssetState::AssetLoading), preload_player_system)
            .add_systems(OnEnter(AssetState::AssetLoading), preload_world_grid_config_system)
            .add_systems(OnEnter(AssetState::AssetLoading), preload_item_system)
            .add_systems(OnEnter(AssetState::AssetLoading), preload_projectile_system)
            .add_systems(OnEnter(AssetState::AssetLoading), preload_stage_spawn_behavior_system)
            .add_systems(OnEnter(AssetState::AssetLoading), preload_enemy_system)
            .add_systems(OnEnter(AssetState::AssetLoading), preload_mod_system);
    }
}