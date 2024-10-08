use bevy::math::Vec3;
use bevy::prelude::{BuildChildren, Commands, Entity, EventWriter, GlobalTransform, Name, Quat, Query, Res, ResMut, Sprite, SpriteBundle, Transform, Vec2, With};
use bevy::sprite::Anchor;

use crate::assets_handling::preload_audio_system::SoundHandles;
//use crate::audio::sound_manager::SoundManager;

use crate::models::aim_direction::AimDirection;
use crate::models::audio::sound_handle_channel::SoundHandleChannel;
use crate::models::projectile::Projectile;
use crate::models::bundles::damage_bundle::DamageBundle;
use crate::models::collision::collider_owner::ColliderOwner;
use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::enemy_hit_box_collider::EnemyHitBoxCollider;
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::events::projectile_shot_event::ProjectileShotEvent;
use crate::models::gun::basic_sword::BasicSword;
use crate::models::move_direction::MoveDirection;
use crate::models::time_alive::TimeAlive;
use crate::models::unit_attributes::attribute::*;
use crate::models::unit_attributes::reload::Reload;
use crate::models::weapon_slot::WeaponSlot;
use crate::assets_handling::preload_texture_system::TextureHandles;

pub fn basic_sword_system(
    mut command: Commands,
    texture_handle: Res<TextureHandles>,
    //mut sound_manager: ResMut<SoundManager>,
    sound_handles: Res<SoundHandles>,
    mut projectile_shot_event_writer: EventWriter<ProjectileShotEvent>,
    mut weapon_holder_query: Query<(Entity, &AimDirection, &WeaponSlot, &mut Reload), With<BasicSword>>,
) {    
    for (entity, holder_aim_direction, weapon_holder_slot, mut holder_reloadable) in weapon_holder_query.iter_mut() {
        if holder_aim_direction.direction.length() == 0.0 {
            continue;
        }

        if holder_reloadable.reload_timer > 0.0 {
            continue;
        }
        holder_reloadable.reload_timer = holder_reloadable.get_total_amount();

        let projectile = command.spawn_empty().id();

        let mut collider_entities: Vec<Entity> = Vec::new();
        for index in 0..(400 / 64) {
            let collider = command.spawn_empty()
                .insert(Transform::from_translation(Vec3::new(0.0, index as f32 * 64.0, 0.0)))
                .insert(GlobalTransform::default())
                .insert(HitBoxCollider { collider_type: ColliderType::Circle(32.0) })
                .insert(EnemyHitBoxCollider)
                .insert(ColliderOwner(projectile))
                .id();

            collider_entities.push(collider)
        }

        command.entity(projectile).insert(SpriteBundle {
            transform: Transform::from_rotation(Quat::from_rotation_arc_2d(Vec2::new(0.0, 1.0), holder_aim_direction.direction)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(73.0, 288.0)),
                anchor: Anchor::BottomCenter,
                ..Default::default()
            },
            texture: texture_handle.sword.clone(),
            ..Default::default()
        })
            .insert(Name::new("SwordHit"))
            .insert(TimeAlive { time_alive: 0.4 })
            .insert(MoveDirection { direction: holder_aim_direction.direction })
            .insert(DamageBundle::new(0.5, 60.0))
            .insert(Projectile { source_entity: weapon_holder_slot.weapon_entity })
            .push_children(&*collider_entities);

        command.entity(entity).add_child(projectile);
        projectile_shot_event_writer.send(ProjectileShotEvent { entity: projectile });

       // sound_manager.queue_sound(SoundHandleChannel::Projectile(sound_handles.shoot_sound.clone()));
    }
}