use bevy::{
    color::Color,
    core_pipeline::core_2d::Camera2d,
    ecs::{query::With, system::*},
    math::*,
    sprite::Sprite,
    time::Time,
    transform::components::Transform,
    utils::default,
};

use crate::components::*;
use crate::resources::*;

const PADDLE_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);

pub fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
    commands.spawn((
        Sprite::from_color(PADDLE_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(10.0, 50.0, 10.0),
            scale: Vec3::new(100.0, 100.0, 100.0),
            ..default()
        },
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.2, 0.2), Vec2::ONE),
        Transform {
            translation: Vec3::new(5.0, 30.0, 8.0),
            scale: Vec3::new(100.0, 100.0, 120.0),
            ..default()
        },
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.2, 0.2), Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, 0.0, 15.0),
            scale: Vec3::new(10.0, 10.0, 10.0),
            ..default()
        },
    ));
    commands.spawn(Camera2d);
}

pub fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>,
) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}
