use bevy::prelude::*;
use systems::*;

mod components;
mod resources;
mod systems;

pub struct TowerDefenseGui;

impl Plugin for TowerDefenseGui {
    fn build(&self, app: &mut App) {
        // Add events

        // Insert resources
        app.insert_resource(ClearColor(Color::BLACK));

        // Add systems
        app.add_systems(Startup, setup)
            .add_systems(Update, (mouse_input, new_turrets, handle_new_creep));
    }
}
