use crate::components::*;
use crate::resources::*;
use bevy::math::vec2;
use bevy::render::camera::ScalingMode;
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;
use bevy::{core_pipeline::core_2d::Camera2d, ecs::system::*, prelude::*};
use tower_defense_plugin::components::Creep;
use tower_defense_plugin::components::TurretType;
use tower_defense_plugin::events::NewTurretEvent;
use tower_defense_plugin::events::PlaceTurretEvent;
use tower_defense_plugin::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map: Res<Map>,
) {
    commands.insert_resource(TowerAssets {
        mesh: meshes.add(Rectangle::new(10.0, 10.0)),
        material: materials.add(Color::BLACK),
    });

    commands.insert_resource(CreepAssets {
        creep_sprite: Sprite {
            ..Sprite::from_color(Color::srgb(0.25, 0.25, 0.75), vec2(8.0, 8.0))
        },
        health_bar_back_sprite: Sprite {
            ..Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), vec2(1.5, 8.0))
        },
        health_bar_front_sprite: Sprite {
            ..Sprite::from_color(Color::srgb(0.0, 1.0, 0.0), vec2(1.5, 8.0))
        },
    });

    let path_assets = PathAssets {
        mesh: meshes.add(Rectangle::new(3.0, 3.0)),
        material: materials.add(Color::srgb_u8(218, 165, 35)),
        start_mesh: meshes.add(Rectangle::new(8.0, 8.0)),
        start_material: materials.add(Color::srgb_u8(0, 165, 0)),
        end_mesh: meshes.add(Rectangle::new(8.0, 8.0)),
        end_material: materials.add(Color::srgb_u8(165, 0, 0)),
    };

    draw_path(
        &mut commands,
        &path_assets.mesh,
        &path_assets.material,
        &map,
        Vec2::new(-45.0, -45.0),
    );

    commands.spawn((
        Mesh2d(path_assets.start_mesh.clone()),
        MeshMaterial2d(path_assets.start_material.clone()),
        Transform::from_xyz(
            map.start.x as f32 * 10.0 - 45.0,
            map.start.y as f32 * 10.0 - 45.0,
            1.0,
        ),
    ));

    commands.spawn((
        Mesh2d(path_assets.end_mesh.clone()),
        MeshMaterial2d(path_assets.end_material.clone()),
        Transform::from_xyz(
            map.end.x as f32 * 10.0 - 45.0,
            map.end.y as f32 * 10.0 - 45.0,
            1.0,
        ),
    ));

    commands.insert_resource(path_assets);
    commands.spawn((
        Camera2d,
        MainCamera,
        OrthographicProjection {
            near: -1000.0,
            far: 1000.0,
            scale: 1.0,
            area: Rect::new(-100.0, -100.0, 100.0, 100.0),
            viewport_origin: Vec2::new(0.5, 0.5),
            scaling_mode: ScalingMode::AutoMin {
                min_width: 110.0,
                min_height: 110.0,
            },
        },
    ));
    commands.spawn((
        Transform::from_xyz(-45.0, -45.0, 10.0),
        Visibility::default(),
        MapAnchor,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(100.0, 100.0))),
        MeshMaterial2d(materials.add(Color::srgb_u8(85, 20, 10))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

pub fn mouse_input(
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    map_anchor_query: Query<&Transform, With<MapAnchor>>,
    mut turret_events: EventWriter<PlaceTurretEvent>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera.single();
        if let Some(cursor) = q_windows.single().cursor_position() {
            if let Ok(position) = camera.viewport_to_world_2d(camera_transform, cursor) {
                let map_anchor = map_anchor_query.single();
                let grid_origin = map_anchor.translation.truncate();

                println!(
                    "position: {:?}",
                    position - grid_origin + Vec2::new(2.5, 2.5)
                );

                let pos = IVec2 {
                    x: ((position.x - grid_origin.x + 5.0) / 10.0) as i32,
                    y: ((position.y - grid_origin.y + 5.0) / 10.0) as i32,
                };

                println!("placing turret at {:?}", pos);

                turret_events.send(PlaceTurretEvent {
                    turret_type: TurretType::Basic,
                    position: pos,
                });
            }
        }
    }
}

pub fn new_turrets(
    mut commands: Commands,
    tower_assets: Res<TowerAssets>,
    mut events: EventReader<NewTurretEvent>,
    q_path: Query<Entity, With<Path>>,
    path_assets: Res<PathAssets>,
    map: Res<Map>,
    map_anchor_query: Query<&Transform, With<MapAnchor>>,
) {
    let mut should_update_path = false;
    let map_anchor = map_anchor_query.single();
    let grid_origin = map_anchor.translation.truncate();

    for event in events.read() {
        commands.spawn((
            Mesh2d(tower_assets.mesh.clone()),
            MeshMaterial2d(tower_assets.material.clone()),
            Transform::from_xyz(
                (event.position.x as f32) * 10.0 + grid_origin.x,
                (event.position.y as f32) * 10.0 + grid_origin.y,
                50.0,
            ),
        ));
        should_update_path = true;
    }
    if should_update_path {
        q_path.iter().for_each(|e| commands.entity(e).despawn());
        draw_path(
            &mut commands,
            &path_assets.mesh,
            &path_assets.material,
            &map,
            grid_origin,
        );
    }
}

pub fn draw_path(
    commands: &mut Commands,
    mesh: &Handle<Mesh>,
    material: &Handle<ColorMaterial>,
    map: &Res<Map>,
    grid_origin: Vec2,
) {
    for pos in &map.path[1..map.path.len() - 1] {
        commands.spawn((
            Mesh2d(mesh.clone()),
            MeshMaterial2d(material.clone()),
            Transform::from_xyz(
                (pos.x as f32) * 10.0 + grid_origin.x,
                (pos.y as f32) * 10.0 + grid_origin.y,
                1.0,
            ),
            Path {},
        ));
    }
}

pub fn handle_new_creep(
    mut commands: Commands,
    mut query: Query<(Entity, &Creep), Added<Creep>>,
    map_anchor_query: Query<(Entity, &MapAnchor)>,
    creep_assets: Res<CreepAssets>,
) {
    let (anchor, _) = map_anchor_query.single();
    for (creep, _) in &mut query {
        commands.entity(anchor).add_child(creep);
        commands
            .entity(creep)
            .insert_if_new(creep_assets.creep_sprite.clone());

        let health_bar = commands
            .spawn((
                creep_assets.health_bar_back_sprite.clone(),
                Transform::from_xyz(0.0, 0.0, 1.0),
            ))
            .id();
        commands.entity(creep).add_child(health_bar);

        let inner_health_bar = commands
            .spawn((
                Sprite {
                    anchor: Anchor::TopCenter,
                    ..creep_assets.health_bar_front_sprite.clone()
                },
                Transform::from_xyz(0.0, 4.0, 2.0),
                HealthBar {},
            ))
            .id();
        commands.entity(creep).add_child(inner_health_bar);
    }
}

pub fn health_bar_system(
    q_creep: Query<(&Creep, &Children), Changed<Creep>>,
    mut q_health_bar: Query<&mut Sprite, With<HealthBar>>,
) {
    for (creep, children) in &q_creep {
        for &child in children.iter() {
            if let Ok(mut sprite) = q_health_bar.get_mut(child) {
                sprite.custom_size = Some(Vec2::new(1.5, creep.health / creep.max_health * 8.0));
            }
        }
    }
}
