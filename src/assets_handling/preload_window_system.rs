extern crate core;

use bevy::asset::AssetServer;
use bevy::prelude::*;

use crate::models::configurations::window_config::WindowConfig;


#[derive(Default, Resource)]
pub struct WindowConfigHandles {
    pub config: WindowConfig,
}


pub fn preload_window_system(
    window_handles: ResMut<WindowConfigHandles>,
    asset_server: Res<AssetServer>,
) {
}