use bevy::prelude::Resource;
use serde::Deserialize;

#[derive(Default, Resource, Deserialize, Debug, Clone)]
pub struct AudioConfig {
    pub background_path: String,
    pub hit_path: String,
    pub shoot_path: String,
    pub coin_path: String
}