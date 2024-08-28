use bevy::prelude::*;

use crate::models::input::player_move_controlled::PlayerMoveControlled;
use crate::models::move_direction::MoveDirection;

pub fn player_control_movement_system(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut MoveDirection, With<PlayerMoveControlled>>,
) {    
    for mut player_direction in player_query.iter_mut() {
        let mut direction = Vec2::default();

        if input.pressed(KeyCode::Left) {
            direction += Vec2::new(-1.0, 0.0);
        }

        if input.pressed(KeyCode::Right) {
            direction += Vec2::new(1.0, 0.0);
        }

        if input.pressed(KeyCode::Up) {
            direction += Vec2::new(0.0, 1.0);
        }

        if input.pressed(KeyCode::Down) {
            direction += Vec2::new(0.0, -1.0);
        }

        direction = direction.normalize_or_zero();

        if player_direction.direction != direction {
            player_direction.direction = direction;
        }
    }
}