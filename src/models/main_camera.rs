use bevy::ecs::component::Component;
use bevy::prelude::*;

pub const ZOOM_LOWER_BOUND: f32 = 0.3;
pub const ZOOM_UPPER_BOUND: f32 = 0.9;

#[derive(Component)]
pub struct MainCamera;

#[derive(Resource, Debug)]
pub struct Zoom(pub f32);

pub const BACKGROUND: Color = Color::rgba(0.133, 0.655, 1., 0.5);
