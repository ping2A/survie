use bevy::prelude::*;

use crate::models::aim_direction::AimDirection;
use crate::models::input::player_aim_controlled::PlayerAimControlled;

pub fn player_control_aim_system(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut AimDirection, With<PlayerAimControlled>>,
) {
    for mut player_aim_direction in player_query.iter_mut() {
        let mut direction = Vec2::default();

        if player_aim_direction.direction.length() != 0.0 {
            player_aim_direction.last_direction = player_aim_direction.direction;
        }
 
        if input.pressed(KeyCode::A) {
            direction += Vec2::new(-1.0, 0.0);
        }

        if input.pressed(KeyCode::D) {
            direction += Vec2::new(1.0, 0.0);
        }

        if input.pressed(KeyCode::W) {
            direction += Vec2::new(0.0, 1.0);
        }

        if input.pressed(KeyCode::S) {
            direction += Vec2::new(0.0, -1.0);
        }
        
        direction = direction.normalize_or_zero();

        if direction != player_aim_direction.direction {
            player_aim_direction.direction = direction;
        }
    }
}