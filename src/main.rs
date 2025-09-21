mod ui;
mod grid;

use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass, EguiGlobalSettings, PrimaryEguiContext};
use grid::{grid::*, placement::*};
use bevy_render::view::RenderLayers;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2D Frame Editor".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        // startup
        .add_systems(Startup, (
            setup_system,
            spawn_grid,
        ))
        // main loop
        .add_systems(Update, (
            draw_grid,
            place_node,
        ))
        .add_systems(EguiPrimaryContextPass, ui::terminal_ui)
        .run();
}

fn setup_system(
    mut commands: Commands,
    mut egui_global_settings: ResMut<EguiGlobalSettings>,
) {
    egui_global_settings.auto_create_primary_context = false;
    commands.spawn(Camera2d);

    commands.spawn((
        // The `PrimaryEguiContext` component requires everything needed to render a primary context.
        PrimaryEguiContext,
        Camera2d,
        // Setting RenderLayers to none makes sure we won't render anything apart from the UI.
        RenderLayers::none(),
        Camera {
            order: 1,
            ..default()
        },
    ));
}