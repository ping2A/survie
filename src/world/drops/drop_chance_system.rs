use bevy::prelude::*;

use rand::Rng;

use colorgrad;
use colorgrad::Gradient;

use crate::assets_handling::preload_item_system::ItemConfigHandles;
use crate::assets_handling::preload_texture_system::TextureHandles;
use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::collider_weight::ColliderWeight;
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::enemy::Enemy;
use crate::models::events::target_died_event::TargetDiedEvent;
use crate::models::items::coin::Coin;
use crate::models::items::descriptor::gold_value::GoldValue;

use crate::models::items::descriptor::item::Item;
use crate::models::layerable::Layerable;
use crate::models::sprite_layer::SpriteLayer;

pub fn drop_chance_system(
    mut commands: Commands,
    mut enemy_died_event: EventReader<TargetDiedEvent>,
    texture_handles: Res<TextureHandles>,
    item_handles: Res<ItemConfigHandles>,
    enemy_query: Query<&Transform, With<Enemy>>) 
{
    for event in enemy_died_event.read() {
        let enemy_position = match enemy_query.get(event.target_entity) {
            Ok(transform) => transform.translation,
            Err(_) => continue,
        };


        let mut drop_translation = enemy_position;
        drop_translation.z = 0.0;

        let value = rand::thread_rng().gen_range(0.0..1.0);

        let g = colorgrad::GradientBuilder::new()
            .html_colors(&["seagreen", "gold", "crimson"])
            .domain(&[0.0, 0.7, 1.0])
            .build::<colorgrad::LinearGradient>().unwrap();
        let color_grad = g.at(value);

        commands.spawn(
            SpriteBundle {
                transform: Transform::from_translation(drop_translation),
                texture: texture_handles.coin_sprite.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(item_handles.coin.sprite_custom_size_x, item_handles.coin.sprite_custom_size_y)),
                    color: Color::from(color_grad.to_array()),
                    ..Default::default()
                },
                ..Default::default()
            }
        )
            .insert(Item)
            .insert(Layerable::new(SpriteLayer::LowGroundLevel.get_layer_z()))
            .insert(Coin)
            .insert(Name::new("Item Coin"))
            .insert(GoldValue { gold_value: (value * 10.0) as i32 })
            .insert(HitBoxCollider { collider_type: ColliderType::Circle(item_handles.coin.sprite_custom_size_x / 2.0) })
            .insert(ColliderWeight { weight: 0.0 });
    }
}