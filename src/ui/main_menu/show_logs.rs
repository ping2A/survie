use bevy::prelude::*;

use crate::ui::main_menu::OnSplashScreen;

use crate::models::ui::logs::LogText;
use crate::models::events::logs_event::SplashLogEvent;

pub const MISTY: Color = Color::rgb(0.992, 0.91, 0.914); //#FDE8E9

pub fn startup_logs_system(mut commands: Commands) {
    info!("startup_logs_system");
    
    commands
        .spawn((
            NodeBundle {
            style: Style {
                align_items: AlignItems::Start,
                justify_content: JustifyContent::End,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|p| {
            p.spawn(TextBundle::default()).insert(LogText);
        });
}

pub fn show_logs_system(mut log: EventReader<SplashLogEvent>, mut qry: Query<&mut Text, With<LogText>>) {
    if let Ok(mut target) = qry.get_single_mut() {
        for l in log.read() {
        if let Some(m) = target
            .sections
            .iter_mut()
            .find(|t| l.0.starts_with(&t.value[1..]))
        {
            m.value = format!("\n{}", l.0);
        } else {
            target.sections.push(TextSection {
            value: format!("\n{}", l.0),
            style: TextStyle {
                font_size: 20.0,
                color: MISTY,
                ..default()
            },
            ..default()
            })
        }
        }
    }
}