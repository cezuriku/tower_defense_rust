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
        insert_common_resources(app);

        // Add systems
        app.add_systems(Startup, setup::<FreeMap>);
        // Systems at update
        insert_common_systems(app);
        app.add_systems(Update, update_path);
    }
}

pub struct TowerDefenseGuiSimpleMap;

impl Plugin for TowerDefenseGuiSimpleMap {
    fn build(&self, app: &mut App) {
        // Add events

        // Insert resources
        insert_common_resources(app);

        // Add systems
        app.add_systems(Startup, setup::<SimpleMap>);
        insert_common_systems(app);
    }
}

fn insert_common_systems(app: &mut App) {
    app.add_systems(PreUpdate, handle_new_bullets);
    app.add_systems(
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
    app.add_systems(PostUpdate, despawn_dead_bullets);
}

fn insert_common_resources(app: &mut App) {
    app.insert_resource(ClearColor(Color::BLACK));
}
