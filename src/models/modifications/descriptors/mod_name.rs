use bevy::ecs::component::Component;
use serde::Deserialize;

#[derive(Component, Deserialize, Debug)]
pub struct ModName {
    pub mod_name: String,
}