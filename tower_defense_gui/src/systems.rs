use crate::components::*;
use crate::resources::*;
use bevy::asset::RenderAssetUsages;
use bevy::math::vec2;
use bevy::render::camera::ScalingMode;
use bevy::render::mesh::PrimitiveTopology;
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;
use bevy::{core_pipeline::core_2d::Camera2d, ecs::system::*, prelude::*};
use tower_defense_plugin::components::Creep;
use tower_defense_plugin::components::FollowerBullet;
use tower_defense_plugin::components::TurretType;
use tower_defense_plugin::events::BasicFireEvent;
use tower_defense_plugin::events::MapChangedEvent;
use tower_defense_plugin::events::NewTurretEvent;
use tower_defense_plugin::events::PlaceTurretEvent;
use tower_defense_plugin::*;

pub fn setup<T>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map: Res<T>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) where
    T: Resource + Map,
{
    init_assets(
        &mut commands,
        &mut meshes,
        &mut materials,
        &asset_server,
        &mut texture_atlas_layouts,
    );

    init_path(&mut commands, &mut meshes, &mut materials, map);

    commands.spawn((
        Camera2d,
        MainCamera,
        Projection::from(OrthographicProjection {
            near: -1000.0,
            far: 1000.0,
            scale: 1.0,
            area: Rect::new(-100.0, -100.0, 100.0, 100.0),
            viewport_origin: Vec2::new(0.5, 0.5),
            scaling_mode: ScalingMode::AutoMin {
                min_width: 110.0,
                min_height: 110.0,
            },
        }),
    ));
    commands.spawn((
        Transform::from_xyz(-45.0, -45.0, 10.0),
        Visibility::default(),
        MapAnchor,
    ));
}

fn init_assets(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("smoke05.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 11, 15, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.insert_resource(TowerAssets {
        mesh: meshes.add(Circle::new(4.5)),
        material: materials.add(Color::srgb(0.5, 0.5, 0.5)),
        bomb_material: materials.add(Color::srgb(1.0, 0.9, 0.8)),
        follower_material: materials.add(Color::srgb(1.0, 0.5, 0.8)),
        slow_material: materials.add(Color::srgb_u8(100, 100, 250)),
        fire_image: asset_server.load("shots/shotLarge.png"),
        smoke_image: texture,
        smoke_atlas_layout: texture_atlas_layout,
    });

    commands.insert_resource(BulletAssets {
        mesh: meshes.add(Circle::new(1.0)),
        material: materials.add(Color::srgb(0.9, 0.4, 0.7)),
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
}

fn init_path<T>(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    map: Res<T>,
) where
    T: Resource + Map,
{
    let path_assets = PathAssets {
        mesh: meshes.add(Rectangle::new(3.0, 3.0)),
        material: materials.add(Color::srgb_u8(218, 165, 35)),
        start_mesh: meshes.add(Rectangle::new(8.0, 8.0)),
        start_material: materials.add(Color::srgb_u8(0, 165, 0)),
        end_mesh: meshes.add(Rectangle::new(8.0, 8.0)),
        end_material: materials.add(Color::srgb_u8(165, 0, 0)),
    };

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(100.0, 100.0))),
        MeshMaterial2d(materials.add(Color::srgb_u8(85, 20, 10))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    draw_path(
        commands,
        &path_assets.mesh,
        &path_assets.material,
        &map,
        Vec2::new(-45.0, -45.0),
        meshes,
    );

    commands.spawn((
        Mesh2d(path_assets.start_mesh.clone()),
        MeshMaterial2d(path_assets.start_material.clone()),
        Transform::from_xyz(
            map.get_start().x as f32 * 10.0 - 45.0,
            map.get_start().y as f32 * 10.0 - 45.0,
            1.0,
        ),
    ));

    commands.spawn((
        Mesh2d(path_assets.end_mesh.clone()),
        MeshMaterial2d(path_assets.end_material.clone()),
        Transform::from_xyz(
            map.get_end().x as f32 * 10.0 - 45.0,
            map.get_end().y as f32 * 10.0 - 45.0,
            1.0,
        ),
    ));

    commands.insert_resource(path_assets);
}

