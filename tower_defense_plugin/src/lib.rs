//use crate::systems::*;
use crate::systems::*;
use bevy::{
    app::*,
    color::Color,
    render::camera::ClearColor,
    time::{Timer, TimerMode},
};
use resources::{GreetTimer, Map};

pub mod components;
// pub mod events;
pub mod resources;
mod systems;

pub struct TowerDefensePlugin;

impl Plugin for TowerDefensePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.insert_resource(Map::new());
        app.add_systems(Startup, setup);
        app.add_systems(Update, move_creeps);
        app.insert_resource(ClearColor(Color::BLACK));
    }
}
