use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use super::components::Node;

pub fn place_node(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>
) -> Result{
    let (camera, cam_tf) = camera.into_inner();

    if mouse.just_pressed(MouseButton::Left) {
        println!("Placing node...");
        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok(world_pos) = camera.viewport_to_world_2d(cam_tf, cursor_pos) {
                // snap to nearest 50
                let snapped = (world_pos / 50.0).round() * 50.0;

                commands.spawn((
                    Node { pos: snapped },
                    
                    Mesh2d(meshes.add(Circle::new(8.))),
                    MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.2, 0.1, 0.0)))),
                    Transform::from_translation(snapped.extend(0.0)),
                ));

                info!("Node placed at {:?}", snapped);
            }
        }
    }
    Ok(())
}
