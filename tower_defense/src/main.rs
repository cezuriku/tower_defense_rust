use bevy::{app::*, *};
use tower_defense_gui::TowerDefenseGui;
use tower_defense_plugin::{FreeMap, Map, TowerDefensePlugin};

fn main() {
    App::new()
        .insert_resource(Map::<FreeMap>::new(FreeMap::default()))
        .add_plugins(DefaultPlugins)
        .add_plugins(TowerDefensePlugin::<FreeMap> {
            _marker: std::marker::PhantomData,
        })
        .add_plugins(TowerDefenseGui)
        .run();
}
