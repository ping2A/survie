use std::fs;

use bevy::prelude::*;

use crate::util::entity_builder::EntityBuilder;

pub fn preload_mod_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    entity_builder: Res<EntityBuilder>,
) 
{
    info!("preload_mod_system");
}