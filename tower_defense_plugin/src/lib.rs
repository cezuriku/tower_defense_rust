use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod map;
pub use map::*;
use resources::GameData;
use systems::*;
pub mod resources;
mod systems;
mod utils;
pub use utils::*;

pub struct TowerDefensePlugin;

impl Plugin for TowerDefensePlugin {
    fn build(&self, app: &mut App) {
        // Add events
        app.add_event::<events::PlaceTurretEvent>()
            .add_event::<events::NewTurretEvent>()
            .add_event::<events::BasicFireEvent>()
            .add_event::<events::MapChangedEvent>();

        // Insert resources
        app.insert_resource(FreeMap::default())
            .insert_resource(GameData::default());

        // Add systems
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                spawn_creeps::<FreeMap>,
                move_creeps,
                handle_turret_placement::<FreeMap>,
                shoot_creeps,
                update_creep_paths::<FreeMap>,
            ),
        );
    }
}
pub struct TowerDefensePluginSimpleMap;

impl Plugin for TowerDefensePluginSimpleMap {
    fn build(&self, app: &mut App) {
        // Add events
        app.add_event::<events::PlaceTurretEvent>()
            .add_event::<events::NewTurretEvent>()
            .add_event::<events::BasicFireEvent>()
            .add_event::<events::MapChangedEvent>();

        // Insert resources
        app.insert_resource(SimpleMap::default())
            .insert_resource(GameData::default());

        // Add systems
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                spawn_creeps::<SimpleMap>,
                move_creeps,
                handle_turret_placement::<SimpleMap>,
                shoot_creeps,
            ),
        );
    }
}
