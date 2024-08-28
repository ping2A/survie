use bevy::prelude::{Entity, Query, With};

use crate::models::modifications::auto_shot::AutoShot;
use crate::models::move_direction::MoveDirection;
use crate::models::unit_attributes::travel_range::TravelRange;

/// A system to auotmatically find a target the [Projectile] that has [AuotShot] applied to it.
pub fn auto_shot_system(
    mut projectile_query: Query<(Entity, &mut MoveDirection, &TravelRange), With<AutoShot>>,
) {
}