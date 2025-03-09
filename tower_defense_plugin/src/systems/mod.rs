use bevy::math::vec2;
use bevy::{ecs::system::*, time::Time, transform::components::Transform};

use crate::components::*;

pub fn setup(mut commands: Commands) {
    commands.spawn(Creep {
        moving_entity: MovingEntity {
            pos: vec2(15.0, 15.0),
            speed: 20.0,
            waypoints: vec![vec2(285.0, 285.0), vec2(75.0, 125.0), vec2(40.0, 40.0)],
        },
        transform: Transform::from_xyz(0.0, 0.0, 200.0),
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
                    delta = 0.0; // No more movement this frame
                } else {
                    // Move as close as possible to the waypoint
                    creep.pos = *waypoint;
                    // Remove the waypoint from the list
                    creep.waypoints.pop();
                    delta -= distance; // Reduce the remaining delta
                }
            }
        }
    }
}
