use bevy::prelude::*;

use crate::models::animation::animation_state::{AnimationState, CurrentAnimationState};
use crate::models::animation::attack_animation_component::AttackAnimation;
use crate::models::move_direction::MoveDirection;

use crate::models::damaged_entities::{DamagedEntities, DamagedEntity};
use crate::models::enemy::Enemy;
use crate::models::player::Player;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::health::Health;
use crate::models::events::player_enemy_collision_event::PlayerEnemyCollisionEvent;

pub fn animation_move_attack_state_handle_system(
    mut player_enemy_collision_events: EventReader<PlayerEnemyCollisionEvent>,
    mut player_query: Query<(Entity, Option<&mut Health>, &Damage, &mut DamagedEntities), (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(Entity, Option<&mut Health>, &Damage, &mut DamagedEntities), (With<Enemy>, Without<Player>)>,
    mut player_anim_query: Query<(&mut CurrentAnimationState), (With<AttackAnimation>)>
) 
{
    for event in player_enemy_collision_events.read() {
        let (player_entity, player_health, player_damage, mut player_damaged_entities) = match player_query.get_mut(event.player_entity) {
            Ok(player) => player,
            Err(_) => continue,
        };

        let (enemy_entity, enemy_health, enemy_damage, mut enemy_damaged_entities) = match enemy_query.get_mut(event.enemy_entity) {
            Ok(enemy) => enemy,
            Err(_) => continue,
        };

        for (mut state) in player_anim_query.iter_mut() {
            state.state = AnimationState::Attack
        }
    }
}