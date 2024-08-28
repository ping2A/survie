use bevy::prelude::*;

use crate::ui::main_menu::OnSplashScreen;

use crate::models::ui::text::TextAnimation;

use crate::MainMenuState;

pub const MISTY: Color = Color::rgb(0.992, 0.91, 0.914); //#FDE8E9


#[derive(Component)]
pub struct PressSpace;

pub fn startup_splash_title(
    mut commands: Commands,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
) {
    info!("startup_splash_title");

    commands
    .spawn((
      NodeBundle {
        style: Style {
          align_items: AlignItems::Center,
          justify_content: JustifyContent::Center,
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
    .with_children(|parent| {
        /* 
       parent
        .spawn(NodeBundle {
          style: Style {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            ..default()
          },
          ..default()
        })
        .with_children(|parent2| {
          parent2.spawn(ImageBundle {
            style: Style {
              width: Val::Px(800.0),
              ..default()
            },
          //  image: UiImage::new(icon),
            ..default()
          });
        });*/
      parent
        .spawn(
          TextBundle::from_section(
            "",
            TextStyle {
              font_size: 100.0,
              color: MISTY,
              ..default()
            },
          )
          .with_style(Style {
            margin: UiRect::top(Val::Px(50.0)),
            ..default()
          }),
        )
        .insert(PressSpace);
    });

    next_menu_state.set(MainMenuState::Entry);
}



pub fn show_title_screen_system(
    mut commands: Commands,
    qry: Query<Entity, With<PressSpace>>,
) {
    commands.entity(qry.single()).insert(TextAnimation {
        text: "Press space to play".to_owned(),
        animation_speed: 0.6,
      });
}

pub fn get_title_screen_system(
    keys: Res<Input<KeyCode>>,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
) {
   if keys.pressed(KeyCode::Space) {
        next_menu_state.set(MainMenuState::MainMenu);        
    }
}