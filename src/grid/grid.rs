use bevy::{color::palettes::css::GRAY, prelude::*};
use super::components::Grid;

pub fn spawn_grid(mut commands: Commands) {
    // Just spawn a marker; grid lines will be drawn in a system
    commands.spawn(Grid);
}

pub fn draw_grid(
    mut gizmos: Gizmos,
    window: Single<&Window>,
) {
    let step = 50.0; // grid spacing
    let width = window.width();
    let height = window.height();

    let half_w = width / 2.0;
    let half_h = height / 2.0;

    // vertical lines
    for x in (-half_w as i32..=half_w as i32).step_by(step as usize) {
        gizmos.line_2d(
            Vec2::new(x as f32, -half_h),
            Vec2::new(x as f32, half_h),
            GRAY);
    }
    // horizontal lines
    for y in (-half_h as i32..=half_h as i32).step_by(step as usize) {
        gizmos.line_2d(
            Vec2::new(-half_w, y as f32), 
            Vec2::new(half_w, y as f32), 
            GRAY
        );
    }
}
