use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::{PrimaryWindow, WindowResolution, WindowMode, WindowResized};
use bevy::core_pipeline::tonemapping::Tonemapping;

use crate::models::main_camera::MainCamera;
use crate::models::main_camera::Zoom;

use crate::models::events::logs_event::SplashLogEvent;


use crate::assets_handling::preload_window_system::WindowConfigHandles;

pub fn setup_camera_system(
    mut commands: Commands,
    mut log: EventWriter<SplashLogEvent>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    window_handles: Res<WindowConfigHandles>,
    zoom: Res<Zoom>,
) {
    log.send("Setting up camera...".into());
    info!("Setup camera {:?}", zoom);

    let mut camera_bundle = Camera2dBundle {
        camera: Camera {
          hdr: true, // 1. HDR is required for bloom
          ..default()
        },
        tonemapping: Tonemapping::TonyMcMapface, /* 2. Using a tonemapper that desaturates to white
                                                  *    is recommended */
        ..default()
      };
    camera_bundle.transform.translation.x = 0.0;
    camera_bundle.transform.translation.y = 0.0;
    camera_bundle.transform.translation.z = 400000000000000.0;

    camera_bundle.projection.scale = 4.0;
    camera_bundle.projection.far = 1000000000000000.0;

 //   camera_bundle.projection.far = 10000000000000000000000000.0;

    let width = window_handles.config.width;
    let height = window_handles.config.height;

//    camera_bundle.projection.scaling_mode = ScalingMode::Fixed { width: width, height: height };

    commands.spawn(camera_bundle)
        .insert(MainCamera)
        .insert(Name::new("MainCamera"));

    for mut window in window_query.iter_mut() {
        window.title = String::from(window_handles.config.title.clone());
       // window.resolution = WindowResolution::new(width * zoom.0, height * zoom.0);
        window.mode = WindowMode::Windowed;
        window.resizable = true;
    }
}

pub fn on_zoom_changed_system(
    mut camera: Query<&mut Transform, With<MainCamera>>,
    window_handles: Res<WindowConfigHandles>,
    mut window_query: Query<&mut bevy::prelude::Window, With<PrimaryWindow>>,
    zoom: ResMut<Zoom>,
    mut resize_reader: EventReader<WindowResized>,
) {

    let mut camera_transform = match camera.get_single_mut() {
        Ok(value) => value,
        Err(_) => return,
    };

    let primary_window = match window_query.get_single_mut() {
        Ok(value) => value,
        Err(_) => return,
    };

    let x = window_handles.config.width / primary_window.width() * zoom.0;
    let y = window_handles.config.height / primary_window.height() * zoom.0;

    camera_transform.scale.x = x.min(y);
    camera_transform.scale.y = y.min(y);

    for e in resize_reader.read() {
        println!("Resize");
    }
}