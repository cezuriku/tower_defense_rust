use bevy::math::vec2;
use bevy::{ecs::system::*, time::Time, transform::components::Transform};

use crate::components::*;

pub fn setup(mut commands: Commands) {
    commands.spawn(CreepBundle {
        moving_entity: MovingEntity {
            pos: vec2(15.0, 15.0),
            speed: 20.0,
            waypoints: vec![vec2(285.0, 285.0), vec2(75.0, 125.0), vec2(40.0, 40.0)],
        },
        transform: Transform::from_xyz(0.0, 0.0, 200.0),
        creep: Creep {},
    });
}

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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use bevy::{prelude::*, time::TimeUpdateStrategy};

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
