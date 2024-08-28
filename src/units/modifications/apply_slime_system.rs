use bevy::prelude::*;

use crate::models::sprite_layer::SpriteLayer;

use crate::models::animation::animation_state::CurrentAnimationState;
use crate::models::animation::idle_animation_component::IdleAnimation;

use crate::models::behavior::chase_target_behavior::ChaseTargetBehavior;
use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::layerable::Layerable;
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::modifications::slime::{Slime, SlimeUnit};
use crate::models::modifications::utils::owner::Owner;
use crate::models::move_direction::MoveDirection;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::util::get_close_position_2d::get_close_position_2d;

pub fn apply_slime_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut apply_events: EventReader<ApplyModToTargetEvent>,
    mod_query: Query<&Slime, With<Modification>>,
    owner_query: Query<(Entity, &Transform)>,
    unit_query: Query<&Owner, With<SlimeUnit>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for apply_event in apply_events.read() {
        let modification = match mod_query.get(apply_event.mod_entity) {
            Ok(modification) => modification,
            Err(_) => continue,
        };

        let (owner_entity, owner_transform) = match owner_query.get(apply_event.target_entity) {
            Ok(owner) => owner,
            Err(_) => continue,
        };

        let mut unit_exists = false;
        for owner in unit_query.iter() {
            if owner_entity == owner.entity {
                unit_exists = true;
            }
        }

        if unit_exists {
            continue;
        }

        let pos_vec = get_close_position_2d(owner_transform.translation.x, owner_transform.translation.y, 300.0, 1000.0);

        commands.spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(256.0, 256.0)),
                ..Default::default()
            },
            texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load(&modification.sprite_path), Vec2::new(32.0, 32.0), 34, 1, None, None)),
            transform: Transform::from_xyz(pos_vec[0], pos_vec[1], 0.0),
            ..Default::default()
        })
            .insert(Layerable::new(SpriteLayer::GroundLevel.get_layer_z()))
            .insert(SlimeUnit)
            .insert(Owner::new(owner_entity))
            .insert(Name::new("Slime"))
            .insert(ChaseTargetBehavior { target: owner_entity, proximity: 200.0 })
            .insert(MoveDirection::default())
            .insert(MoveSpeed::new(6.0))
            .insert(IdleAnimation::new(0.0, 5, 34, 0, 1.))
            .insert(CurrentAnimationState::new());
    }
}
