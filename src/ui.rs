use bevy::{prelude::*, window::PrimaryWindow, render::camera::Viewport};
use bevy_egui::{egui, EguiContexts, EguiContext};

pub fn terminal_ui(
    mut contexts: EguiContexts,
    mut camera: Single<&mut Camera, Without<EguiContext>>,
    window: Single<&mut Window, With<PrimaryWindow>>,
) -> Result{
    let ctx = contexts.ctx_mut()?;

    let mut bottom = egui::TopBottomPanel::bottom("terminal")
        .resizable(true)
        .default_height(150.0)
        .min_height(100.0)
        .show(ctx, |ui| {
            ui.label("Events will appear here...");
        })
        .response
        .rect
        .height();
    
    bottom *= window.scale_factor();

    let pos = UVec2::new(0, 0);
    let size = UVec2::new(window.physical_width(), window.physical_height() - bottom as u32);

    camera.viewport = Some(Viewport {
        physical_position: pos,
        physical_size: size,
        ..default()
    });
    
    Ok(())
}