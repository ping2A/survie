use bevy::prelude::*;

#[derive(Event)]
pub struct SplashLogEvent(pub String);
impl From<&str> for SplashLogEvent {
  fn from(value: &str) -> Self {
    Self(value.to_owned())
  }
}