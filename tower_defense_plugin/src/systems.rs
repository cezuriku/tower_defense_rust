use bevy::prelude::*;
use bevy::{time::Time, transform::components::Transform};

use crate::components::*;
use crate::events::*;
use crate::map::Map;
use crate::resources::*;

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

pub fn handle_turret_placement(
    mut commands: Commands,
    mut events: EventReader<PlaceTurretEvent>,
    mut game_data: ResMut<GameData>,
    mut map: ResMut<Map>,
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
                fire_rate: 1.0,
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

pub fn spawn_creeps(
    mut commands: Commands,
    time: Res<Time>,
    mut last_spawn_time: Local<f32>,
    map: Res<Map>,
) {
    *last_spawn_time += time.delta_secs();

    if *last_spawn_time > 5.0 {
        *last_spawn_time = 0.0;

        let start_pos = map.start;
        let waypoints: Vec<Vec2> = map
            .path
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
            Creep { health: 40.0 },
        ));
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
