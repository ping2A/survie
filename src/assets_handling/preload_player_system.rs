use bevy::prelude::*;

use crate::models::configurations::player_config::PlayerConfig;
use crate::assets_handling::preload_animation_system::AtlasHandles;
use crate::models::events::logs_event::SplashLogEvent;

#[derive(Default, Resource)]
pub struct PlayerConfigHandles {
    pub config: PlayerConfig,
}

pub fn preload_player_system(
    mut log: EventWriter<SplashLogEvent>,
    asset_server: Res<AssetServer>,
    player_config: Res<PlayerConfigHandles>,
    mut atlas_handles: ResMut<AtlasHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    log.send("Preloading player...".into());

    atlas_handles.player_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load(&player_config.config.sprite_path), 
        player_config.config.texture_atlas_grid_size, 
        player_config.config.texture_atlas_columns, 
        player_config.config.texture_atlas_rows, 
        None, None));

    atlas_handles.explosion_atlas = texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("default/sprites/weapons/explosion.png"), 
            Vec2::new( 128.0, 128.0), 
            8, 
            5, 
            None, None));
    
}