//use crate::systems::*;
use crate::systems::*;
use bevy::{app::*, color::Color, render::camera::ClearColor};

pub mod components;
// pub mod events;
mod resources;
mod systems;

pub struct TowerDefenseGui;

impl Plugin for TowerDefenseGui {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_systems(Startup, setup);
    }
}
