use bevy::prelude::*;
use bevy_egui::*;

use crate::models::resources::counter::CounterInformation;
use crate::models::player::Player;
use crate::models::resources::ui_states::hud_state::HudState;
use crate::models::resources::world::game_time::GameTime;
use crate::models::resources::world::active_stage::ActiveStage;

pub fn show_hud_system(
    mut egui_context: EguiContexts,
    game_time: Res<GameTime>,
    state: Res<HudState>,
    spawn_stage_state: Res<ActiveStage>,
    counter_info: Res<CounterInformation>)
{
    let mut counter = 10;

    let mut images: Vec<egui::TextureId> = Vec::new();
    for handle in state.image_handles.iter() {
        counter -= 1;
        let image = egui_context.add_image(handle.clone_weak());
        images.push(image);

        if counter == 0 {
            break;
        }
    }

    let screen_size = egui_context.ctx_mut().screen_rect();

    let mod_size = screen_size.width() * 0.05;
    let mod_hud_width = screen_size.width() * 0.12;
    let mod_hud_height = screen_size.height() * 0.75;
    let screen_padding = screen_size.width() * 0.02;

    egui::Area::new("Hud Area")
        .anchor(egui::Align2::LEFT_CENTER, egui::Vec2::from([screen_padding, 0.0]))
        .show(egui_context.ctx_mut(), |ui| {
            ui.set_max_size(egui::Vec2::new(mod_hud_width, mod_hud_height));
            ui.set_min_size(egui::Vec2::new(mod_hud_width, mod_hud_height));
            ui.horizontal_wrapped(|ui| {
                for image in images.iter() {
               //     ui.image(*image, egui::Vec2::new(mod_size, mod_size));
                }
            });
        });

    let minutes = spawn_stage_state.phase_time as i32 / 60;
    let seconds = spawn_stage_state.phase_time as i32  % 60;

    egui::Area::new("Game Data Area")
        .anchor(egui::Align2::CENTER_TOP, egui::Vec2::from([0.0, screen_padding]))
        .show(egui_context.ctx_mut(), |ui| {
            ui.label(egui::RichText::new(format!("{:02}:{:02} / Stage {02}", minutes, seconds, spawn_stage_state.current_spawn_phase))
                .heading()
                .color(egui::Color32::from_rgb(255, 0, 0))
                .size(20.0)
            );
        });
}


