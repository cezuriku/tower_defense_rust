use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod map;
pub use map::*;
use resources::{CreepRng, GameData};
use systems::*;
pub mod resources;
mod systems;
mod utils;
pub use utils::*;

pub struct TowerDefensePlugin;

impl Plugin for TowerDefensePlugin {
    fn build(&self, app: &mut App) {
        // Add events
        insert_common_events(app);

        // Insert resources
        insert_common_resources(app);
        app.insert_resource(FreeMap::default());

        // Add systems
        insert_common_systems(app);
        app.add_systems(
            Update,
            (
                spawn_creeps::<FreeMap>,
                handle_turret_placement::<FreeMap>,
                update_creep_paths::<FreeMap>,
            ),
        );
    }
}
pub struct TowerDefensePluginSimpleMap;

impl Plugin for TowerDefensePluginSimpleMap {
    fn build(&self, app: &mut App) {
        // Add events
        insert_common_events(app);

        // Insert resources
        insert_common_resources(app);
        app.insert_resource(SimpleMap::default());

        // Add systems
        insert_common_systems(app);
        app.add_systems(
            Update,
            (
                spawn_creeps::<SimpleMap>,
                handle_turret_placement::<SimpleMap>,
            ),
        );
    }
}

fn insert_common_systems(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        (
            move_creeps,
            shoot_creeps,
            move_follower_bullets,
            bullet_thrower_system,
        ),
    );
    app.add_systems(PostUpdate, despawn_dead_creeps);
}

fn insert_common_resources(app: &mut App) {
    app.insert_resource(GameData::default())
        .insert_resource(CreepRng::default());
}

fn insert_common_events(app: &mut App) {
    app.add_event::<events::PlaceTurretEvent>()
        .add_event::<events::NewTurretEvent>()
        .add_event::<events::BasicFireEvent>()
        .add_event::<events::MapChangedEvent>();
}
