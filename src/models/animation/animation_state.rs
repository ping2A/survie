use bevy::ecs::component::Component;
use bevy::prelude::States;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AnimationState {
    #[default]
    Idle,
    WalkSide,
    WalkUp,
    WalkDown,
    Attack,
}

#[derive(Component)]
pub struct CurrentAnimationState {
    pub state: AnimationState,
}

impl CurrentAnimationState {
    pub fn new() -> CurrentAnimationState {
        CurrentAnimationState { state: AnimationState::Idle }
    }
}