use bevy::{
    core_pipeline::core_2d::Camera2d,
    ecs::system::*,
    prelude::*,
};


pub fn setup(mut commands: Commands, meshes: ResMut<Assets<Mesh>>) {
    commands.spawn(Camera2d);
}
