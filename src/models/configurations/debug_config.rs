use bevy::prelude::Resource;
use serde::Deserialize;

#[derive(Default, Resource, Deserialize, Debug, Clone)]
pub struct DebugConfig {
    pub collider_circle_path: String,
    pub collider_rectangle_path: String,
}