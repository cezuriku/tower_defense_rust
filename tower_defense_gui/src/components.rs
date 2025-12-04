use bevy::{prelude::*, sprite::Anchor};

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
    pub anchor: Anchor,
}

#[derive(Component)]
pub struct AnimationIndices {
    pub(crate) first: usize,
    pub(crate) last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
