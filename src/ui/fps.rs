use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::utils::Duration;

use bevy_egui::*;

const UPDATE_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Resource, Default)]
pub struct FrameRate(pub f64);

#[derive(Resource)]
pub struct ScreenDiagsState {
  pub timer: Timer,
  pub update_now: bool,
}

impl Default for ScreenDiagsState {
  fn default() -> Self {
    Self {
      timer: Timer::new(UPDATE_INTERVAL, TimerMode::Repeating),
      update_now: true,
    }
  }
}

impl ScreenDiagsState {
  pub fn enable(&mut self) {
    self.timer.unpause();
    self.update_now = true;
  }

  pub fn disable(&mut self) {
    self.timer.pause();
    self.update_now = true;
  }

  pub fn enabled(&self) -> bool {
    !self.timer.paused()
  }
}

pub fn update_frame_rate(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    state_resource: Option<ResMut<ScreenDiagsState>>,
    mut frame_rate: ResMut<FrameRate>,
  ) {
    if let Some(mut state) = state_resource {
      if state.update_now || state.timer.tick(time.delta()).just_finished() {
        if state.timer.paused() {
          return;
        } else {
          let fps_diags = extract_fps(&diagnostics);
  
          if let Some(fps) = fps_diags {
            frame_rate.0 = fps;
          } else {
            frame_rate.0 = 0.0;
          }
        }
      }
    }
}

fn extract_fps(diagnostics: &DiagnosticsStore) -> Option<f64> {
    diagnostics
      .get(FrameTimeDiagnosticsPlugin::FPS)
      .and_then(|fps| fps.average())
}

pub fn update_frame_rate_text(
    mut egui_context: EguiContexts,
    time: Res<Time>,
    state_resource: Option<ResMut<ScreenDiagsState>>,
    frame_rate: Res<FrameRate>,
  ) {
    let screen_size = egui_context.ctx_mut().screen_rect();
    let mod_hud_height = screen_size.height() * 0.75;

    if let Some(mut state) = state_resource {
      if state.update_now || state.timer.tick(time.delta()).just_finished() {
        //info!("FPS {:?}", frame_rate.0);
        
        egui::Area::new("FPS AREA")
        .anchor(egui::Align2::CENTER_TOP, egui::Vec2::from([mod_hud_height, 0.0]))
        .show(egui_context.ctx_mut(), |ui| {
            ui.label(egui::RichText::new(format!("FPS {:.2}", frame_rate.0))
                .heading()
                .color(egui::Color32::from_rgb(255, 0, 0))
                .size(20.0)
            );
        });
      }
    }
  }