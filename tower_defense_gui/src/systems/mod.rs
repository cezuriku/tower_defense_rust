use crate::components::*;
use crate::resources::*;
use bevy::window::PrimaryWindow;
use bevy::{core_pipeline::core_2d::Camera2d, ecs::system::*, prelude::*};
use tower_defense_plugin::resources::Map;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(PathAssets {
        mesh: meshes.add(Rectangle::new(10.0, 10.0)),
        material: materials.add(Color::srgb_u8(218, 165, 35)),
    });
    commands.spawn((Camera2d, MainCamera));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(300.0, 300.0))),
        MeshMaterial2d(materials.add(Color::srgb_u8(85, 20, 10))),
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));
}

pub fn mouse_input(
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut new_target: EventWriter<NewTarget>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera.single();
        if let Some(cursor) = q_windows.single().cursor_position() {
            if let Ok(position) = camera.viewport_to_world_2d(camera_transform, cursor) {
                println!("Cursor is inside the primary window, at {:?}", position);

                new_target.send(NewTarget {
                    pos: IVec2 {
                        x: (position.x as i32 + 150) / 30,
                        y: (position.y as i32 + 150) / 30,
                    },
                });
            }
        } else {
            println!("Cursor is not in the game window.");
        }
    }
}

pub fn update_path(
    path_assets: Res<PathAssets>,
    mut commands: Commands,
    mut target: EventReader<NewTarget>,
    map: Res<Map>,
    q_path: Query<Entity, With<Path>>,
) {
    for ev in target.read() {
        q_path.iter().for_each(|e| commands.entity(e).despawn());

        match map.find_path(&IVec2 { x: 0, y: 0 }, &ev.pos) {
            Some(path) => {
                for pos in path.0 {
                    commands.spawn((
                        Mesh2d(path_assets.mesh.clone()),
                        MeshMaterial2d(path_assets.material.clone()),
                        Transform::from_xyz(
                            pos.x as f32 * 30.0 - 135.0,
                            pos.y as f32 * 30.0 - 135.0,
                            100.0,
                        ),
                        Path {},
                    ));
                }
            }
            None => println!("Outside of window"),
        }
    }
}
