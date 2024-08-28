use bevy::prelude::*;

#[derive(Component)]
pub struct TextAnimation {
  pub text: String,
  pub animation_speed: f32,
}

#[derive(Component)]
pub struct AnimatingText {
  pub progress: f32,
}

#[derive(Component)]
pub struct AnimatedText;