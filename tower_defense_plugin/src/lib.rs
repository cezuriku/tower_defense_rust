//use crate::systems::*;
use crate::systems::*;
use bevy::{
    app::*,
    color::Color,
    render::camera::ClearColor,
    time::{Timer, TimerMode},
};
use resources::GreetTimer;

pub mod components;
// pub mod events;
mod resources;
mod systems;

pub struct TowerDefensePlugin;

impl Plugin for TowerDefensePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, greet_people);
        app.insert_resource(ClearColor(Color::srgb(0.1, 0.3, 0.7)));
    }
}
