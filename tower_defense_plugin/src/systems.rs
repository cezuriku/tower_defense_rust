use bevy::prelude::*;
use bevy::{time::Time, transform::components::Transform};

use crate::map::FreeMap;
use crate::resources::*;
use crate::utils::world_to_grid;
use crate::{Map, events::*};
use crate::{MapTrait, components::*};

pub fn setup() {}

pub fn move_creeps(mut creeps: Query<(&mut MovingEntity, &mut Transform)>, time: Res<Time>) {
    for (mut creep, mut transform) in &mut creeps {
        let mut delta = creep.speed * time.delta_secs();
        while delta > 0.0 && !creep.waypoints.is_empty() {
            if let Some(waypoint) = creep.waypoints.last() {
                let distance = transform.translation.distance(waypoint.extend(0.0));
                if delta < distance {
                    transform.translation = transform
                        .translation
                        .move_towards(waypoint.extend(0.0), delta);
                    delta = 0.0;
                } else {
                    transform.translation = waypoint.extend(0.0);
                    creep.waypoints.pop();
                    delta -= distance;
                }
            }
        }
    }
}

pub fn update_creep_paths(
    mut events: EventReader<MapChangedEvent>,
    mut creeps: Query<(&Transform, &mut MovingEntity), With<Creep>>,
    map: Res<Map<FreeMap>>,
) {
    for _event in events.read() {
        for (transform, mut moving_entity) in creeps.iter_mut() {
            let start = world_to_grid(transform.translation);
            if let Some((new_path, _)) = map.compute_path(&start) {
                let mut waypoints: Vec<Vec2> = new_path
                    .iter()
                    .map(|pos| Vec2::new(pos.x as f32 * 10.0, pos.y as f32 * 10.0))
                    .rev()
                    .collect();
                waypoints.pop();

                moving_entity.waypoints = waypoints;
            } else {
                // Do not update the path and let the creep continue on its current path even though it is likely to go through walls
            }
        }
    }
}

pub fn handle_turret_placement<T: MapTrait + Send + Sync + 'static>(
    mut commands: Commands,
    mut events: EventReader<PlaceTurretEvent>,
    mut game_data: ResMut<GameData>,
    mut map: ResMut<Map<T>>,
    mut new_turret_writer: EventWriter<NewTurretEvent>,
    mut map_changed_writer: EventWriter<MapChangedEvent>,
) {
    for event in events.read() {
        let cost = match event.turret_type {
            TurretType::Basic => 50,
            TurretType::Advanced => 100,
        };

        if game_data.gold >= cost && map.is_turret_possible(&event.position) {
            // Deduct gold
            game_data.gold -= cost;

            map.place_tower(&event.position);

            commands.spawn((Turret {
                turret_type: event.turret_type,
                position: event.position,
                transform: Transform::from_xyz(
                    event.position.x as f32 * 10.0,
                    event.position.y as f32 * 10.0,
                    0.0,
                ),
                range: 25.0,
                damage: 10.0,
                reload_time: 1.0,
                last_fired: 0.0,
            },));

            new_turret_writer.send(NewTurretEvent {
                turret_type: event.turret_type,
                position: event.position,
            });

            map_changed_writer.send(MapChangedEvent {});

            println!("Turret placed successfully!");
        } else {
            println!(
                "Insufficient resources to place turret! Cost: {cost} Gold: {} ",
                game_data.gold
            );
        }
    }
}

pub fn spawn_creeps<T>(
    mut commands: Commands,
    time: Res<Time>,
    mut last_spawn_time: Local<f32>,
    map: Res<Map<T>>,
) where
    T: MapTrait + Send + Sync + 'static,
{
    *last_spawn_time += time.delta_secs();

    if *last_spawn_time > 2.0 {
        *last_spawn_time = 0.0;

        let start_pos = map.get_start();
        let waypoints: Vec<Vec2> = map
            .get_path()
            .iter()
            .map(|pos| Vec2::new(pos.x as f32 * 10.0, pos.y as f32 * 10.0))
            .rev()
            .collect();

        commands.spawn((
            MovingEntity {
                speed: 20.0,
                waypoints,
            },
            Transform::from_translation(Vec3::new(
                start_pos.x as f32 * 10.0,
                start_pos.y as f32 * 10.0,
                0.0,
            )),
            Creep {
                health: 100.0,
                max_health: 100.0,
            },
        ));
    }
}

// System to shoot creeps within range
pub fn shoot_creeps(
    time: Res<Time>,
    mut turrets: Query<&mut Turret>,
    mut creeps: Query<(Entity, &mut Creep, &Transform)>,
    mut commands: Commands,
    mut fire_events: EventWriter<BasicFireEvent>,
) {
    for mut turret in turrets.iter_mut() {
        let turret_world_position = turret.transform.translation.truncate();

        // Update last fired time
        turret.last_fired += time.delta_secs();

        if turret.last_fired >= turret.reload_time {
            for (creep_entity, mut creep, creep_transform) in creeps.iter_mut() {
                let creep_position = creep_transform.translation.truncate();
                let distance = turret_world_position.distance(creep_position);

                if turret.last_fired >= turret.reload_time
                    && distance <= turret.range
                    && creep.health > 0.0
                {
                    // Reduce creep health
                    creep.health -= turret.damage;

                    turret.last_fired = 0.0;

                    let kill: bool = creep.health <= 0.0;
                    if kill {
                        println!("Creep killed by turret!");
                        commands.entity(creep_entity).despawn_recursive();
                    } else {
                        println!("Creep hit by turret! Health: {}", creep.health);
                    }

                    fire_events.send(BasicFireEvent {
                        origin: turret.position,
                        target: creep_position,
                        kill,
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use bevy::time::TimeUpdateStrategy;

    #[test]
    fn test_move_creeps() {
        let mut app = App::new();
        let next_waypoint = Vec2::new(15.0, 4000.0);
        let fixed_delta = 0.1;
        let speed = 10.0;

        app.add_plugins(MinimalPlugins)
            .add_systems(Update, move_creeps)
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f32(
                fixed_delta,
            )));

        app.world_mut().spawn((
            MovingEntity {
                speed,
                waypoints: vec![Vec2::new(200.0, 15.0), next_waypoint],
            },
            Transform::from_translation(Vec3::new(15.0, 15.0, 0.0)),
        ));

        app.update();

        let world = app.world_mut();
        let mut query = world.query::<(&MovingEntity, &Transform)>();
        let (_creep, transform) = query.single(world);

        assert_eq!(transform.translation.truncate(), Vec2::new(15.0, 15.0)); // No movement since delta time is 0

        app.update();

        let world = app.world_mut();
        let mut query = world.query::<(&MovingEntity, &Transform)>();
        let (_creep, transform) = query.single(world);

        // Update the expected position based on the movement logic in `move_creeps`
        let expected_pos = Vec2::new(15.0, 15.0).move_towards(next_waypoint, fixed_delta * speed);
        assert_eq!(transform.translation.truncate(), expected_pos); // Check if the position has updated correctly
    }
}
