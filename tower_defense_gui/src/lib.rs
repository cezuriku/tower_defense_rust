use crate::resources::*;
use crate::systems::*;
use bevy::prelude::*;

pub mod components;
// pub mod events;
mod resources;
mod systems;

pub struct TowerDefenseGui;

impl Plugin for TowerDefenseGui {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_systems(Startup, setup)
            .add_event::<NewTarget>()
            .add_systems(Update, (mouse_input, update_path));
    }
}
