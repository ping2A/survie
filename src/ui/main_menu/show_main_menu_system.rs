use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::*;
use bevy_egui::egui::*;

use crate::models::main_camera::BACKGROUND;

use crate::MainMenuState;
use crate::assets_handling::preload_window_system::WindowConfigHandles;

pub fn show_main_menu_system(
    asset_loader: Res<AssetServer>,
    window_handles: ResMut<WindowConfigHandles>,
    mut egui_context: EguiContexts,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
    mut exit: EventWriter<AppExit>,

) {
    let screen_size = egui_context.ctx_mut().screen_rect();
    let image_handle = asset_loader.load(window_handles.config.background_path.clone());
    let background_image = egui_context.add_image(image_handle);

    let background: [u8; 4] = BACKGROUND.as_rgba_u8();
    //background color
    let background_color = Frame {
        fill: Color32::from_rgb(background[0], background[1], background[2]),
        ..Default::default()
    };
    let frame = Frame {
        fill: Color32::from_rgba_premultiplied(0, 0, 0, 0),
        ..Default::default()
    };

    //main window background
    CentralPanel::default()
        .frame(background_color)
        .show(egui_context.ctx_mut(), |ui| {
            ui.image(egui::load::SizedTexture::new(background_image, vec2(screen_size.width(), screen_size.height())));
        });

    CentralPanel::default()
        .frame(frame)
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.allocate_space(egui::Vec2::new(0.0, 100.0));

                ui.set_min_size(egui::Vec2::new(0.0, 100.0));

                ui.label(RichText::new("Main menu")
                    .heading()
                    .color(Color32::from_rgb(255, 255, 255))
                    .size(50.0)
                );
            });

            ui.vertical_centered(|ui| {
                ui.set_min_width(100.0);
                ui.set_max_width(400.0);
                ui.vertical_centered_justified(|ui| {
                    if ui.add(egui::Button::new(
                        RichText::new("Start Game")
                            .color(Color32::from_rgb(255, 255, 255))
                            .size(30.0))
                    ).clicked() {
                        next_menu_state.set(MainMenuState::ChooseStage);
                    };

                    ui.add_space(50.0);

                    if ui.add(egui::Button::new(
                        RichText::new("Option")
                            .color(Color32::from_rgb(255, 255, 255))
                            .size(30.0))
                    ).clicked() {
                        next_menu_state.set(MainMenuState::Options);
                    };

                    ui.add_space(50.0);

                    if ui.add(egui::Button::new(
                        RichText::new("Quit game")
                            .color(Color32::from_rgb(255, 255, 255))
                            .size(30.0))
                    ).clicked() {
                        exit.send(AppExit);
                    };
                });
            });
        });
}