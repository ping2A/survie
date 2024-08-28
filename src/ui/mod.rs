use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

use crate::AppState;

use crate::ui::debug::CmdUiPlugin;

use crate::ui::main_menu::MainMenuPlugin;

use crate::ui::views::show_hud_system::show_hud_system;
use crate::ui::update::update_hud_state::update_hud_state;

use crate::ui::update::update_shop_state::update_shop_system;
use crate::ui::views::show_shop_system::show_shop_system;

use crate::ui::pause::{close_pause_system, spawn_pause_system};

use crate::ui::fps::{FrameRate, ScreenDiagsState, update_frame_rate, update_frame_rate_text};
use crate::ui::text_animation::{start_animate_text, animate_text};
use crate::ui::kill_counter::{spawn_kill_counter, despawn_kill_counter, update_kill_counter_text};
use crate::ui::highscore::{spawn_highscore, despawn_highscore, update_highscore_text};
use crate::ui::world_text::{spawn_world_texts, despawn_world_texts, animate_world_texts, SpawnWorldText};

mod pause;
mod main_menu;
mod debug;
mod views;
mod update;
mod fps;
mod text_animation;
mod kill_counter;
mod highscore;
pub mod world_text;

/// This plugin generates the UI elements for game menus and
/// the ingame hud. Furthermore it holds systems to control the
/// spawning and despawning of UI elements as well as interactions
/// like button clicks.
pub struct UiPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct UiUpdateSystemSet;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());

        app.init_resource::<FrameRate>();
        app.init_resource::<ScreenDiagsState>();

        app.add_plugins(CmdUiPlugin);
        app.add_plugins(MainMenuPlugin);

        app.configure_sets(
            Update,
            UiUpdateSystemSet
        );

        app.add_systems(Update, (start_animate_text, animate_text));

        app.add_systems(Update, update_frame_rate);
        app.add_systems(Update, update_frame_rate_text);

        app
            .add_systems(OnEnter(AppState::Paused), spawn_pause_system)
            .add_systems(OnExit(AppState::Paused), close_pause_system);

        app
            .add_systems(OnEnter(AppState::Shop), update_shop_system)
            .add_systems(Update, show_shop_system.in_set(UiUpdateSystemSet).run_if(in_state(AppState::Shop)));

        // Highscore
        app
        .add_systems(OnEnter(AppState::InGame), spawn_highscore)
        .add_systems(OnExit(AppState::InGame), despawn_highscore)
        .add_systems(Update, update_highscore_text.in_set(UiUpdateSystemSet).run_if(in_state(AppState::InGame)));


        // Kill Counter
        app
            .add_systems(OnEnter(AppState::InGame), spawn_kill_counter)
            .add_systems(OnExit(AppState::InGame), despawn_kill_counter)
            .add_systems(Update, update_kill_counter_text.in_set(UiUpdateSystemSet).run_if(in_state(AppState::InGame)));

        // Animated Text
        app
            .add_systems(
            Update,
            (spawn_world_texts).run_if(in_state(AppState::InGame)),
            )
            .add_systems(Update, (despawn_world_texts, animate_world_texts).run_if(in_state(AppState::InGame)))
            .add_event::<SpawnWorldText>();

        app
            .add_systems(Update, show_hud_system.in_set(UiUpdateSystemSet).run_if(in_state(AppState::InGame)))
            .add_systems(Update, show_hud_system.in_set(UiUpdateSystemSet).run_if(in_state(AppState::Shop)))
            .add_systems(Update, update_hud_state.in_set(UiUpdateSystemSet).run_if(in_state(AppState::InGame)));
    }
}