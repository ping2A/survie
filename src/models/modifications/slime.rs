use bevy::ecs::component::Component;
use serde::Deserialize;

#[derive(Clone, Component, Deserialize)]
pub struct Slime {
    pub sprite_path: String,
}

#[derive(Component)]
pub struct SlimeUnit;