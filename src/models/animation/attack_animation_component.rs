use bevy::ecs::component::Component;

#[derive(Component, Debug)]
pub struct AttackAnimation {
    pub progress: f32,
    pub sprite_row_length: usize,
    pub animation_frame_count: usize,
    pub atlas_row: usize,
    pub duration: f32,
}

impl AttackAnimation {
    pub fn new(sprite_row_length:usize, animation_frame_count: usize, atlas_row: usize, duration: f32) -> AttackAnimation {
        AttackAnimation { progress: 0.0, sprite_row_length, animation_frame_count, atlas_row, duration }
    }
}