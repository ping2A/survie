use std::fs;

use bevy::prelude::*;

use crate::models::configurations::raw_configs::raw_enemy_config::RawEnemyConfig;
use crate::models::spawner::enemy_config::EnemyConfig;
use crate::models::spawner::enemy_config_handle::EnemyConfigHandles;
use crate::util::read_file_to_string::read_file_to_string;
use crate::models::configurations::game_config::GameConfig;

pub fn preload_enemy_system(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut enemy_handles_new: ResMut<EnemyConfigHandles>,
) 
{
    info!("preload_enemy_system");
}