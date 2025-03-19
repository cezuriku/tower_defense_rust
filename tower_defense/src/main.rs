use std::env;

use bevy::{app::*, *};
use tower_defense_gui::{TowerDefenseGui, TowerDefenseGuiSimpleMap};
use tower_defense_plugin::{TowerDefensePlugin, TowerDefensePluginSimpleMap};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if args[1].as_str() == "freemap" {
            println!("Running FreeMap mode");
            App::new()
                .add_plugins(DefaultPlugins)
                .add_plugins(TowerDefensePlugin)
                .add_plugins(TowerDefenseGui)
                .run();
        } else {
            println!("Unrecognized argument {:?}", args[1].as_str());
            println!("Valid arguments are: freemap");
        }
    } else {
        println!("Running FixedPathMap mode");
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(TowerDefensePluginSimpleMap)
            .add_plugins(TowerDefenseGuiSimpleMap)
            .run();
    }

    println!("{:?}", args);
}
