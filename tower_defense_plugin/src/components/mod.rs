use bevy::prelude::*;

#[derive(Bundle)]
pub struct Creep {
    pub moving_entity: MovingEntity,
    pub transform: Transform,
}

#[derive(Component)]
pub struct MovingEntity {
    pub waypoints: Vec<Vec2>,
    pub speed: f32,
    pub pos: Vec2,
}
