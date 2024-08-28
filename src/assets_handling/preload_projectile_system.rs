use bevy::prelude::*;

use crate::models::configurations::projectile_config::ProjectileConfig;

#[derive(Default, Resource, Debug)]
pub struct ProjectileConfigHandles {
    pub basic_projectile: ProjectileConfig,
}

pub fn preload_projectile_system(
    mut projectile_handles: ResMut<ProjectileConfigHandles>)
{
    info!("preload_projectile_system");
}