pub fn mouse_input(
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    map_anchor_query: Query<&Transform, With<MapAnchor>>,
    mut turret_events: EventWriter<PlaceTurretEvent>,
) {
    let mut turret_type: Option<TurretType> = None;
    if buttons.just_pressed(MouseButton::Left) {
        turret_type = Some(TurretType::Basic);
    } else if buttons.just_pressed(MouseButton::Right) {
        turret_type = Some(TurretType::Follower);
    } else if buttons.just_pressed(MouseButton::Middle) {
        turret_type = Some(TurretType::Bomb);
    }

    if let Some(turret_type) = turret_type {
        if let Ok((camera, camera_transform)) = q_camera.single() {
            if let Ok(window) = q_windows.single() {
                if let Some(cursor) = window.cursor_position() {
                    if let Ok(position) = camera.viewport_to_world_2d(camera_transform, cursor) {
                        if let Ok(map_anchor) = map_anchor_query.single() {
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
                                turret_type,
                                position: pos,
                            });
                        }
                    }
                }
            }
        }
    }
}

pub fn new_turrets(
    mut commands: Commands,
    tower_assets: Res<TowerAssets>,
    mut events: EventReader<NewTurretEvent>,
    map_anchor_query: Query<&Transform, With<MapAnchor>>,
) {
    if let Ok(map_anchor) = map_anchor_query.single() {
        let grid_origin = map_anchor.translation.truncate();

        for event in events.read() {
            let turret_id = commands
                .spawn((
                    Mesh2d(tower_assets.mesh.clone()),
                    Transform::from_xyz(
                        (event.position.x as f32) * 10.0 + grid_origin.x,
                        (event.position.y as f32) * 10.0 + grid_origin.y,
                        50.0,
                    ),
                ))
                .id();
            match event.turret_type {
                TurretType::Basic => {
                    commands
                        .entity(turret_id)
                        .insert(MeshMaterial2d(tower_assets.material.clone()));
                }
                TurretType::Bomb => {
                    commands
                        .entity(turret_id)
                        .insert(MeshMaterial2d(tower_assets.bomb_material.clone()));
                }
                TurretType::Follower => {
                    commands
                        .entity(turret_id)
                        .insert(MeshMaterial2d(tower_assets.follower_material.clone()));
                }
                TurretType::Slow => {
                    commands
                        .entity(turret_id)
                        .insert(MeshMaterial2d(tower_assets.slow_material.clone()));
                }
            }
        }
    }
}

pub fn update_path(
    mut commands: Commands,
    q_path: Query<Entity, With<Path>>,
    path_assets: Res<PathAssets>,
    map: Res<FreeMap>,
    map_anchor_query: Query<&Transform, With<MapAnchor>>,
    mut events: EventReader<MapChangedEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if events.read().len() != 0 {
        if let Ok(map_anchor) = map_anchor_query.single() {
            let grid_origin = map_anchor.translation.truncate();
            q_path.iter().for_each(|e| commands.entity(e).despawn());
            draw_path::<FreeMap>(
                &mut commands,
                &path_assets.mesh,
                &path_assets.material,
                &map,
                grid_origin,
                &mut meshes,
            );
        }
    }
}

