use bevy::prelude::*;

#[derive(Component)]
pub struct Creep {}

#[derive(Bundle)]
pub struct CreepBundle {
    pub moving_entity: MovingEntity,
    pub transform: Transform,
    pub creep: Creep,
}

#[derive(Component)]
pub struct MovingEntity {
    pub waypoints: Vec<Vec2>,
    pub speed: f32,
    pub pos: Vec2,
}
