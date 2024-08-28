use bevy::prelude::*;

use crate::models::configurations::world_grid_config::WorldGridConfig;
use crate::util::read_file_to_string::read_file_to_string;
use crate::models::configurations::game_config::GameConfig;

pub fn preload_world_grid_config_system(
    mut commands: Commands,
) 
{
    info!("preload_world_grid_config_system");
}