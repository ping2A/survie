use bevy::prelude::Resource;
use serde::Deserialize;

#[derive(Default, Resource, Deserialize, Debug, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub width: f32,
    pub height: f32,
    pub background_path: String
}