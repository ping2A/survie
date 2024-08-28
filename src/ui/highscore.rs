use bevy::prelude::*;

use crate::assets_handling::preload_texture_system::TextureHandles;
use crate::models::resources::counter::CounterInformation;

const SIZE: f32 = 32.0;

#[derive(Component)]
pub struct Score;
#[derive(Component)]
pub struct ScoreText;

pub fn spawn_highscore(mut commands: Commands, texture_handle: Res<TextureHandles>) {
    let icon = commands
        .spawn(ImageBundle {
            style: Style {
                width: Val::Px(SIZE),
                height: Val::Px(2.0 * SIZE),
                margin: UiRect {
                    right: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            image: UiImage {
                texture: texture_handle.highscore.clone(),
                ..default()
            },
            ..default()
        })
        .id();

    let text = commands
        .spawn((
            ScoreText,
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
            Score,
            NodeBundle {
                style: Style {
                    top: Val::Px(100.0),
                    left: Val::Px(40.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
        ))
        .push_children(&[icon, text]);
}

pub fn despawn_highscore(mut commands: Commands, q_counters: Query<Entity, With<Score>>) {
    for entity in &q_counters {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_highscore_text(
    counter_info: Res<CounterInformation>,
    mut q_counter_text: Query<&mut Text, With<ScoreText>>,
) {
    let mut text = match q_counter_text.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    text.sections[0].value = counter_info.highscore().to_string();
}
