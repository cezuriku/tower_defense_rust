use bevy::{app::*, *};
use tower_defense_plugin::TowerDefensePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TowerDefensePlugin)
        .run();
}
