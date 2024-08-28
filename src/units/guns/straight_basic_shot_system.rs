use bevy::math::EulerRot;
use bevy::prelude::*;
use rand::random;

use crate::assets_handling::preload_audio_system::SoundHandles;
use crate::assets_handling::preload_projectile_system::ProjectileConfigHandles;

use crate::audio::sound_manager::SoundManager;

use crate::models::aim_direction::AimDirection;
use crate::models::audio::sound_handle_channel::SoundHandleChannel;
use crate::models::behavior::rotate_behavior::UnitRotation;
use crate::models::bundles::damage_bundle::DamageBundle;
use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::events::projectile_shot_event::ProjectileShotEvent;
use crate::models::gun::straight_basic_shot::StraightBasicShot;
use crate::models::move_direction::MoveDirection;
use crate::models::projectile::Projectile;
use crate::models::sprite_layer::SpriteLayer;
use crate::models::unit_attributes::attribute::*;
use crate::models::unit_attributes::hit_limit::HitLimit;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_attributes::reload::Reload;
use crate::models::unit_attributes::travel_range::TravelRange;
use crate::models::unit_attributes::unit_size::UnitSize;
use crate::models::weapon_slot::WeaponSlot;
use crate::models::knock_back::KnockBack;
use crate::assets_handling::preload_texture_system::TextureHandles;

use crate::models::animation::animation_state::CurrentAnimationState;
use crate::models::animation::idle_animation_component::IdleAnimation;
use crate::models::sprite_flip::SpriteFlip;


pub fn straight_basic_shot_system(
    mut command: Commands,
    texture_handle: Res<TextureHandles>,
    projectile_handle: Res<ProjectileConfigHandles>,
    mut sound_manager: ResMut<SoundManager>,
    sound_handles: Res<SoundHandles>,
    mut projectile_shot_event_writer: EventWriter<ProjectileShotEvent>,
    mut weapon_holder_query: Query<(&GlobalTransform, &AimDirection, &WeaponSlot, &mut Reload), With<StraightBasicShot>>,
) {

    for (holder_transform, holder_aim_direction, weapon_holder_slot, mut holder_reloadable) in weapon_holder_query.iter_mut() {        
        if holder_aim_direction.direction.length() == 0.0 {
            continue;
        }

        let mut shot_direction = holder_aim_direction.direction;

        if holder_aim_direction.direction.length() == 0.0 {
            shot_direction = holder_aim_direction.last_direction;
        }

        if holder_reloadable.reload_timer > 0.0 {
            continue;
        }

        holder_reloadable.reload_timer = holder_reloadable.get_total_amount();

        let mut projectile_transform = Transform::from_xyz(holder_transform.translation().x, holder_transform.translation().y, SpriteLayer::LowGroundLevel.get_layer_z());
        let angle = shot_direction.y.atan2(shot_direction.x);
        projectile_transform.rotation = Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), angle);

        let projectile = command.spawn(
            SpriteSheetBundle {
                transform: projectile_transform,
                sprite: TextureAtlasSprite {
                    custom_size: Some(
                        Vec2::new(projectile_handle.basic_projectile.sprite_custom_size_x, projectile_handle.basic_projectile.sprite_custom_size_y)),
                    ..Default::default()
                },
                texture_atlas: texture_handle.projectile_fireball.clone(),
                ..Default::default()
            },
        )
            .insert(Name::new("Projectile"))
            .insert(Projectile { source_entity: weapon_holder_slot.weapon_entity })

            .insert(UnitSize::new_size(Vec2::new(projectile_handle.basic_projectile.sprite_custom_size_x, projectile_handle.basic_projectile.sprite_custom_size_y)))
            .insert(HitBoxCollider { collider_type: ColliderType::Circle(projectile_handle.basic_projectile.sprite_custom_size_x / 2.0) })

            .insert(DamageBundle::new(projectile_handle.basic_projectile.damage, 60.0))

            .insert(MoveSpeed::new(projectile_handle.basic_projectile.speed))
            .insert(MoveDirection { direction: shot_direction })

            .insert(HitLimit::new(projectile_handle.basic_projectile.hit_limit))
            .insert(TravelRange::new(projectile_handle.basic_projectile.range))

            .insert(KnockBack { push_duration: 0.1, push_force: 2000. })
         
            .insert(IdleAnimation::new(0.0, 5, 5, 0, 0.5))
            .insert(CurrentAnimationState::new())

            .id();

        projectile_shot_event_writer.send(ProjectileShotEvent { entity: projectile });

       // sound_manager.queue_sound(SoundHandleChannel::Sound(sound_handles.shoot_sound.clone()));
    }
}