pub fn draw_path<T>(
    commands: &mut Commands,
    mesh: &Handle<Mesh>,
    material: &Handle<ColorMaterial>,
    map: &Res<T>,
    grid_origin: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
) where
    T: Resource + Map,
{
    for pos in &map.get_path()[1..map.get_path().len() - 1] {
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
    let points: Vec<Vec2> = map
        .get_path()
        .iter()
        .map(|pos| {
            Vec2::new(
                (pos.x as f32) * 10.0 + grid_origin.x,
                (pos.y as f32) * 10.0 + grid_origin.y,
            )
        })
        .collect();

    let mut mesh = Mesh::new(PrimitiveTopology::LineStrip, RenderAssetUsages::all());
    let mut vertices = Vec::new();
    for point in &points {
        vertices.push([point.x, point.y, 0.0]);
    }
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    let mesh_handle = meshes.add(mesh);

    commands.spawn((
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Path {},
    ));
}

pub fn handle_new_creep(
    mut commands: Commands,
    mut query: Query<(Entity, &Creep), Added<Creep>>,
    map_anchor_query: Query<(Entity, &MapAnchor)>,
    creep_assets: Res<CreepAssets>,
) {
    if let Ok((anchor, _)) = map_anchor_query.single() {
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
}

pub fn handle_new_bullets(
    mut commands: Commands,
    mut query: Query<Entity, Added<FollowerBullet>>,
    map_anchor_query: Query<(Entity, &MapAnchor)>,
    bullet_assets: Res<BulletAssets>,
) {
    if let Ok((anchor, _)) = map_anchor_query.single() {
        for entity in &mut query {
            commands.entity(entity).insert_if_new((
                Mesh2d(bullet_assets.mesh.clone()),
                MeshMaterial2d(bullet_assets.material.clone()),
            ));
            commands.entity(entity).set_parent(anchor);
        }
    }
}

pub fn despawn_dead_bullets(mut commands: Commands, query: Query<(Entity, &FollowerBullet)>) {
    for (entity, bullet) in &query {
        if commands.get_entity(bullet.target).is_err() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn health_bar_system(
    q_creep: Query<(&Creep, &Children), Changed<Creep>>,
    mut q_health_bar: Query<&mut Sprite, With<HealthBar>>,
) {
    for (creep, children) in &q_creep {
        for &child in children.into_iter() {
            if let Ok(mut sprite) = q_health_bar.get_mut(child) {
                sprite.custom_size = Some(Vec2::new(1.5, creep.health / creep.max_health * 8.0));
            }
        }
    }
}

pub fn handle_fire_event(
    mut commands: Commands,
    mut fire_events: EventReader<BasicFireEvent>,
    tower_assets: Res<TowerAssets>,
    map_anchor_query: Query<(Entity, &MapAnchor)>,
) {
    if let Ok((anchor, _)) = map_anchor_query.single() {
        for event in fire_events.read() {
            let direction = (event.target - grid_to_world(event.origin)).normalize();
            let angle = direction.y.atan2(direction.x);

            let fire = create_fire_entity(&mut commands, &tower_assets, event.origin, angle);
            let smoke = create_smoke_entity(&mut commands, &tower_assets, event.target);

            commands.entity(anchor).add_child(fire);
            commands.entity(anchor).add_child(smoke);
        }
    }
}

fn create_fire_entity(
    commands: &mut Commands,
    tower_assets: &Res<TowerAssets>,
    origin: IVec2,
    angle: f32,
) -> Entity {
    let fire = commands
        .spawn(FireBundle {
            fire: Fire { time_left: 0.1 },
            sprite: Sprite {
                image: tower_assets.fire_image.clone(),
                custom_size: Some(Vec2::new(3.5, 1.5)),
                anchor: Anchor::CenterLeft,
                ..Default::default()
            },
            transform: Transform {
                translation: grid_to_world(origin).extend(100.0),
                rotation: Quat::from_rotation_z(angle),
                ..Default::default()
            },
        })
        .id();
    fire
}

fn create_smoke_entity(
    commands: &mut Commands,
    tower_assets: &Res<TowerAssets>,
    target: Vec2,
) -> Entity {
    let animation_indices = AnimationIndices {
        first: 66,
        last: 66 + 10,
    };

    let smoke = commands
        .spawn((
            Sprite {
                custom_size: Some(Vec2::new(8.0, 8.0)),
                ..Sprite::from_atlas_image(
                    tower_assets.smoke_image.clone(),
                    TextureAtlas {
                        layout: tower_assets.smoke_atlas_layout.clone(),
                        index: animation_indices.first,
                    },
                )
            },
            Transform {
                translation: target.extend(100.0),
                ..Default::default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .id();
    smoke
}

pub fn update_fire(mut commands: Commands, mut query: Query<(Entity, &mut Fire)>, time: Res<Time>) {
    for (entity, mut fire) in query.iter_mut() {
        fire.time_left -= time.delta_secs();
        if fire.time_left <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn animate_sprite(
    mut commands: Commands,
    mut query: Query<(Entity, &AnimationIndices, &mut Sprite)>,
) {
    for (entity, indices, mut sprite) in &mut query {
        if let Some(atlas) = &mut sprite.texture_atlas {
            if atlas.index != indices.last {
                atlas.index += 1;
            } else {
                commands.entity(entity).despawn();
            };
        }
    }
}
