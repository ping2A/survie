use bevy::prelude::*;

use crate::assets_handling::preload_texture_system::TextureHandles;
use crate::models::resources::counter::CounterInformation;
use crate::models::enemy::Enemy;

const SIZE: f32 = 32.0;

#[derive(Component)]
pub struct Counter;
#[derive(Component)]
pub struct CounterText;

pub fn spawn_kill_counter(mut commands: Commands, texture_handle: Res<TextureHandles>) {
    let icon = commands
        .spawn(ImageBundle {
            style: Style {
                width: Val::Px(SIZE),
                height: Val::Px(SIZE),
                margin: UiRect {
                    right: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            image: UiImage {
                texture: texture_handle.death_counter.clone(),
                ..default()
            },
            ..default()
        })
        .id();

    let text = commands
        .spawn((
            CounterText,
            TextBundle {
                text: Text::from_section(
                    "",
                    TextStyle {
                        font: texture_handle.font.clone(),
                        font_size: SIZE,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            Counter,
            NodeBundle {
                style: Style {
                    top: Val::Px(40.0),
                    left: Val::Px(40.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
        ))
        .push_children(&[icon, text]);
}


pub fn despawn_kill_counter(mut commands: Commands, q_counters: Query<Entity, With<Counter>>) {
    for entity in &q_counters {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_kill_counter_text(
    counter_info: Res<CounterInformation>,
    mut q_counter_text: Query<&mut Text, With<CounterText>>,
    enemy_count_query: Query<With<Enemy>>,
) {
    let mut text = match q_counter_text.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    text.sections[0].value =  format!("{} / {}", counter_info.kills().to_string(), enemy_count_query.iter().count().to_string())
}