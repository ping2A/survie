use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use bevy::input::mouse::MouseScrollUnit;

use crate::models::main_camera::{Zoom, ZOOM_LOWER_BOUND, ZOOM_UPPER_BOUND};

pub fn update_camera_zoom_system(mut scroll_evr: EventReader<MouseWheel>, mut zoom: ResMut<Zoom>)
{
    for ev in scroll_evr.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                println!("Scroll (line units): vertical: {}, horizontal: {}", ev.y, ev.x);
                zoom.0 *= 0.9_f32.powi(ev.y as i32);
                zoom.0 = zoom.0.clamp(ZOOM_LOWER_BOUND, ZOOM_UPPER_BOUND);
            }
            MouseScrollUnit::Pixel => {
                println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
                zoom.0 *= 0.9_f32.powi(ev.y as i32);
                zoom.0 = zoom.0.clamp(ZOOM_LOWER_BOUND, ZOOM_UPPER_BOUND);
            }
        }
    }
}