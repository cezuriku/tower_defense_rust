use bevy::prelude::*;
use bevy::{time::Time, transform::components::Transform};

use crate::components::*;
use crate::events::*;
use crate::resources::*;

pub fn setup() {}

pub fn move_creeps(mut creeps: Query<&mut MovingEntity>, time: Res<Time>) {
    for mut creep in &mut creeps {
        let mut delta = creep.speed * time.delta_secs();
        while delta > 0.0 && !creep.waypoints.is_empty() {
            if let Some(waypoint) = creep.waypoints.last() {
                let distance = creep.pos.distance(*waypoint);
                if delta < distance {
                    creep.pos = creep.pos.move_towards(*waypoint, delta);
                    delta = 0.0;
                } else {
                    creep.pos = *waypoint;
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

            // Send NewTurretEvent
            new_turret_writer.send(NewTurretEvent {
                turret_type: event.turret_type,
                position: event.position,
            });

            println!("Turret placed successfully!");
        } else {
            println!(
                "Insufficient resources to place turret! Cost: {cost} Gold: {} ",
                game_data.gold
            );
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

        app.world_mut().spawn((MovingEntity {
            pos: Vec2::new(15.0, 15.0),
            speed,
            waypoints: vec![Vec2::new(200.0, 15.0), next_waypoint],
        },));

        app.update();

        let world = app.world_mut();
        let mut query = world.query::<&mut MovingEntity>();
        let creep = query.single_mut(world);

        assert_eq!(creep.pos, Vec2::new(15.0, 15.0)); // No movement since delta time is 0

        app.update();

        let world = app.world_mut();
        let mut query = world.query::<&mut MovingEntity>();
        let creep = query.single_mut(world);

        // Update the expected position based on the movement logic in `move_creeps`

        let expected_pos = Vec2::new(15.0, 15.0).move_towards(next_waypoint, fixed_delta * speed);
        assert_eq!(creep.pos, expected_pos); // Check if the position has updated correctly
    }
}
