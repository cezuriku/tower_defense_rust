use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod map;
pub use map::*;
use resources::GameData;
use systems::*;
pub mod resources;
mod systems;

pub struct TowerDefensePlugin;

impl Plugin for TowerDefensePlugin {
    fn build(&self, app: &mut App) {
        // Add events
        app.add_event::<events::PlaceTurretEvent>()
            .add_event::<events::NewTurretEvent>()
            .add_event::<events::MapChangedEvent>();

        // Insert resources
        app.insert_resource(Map::default())
            .insert_resource(GameData::default());

        // Add systems
        app.add_systems(Startup, setup)
            .add_systems(Update, (move_creeps, handle_turret_placement));
    }
}
