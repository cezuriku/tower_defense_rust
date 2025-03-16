use bevy::prelude::*;

#[derive(Component)]
pub struct Path {}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct MapAnchor;

#[derive(Component)]
pub struct HealthBar {}

#[derive(Component)]
pub struct Fire {
    pub time_left: f32,
}

#[derive(Bundle)]
pub struct FireBundle {
    pub fire: Fire,
    pub sprite: Sprite,
    pub transform: Transform,
}
