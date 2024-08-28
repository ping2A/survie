use bevy::app::{App, Plugin};
use bevy::prelude::*;
use crate::{AppState, MainMenuState};

use crate::ui::main_menu::show_choose_stage_system::show_choose_stage_system;
use crate::ui::main_menu::show_title_screen_system::{startup_splash_title, show_title_screen_system, get_title_screen_system};
use crate::ui::main_menu::show_main_menu_system::show_main_menu_system;
use crate::ui::main_menu::show_options_system::show_options_system;
use crate::ui::main_menu::show_logs::{show_logs_system, startup_logs_system};
use crate::ui::main_menu::show_restart_screen_system::show_restart_screen_system;

mod show_title_screen_system;
mod show_main_menu_system;
mod show_options_system;
mod show_choose_stage_system;
mod show_logs;
mod show_restart_screen_system;

pub struct MainMenuPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MainMenuSystemSet;

#[derive(Component)]
struct OnSplashScreen;

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    info!("despawn_screen");

    for entity in &to_despawn {
      commands.entity(entity).despawn_recursive();
    }
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {        
        app.configure_sets(
            Update,
            MainMenuSystemSet
                .run_if(in_state(AppState::MainMenu))
        );
        
        app
            .add_systems(OnEnter(MainMenuState::Splash), (startup_splash_title, startup_logs_system).chain());

        app
            .add_systems(OnEnter(MainMenuState::Entry), show_title_screen_system);

        app
            .add_systems(Update, get_title_screen_system.in_set(MainMenuSystemSet).run_if(in_state(MainMenuState::Entry)));

        app
            .add_systems(Update, show_logs_system.in_set(MainMenuSystemSet).run_if(in_state(MainMenuState::Entry)))

            .add_systems(Update, show_main_menu_system.in_set(MainMenuSystemSet).run_if(in_state(MainMenuState::MainMenu)))
            .add_systems(Update, show_choose_stage_system.in_set(MainMenuSystemSet).run_if(in_state(MainMenuState::ChooseStage)))
            .add_systems(Update, show_options_system.in_set(MainMenuSystemSet).run_if(in_state(MainMenuState::Options)));

        app
            .add_systems(Update, show_restart_screen_system.run_if(in_state(AppState::GameOver)));

        app
            .add_systems(OnExit(MainMenuState::Entry), despawn_screen::<OnSplashScreen>);
    }
}