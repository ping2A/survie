use std::fs;

use bevy::prelude::*;

use crate::models::spawner::spawn_stage::Stage;
use crate::util::read_file_to_string::read_file_to_string;
use crate::models::configurations::game_config::GameConfig;

#[derive(Default, Resource)]
pub struct StageList {
    pub stages: Vec<Stage>,
}

pub fn preload_stage_spawn_behavior_system(
    mut commands: Commands) 
{    
    info!("preload_stage_spawn_behavior_system");
}