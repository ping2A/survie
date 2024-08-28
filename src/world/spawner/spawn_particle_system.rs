use bevy::prelude::*;

use crate::models::enemy::Enemy;

use crate::models::player::Player;

use crate::models::events::enemy_collision_event::EnemyCollisionEvent;


use bevy_particle_systems::{
    ParticleBurst, ParticleSystem, ParticleSystemBundle, Playing,
    VelocityModifier,
};

pub fn spawn_particle_system(
    mut commands: Commands,
    mut enemy_hit_event: EventReader<EnemyCollisionEvent>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {

    for event in enemy_hit_event.read() {
        let enemy_position = match enemy_query.get(event.target_entity) {
            Ok(transform) => transform.translation,
            Err(_) => continue,
        };

        let mut drop_translation = enemy_position;
        drop_translation.z = 0.0;
/* 
        commands.spawn((
            ParticleSystemBundle {
                transform: Transform::from_translation(drop_translation),
                particle_system: ParticleSystem {
                    spawn_rate_per_second: 0.0.into(),
                    max_particles: 400,
                    initial_speed: (0.0..1000.0).into(),
                    scale: 2.0.into(),
                    velocity_modifiers: vec![
                        VelocityModifier::Drag(0.001.into()),
                        VelocityModifier::Vector(Vec3::new(0.0, -400.0, 0.0).into()),
                    ],
                    color: (Color::RED..Color::rgba(1.0, 0.0, 0.0, 0.0)).into(),
                    bursts: vec![ParticleBurst {
                        time: 0.2,
                        count: 50,
                    }],
                    ..ParticleSystem::oneshot()
                },
                ..default()
            },
            Playing,
        ));*/

    }
}