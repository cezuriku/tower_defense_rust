//use crate::systems::*;
use crate::systems::*;
use bevy::{app::*, color::Color, render::camera::ClearColor};
use resources::{GameData, Map};

pub mod components;
pub mod events;
pub mod resources;
mod systems;

pub struct TowerDefensePlugin;

impl Plugin for TowerDefensePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::PlaceTurretEvent>();
        app.add_event::<events::NewTurretEvent>();
        app.insert_resource(Map::default());
        app.insert_resource(GameData::default());
        app.add_systems(Startup, setup);
        app.add_systems(Update, (move_creeps, handle_turret_placement));
        app.insert_resource(ClearColor(Color::BLACK));
    }
}
