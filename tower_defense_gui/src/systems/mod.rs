use crate::components::*;
use crate::resources::*;
use bevy::math::vec2;
use bevy::window::PrimaryWindow;
use bevy::{core_pipeline::core_2d::Camera2d, ecs::system::*, prelude::*};
use tower_defense_plugin::components::Creep;
use tower_defense_plugin::components::MovingEntity;
use tower_defense_plugin::resources::Map;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut new_target: EventWriter<UpdatePath>,
    map: Res<Map>,
) {
    commands.insert_resource(TowerAssets {
        mesh: meshes.add(Rectangle::new(30.0, 30.0)),
        material: materials.add(Color::BLACK),
    });

    let path_assets = PathAssets {
        mesh: meshes.add(Rectangle::new(10.0, 10.0)),
        material: materials.add(Color::srgb_u8(218, 165, 35)),
        start_mesh: meshes.add(Rectangle::new(25.0, 25.0)),
        start_material: materials.add(Color::srgb_u8(0, 165, 0)),
        end_mesh: meshes.add(Rectangle::new(25.0, 25.0)),
        end_material: materials.add(Color::srgb_u8(165, 0, 0)),
    };

    // Draw the first tile
    commands.spawn((
        Mesh2d(path_assets.start_mesh.clone()),
        MeshMaterial2d(path_assets.start_material.clone()),
        Transform::from_xyz(
            map.start.x as f32 * 30.0 - 135.0,
            map.start.y as f32 * 30.0 - 135.0,
            100.0,
        ),
    ));

    // Draw the last tile
    commands.spawn((
        Mesh2d(path_assets.end_mesh.clone()),
        MeshMaterial2d(path_assets.end_material.clone()),
        Transform::from_xyz(
            map.end.x as f32 * 30.0 - 135.0,
            map.end.y as f32 * 30.0 - 135.0,
            100.0,
        ),
    ));

    commands.insert_resource(path_assets);
    commands.spawn((Camera2d, MainCamera));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(300.0, 300.0))),
        MeshMaterial2d(materials.add(Color::srgb_u8(85, 20, 10))),
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));

    new_target.send(UpdatePath {});
}

pub fn mouse_input(
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    mut path_updater: EventWriter<UpdatePath>,
    mut map: ResMut<Map>,
    tower_assets: Res<TowerAssets>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera.single();
        if let Some(cursor) = q_windows.single().cursor_position() {
            if let Ok(position) = camera.viewport_to_world_2d(camera_transform, cursor) {
                println!("Cursor is inside the primary window, at {:?}", position);

                let pos = IVec2 {
                    x: (position.x as i32 + 150) / 30,
                    y: (position.y as i32 + 150) / 30,
                };

                if map.place_tower(&pos) {
                    map.recompute_path();
                    if !map.path.is_empty() {
                        path_updater.send(UpdatePath {});

                        commands.spawn((
                            Mesh2d(tower_assets.mesh.clone()),
                            MeshMaterial2d(tower_assets.material.clone()),
                            Transform::from_xyz(
                                pos.x as f32 * 30.0 - 135.0,
                                pos.y as f32 * 30.0 - 135.0,
                                50.0,
                            ),
                        ));
                    } else {
                        map.remove_tower(&pos)
                    }
                }
            }
        } else {
            println!("Cursor is not in the game window.");
        }
    }
}

pub fn update_path(
    path_assets: Res<PathAssets>,
    mut commands: Commands,
    mut target: EventReader<UpdatePath>,
    mut map: ResMut<Map>,
    q_path: Query<Entity, With<Path>>,
) {
    for _ in target.read() {
        q_path.iter().for_each(|e| commands.entity(e).despawn());
        map.recompute_path();
        // Show the whole path except first and last point
        for pos in &map.path[1..map.path.len() - 1] {
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
}

pub fn move_creeps(mut creeps: Query<(&mut Transform, &MovingEntity)>) {
    for (mut creep, moving_entity) in &mut creeps {
        creep.translation = (moving_entity.pos - vec2(150.0, 150.0)).extend(200.0);
    }
}

pub fn reset_creeps(
    mut commands: Commands,
    mut target: EventReader<UpdatePath>,
    map: Res<Map>,
    q_move: Query<Entity, With<MovingEntity>>,
) {
    for _ in target.read() {
        q_move.iter().for_each(|e| commands.entity(e).despawn());
        commands.spawn(Creep {
            sprite: Sprite {
                ..Sprite::from_color(Color::srgb(0.25, 0.25, 0.75), vec2(20.0, 20.0))
            },
            moving_entity: MovingEntity {
                pos: vec2(15.0, 15.0),
                speed: 120.0,
                waypoints: map
                    .path
                    .iter()
                    .map(|pos| vec2(pos.x as f32 * 30.0 + 15.0, pos.y as f32 * 30.0 + 15.0))
                    .rev()
                    .collect(),
            },
            transform: Transform::from_xyz(0.0, 0.0, 200.0),
        });
    }
}
