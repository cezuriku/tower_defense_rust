use bevy::{ecs::system::*, time::Time, transform::components::Transform};

use crate::components::*;
use crate::resources::*;

pub fn setup(mut commands: Commands) {
    // commands.spawn(Creep {
    // sprite: Sprite {
    // anchor: bevy::sprite::Anchor::TopLeft,
    // ..Sprite::from_color(Color::srgb(0.25, 0.25, 0.75), Vec2::new(50.0, 50.0))
    // },
    // transform: Transform::from_xyz(0.0, 0.0, 0.0),
    // velocity: Velocity {
    // direction: Vec2 { x: 40.0, y: 40.0 },
    // },
    // });
}

pub fn move_creeps(mut creeps: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut creeps {
        let delta = velocity.direction * time.delta_secs();
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
    }
}
