use bevy::prelude::*;

use crate::models::configurations::game_config::GameConfig;
use crate::models::configurations::debug_config::DebugConfig;

use crate::assets_handling::preload_player_system::PlayerConfigHandles;
use crate::assets_handling::preload_item_system::ItemConfigHandles;
use crate::assets_handling::preload_projectile_system::ProjectileConfigHandles;
use crate::assets_handling::preload_window_system::WindowConfigHandles;
use crate::assets_handling::preload_audio_system::SoundHandles;

use crate::models::spawner::enemy_config_handle::EnemyConfigHandles;
use crate::assets_handling::preload_stage_spawn_system::StageList;
use crate::models::spawner::enemy_config::EnemyConfig;
use crate::util::entity_builder::EntityBuilder;
use crate::models::events::logs_event::SplashLogEvent;

use crate::AssetState;

use crate::util::read_file_to_string::read_file_to_string;

#[derive(Default, Resource)]
pub struct DebugConfigHandles {
    pub config: DebugConfig,
}


pub fn preload_game_system(
    mut log: EventWriter<SplashLogEvent>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut player_handles: ResMut<PlayerConfigHandles>,
    mut item_config_handles: ResMut<ItemConfigHandles>,
    mut projectile_handles: ResMut<ProjectileConfigHandles>,
    mut enemy_handles: ResMut<EnemyConfigHandles>,
    mut window_handles: ResMut<WindowConfigHandles>,
    mut sound_handles: ResMut<SoundHandles>,
    mut debug_handles: ResMut<DebugConfigHandles>,
    entity_builder: Res<EntityBuilder>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<AssetState>>,
) {
    info!("Preloading Game assets...");
    log.send("Preloading Game assets...".into());

    //let my_string = include_str!("../../assets/default/configurations/default.json"); //read_file_to_string("assets/default/configurations/default.json");
    let my_string = include_str!("../../assets/default/configurations/default.json"); 
    let config: GameConfig = serde_json::from_str(&my_string).expect("JSON was not well-formatted || GAME");
    
    player_handles.config = config.player_config.clone();
    item_config_handles.coin = config.coin_config.clone();
    projectile_handles.basic_projectile = config.projectile_config.clone();
    window_handles.config = config.window_config.clone();
    debug_handles.config = config.debug_config.clone();

    // Insert other resources configs
    commands.insert_resource(config.spawner_config);
    commands.insert_resource(config.world_config);

    // Insert other resources data

    // Stages
    let mut stage_list = StageList::default();
    stage_list.stages = config.stages_config.clone();
    commands.insert_resource(stage_list);

    // Enemy
    let mut enemy_configs: Vec<EnemyConfig> = Vec::new();
    for raw_enemy_config in config.enemies_config {
        enemy_configs.push(raw_enemy_config.get_config(&asset_server, &mut texture_atlases));
    }
    enemy_configs.sort_by(|a, b| a.config_id.partial_cmp(&b.config_id).unwrap());
    enemy_handles.enemy_configs = enemy_configs;

    // Mods
    let parent = commands.spawn_empty().insert(Name::new("Mod Entities")).id();
    for mut mod_config in config.mods_config {
        let child = entity_builder.spawn_entity(&mut commands, &asset_server, &mut mod_config);
        commands.entity(parent).add_child(child);
    }

    // Load Audio
    //  sound_handles.teleport_sound = asset_server.load("audio/teleport.mp3");
    sound_handles.coin_pickup_sound = asset_server.load(config.audio_config.coin_path.clone());
    sound_handles.background_music = asset_server.load(config.audio_config.background_path.clone());
    sound_handles.hit_sound = asset_server.load(config.audio_config.hit_path.clone());
    sound_handles.shoot_sound = asset_server.load(config.audio_config.shoot_path.clone());

    // Now load images/audio/etc
    next_state.set(AssetState::AssetLoading);
}