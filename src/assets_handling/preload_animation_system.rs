use bevy::asset::{Handle};
use bevy::prelude::{Resource, TextureAtlas};

#[derive(Default, Resource)]
pub struct AtlasHandles {
    pub player_atlas: Handle<TextureAtlas>,
    pub explosion_atlas: Handle<TextureAtlas>,
    pub acid_puddle_atlas: Handle<TextureAtlas>,
    pub burning_atlas: Handle<TextureAtlas>,
}