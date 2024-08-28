use bevy::prelude::*;

pub struct SchedulingPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum BaseSets {
    First,
    Collision,
    PreUpdate,
    Update,
    PostUpdate,
    Last,
    Flush,
}

impl Plugin for SchedulingPlugin {
    fn build(&self, app: &mut App) {
        app
            .configure_sets(Update,
                (
                BaseSets::First,
                BaseSets::Collision,
                BaseSets::PreUpdate,
                BaseSets::Update,
                BaseSets::PostUpdate,
                BaseSets::Last,
                BaseSets::Flush,
            ).chain())

            .add_systems(Update, apply_deferred)
        ;
    }
}