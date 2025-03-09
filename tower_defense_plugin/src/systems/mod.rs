use bevy::color::Color;
use bevy::math::vec2;
use bevy::sprite::Sprite;
use bevy::{ecs::system::*, time::Time, transform::components::Transform};

use crate::components::*;

pub fn setup(mut commands: Commands) {
    commands.spawn(Creep {
        sprite: Sprite {
            ..Sprite::from_color(Color::srgb(0.25, 0.25, 0.75), vec2(20.0, 20.0))
        },
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
        if let Some(waypoint) = creep.waypoints.last() {
            let distance = creep.pos.distance(*waypoint);
            let delta = creep.speed * time.delta_secs();
            if delta < distance {
                creep.pos = creep.pos.move_towards(*waypoint, delta);
            } else {
                // TODO move delta and loop over distance
                creep.pos = *waypoint;
                creep.waypoints.pop();
            }
        }
    }
}
