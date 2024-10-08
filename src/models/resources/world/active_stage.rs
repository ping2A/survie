use bevy::prelude::Resource;
use crate::models::spawner::spawn_stage::Stage;

#[derive(Default, Debug, Resource)]
pub struct ActiveStage {
    pub stage: Stage,
    pub current_spawn_phase: usize,
    pub phase_time: f32,
    pub spawn_interval_time: f32,
}

impl ActiveStage {
    pub fn from_stage(stage: &Stage) -> Self {
        Self { stage: stage.clone(), current_spawn_phase: 0, phase_time: stage.spawn_phases[0].duration, spawn_interval_time: 0.0 }
    }
}