use bevy::prelude::*;
use systems::*;
use tower_defense_plugin::{FreeMap, SimpleMap};

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
        app.add_systems(Startup, setup::<FreeMap>).add_systems(
            Update,
            (
                mouse_input,
                new_turrets,
                update_path,
                handle_new_creep,
                health_bar_system,
                handle_fire_event,
                update_fire,
                animate_sprite,
            ),
        );
    }
}

pub struct TowerDefenseGuiSimpleMap;

impl Plugin for TowerDefenseGuiSimpleMap {
    fn build(&self, app: &mut App) {
        // Add events

        // Insert resources
        app.insert_resource(ClearColor(Color::BLACK));

        // Add systems
        app.add_systems(Startup, setup::<SimpleMap>).add_systems(
            Update,
            (
                mouse_input,
                new_turrets,
                handle_new_creep,
                health_bar_system,
                handle_fire_event,
                update_fire,
                animate_sprite,
            ),
        );
    }
}
