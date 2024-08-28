use serde::Deserialize;
use std::collections::HashMap;

use crate::models::configurations::window_config::WindowConfig;
use crate::models::configurations::audio_config::AudioConfig;
use crate::models::configurations::debug_config::DebugConfig;
use crate::models::configurations::player_config::PlayerConfig;
use crate::models::configurations::spawner_config::SpawnerConfig;
use crate::models::configurations::world_grid_config::WorldGridConfig;
use crate::models::configurations::coin_config::CoinConfig;
use crate::models::configurations::projectile_config::ProjectileConfig;
use crate::models::spawner::spawn_stage::Stage;
use crate::models::configurations::raw_configs::raw_enemy_config::RawEnemyConfig;

#[derive(Default, Deserialize, Debug)]
pub struct GameConfig {
    pub window_config: WindowConfig,
    pub audio_config: AudioConfig,
    pub debug_config: DebugConfig,
    pub player_config: PlayerConfig,
    pub spawner_config: SpawnerConfig,
    pub world_config: WorldGridConfig,
    pub coin_config: CoinConfig,
    pub projectile_config: ProjectileConfig,
    pub stages_config: Vec<Stage>,
    pub enemies_config: Vec<RawEnemyConfig>,
    pub mods_config: Vec<HashMap<String, serde_json::Value>>,
}