use bevy::ecs::component::Component;
use bevy::prelude::Vec2;

#[derive(Component, Default, Debug)]
pub struct AimDirection {
    pub direction: Vec2,
    pub last_direction: Vec2,
    pub trigger: bool,
}