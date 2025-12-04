use std::f32;

use bevy::prelude::*;
use bevy::{time::Time, transform::components::Transform};

use crate::creep_tuple::CreepTuple;
use crate::resources::*;
use crate::top_n::TopN;
use crate::utils::world_to_grid;
use crate::{DynamicMap, events::*};
use crate::{Map, components::*};
use rand::RngCore;

pub fn setup() {}

pub fn move_creeps(
    mut creeps: Query<(&mut MovingEntity, &mut Transform, Option<&SlowDown>)>,
    time: Res<Time>,
) {
    for (mut creep, mut transform, slowdown) in &mut creeps {
        let mut delta = creep.speed * time.delta_secs();
        if let Some(slowdown) = slowdown {
            delta /= slowdown.strength
        }
        while delta > 0.0 && !creep.waypoints.is_empty() {
            if let Some(waypoint) = creep.waypoints.last() {
                let distance = transform.translation.distance(waypoint.extend(0.0));
                if delta < distance {
                    transform.translation = transform
                        .translation
                        .move_towards(waypoint.extend(0.0), delta);
                    delta = 0.0;
                } else {
                    transform.translation = waypoint.extend(0.0);
                    creep.waypoints.pop();
                    delta -= distance;
                }
            }
        }
    }
}

pub fn update_creep_paths<T: Resource + DynamicMap>(
    mut events: MessageReader<MapChangedMessage>,
    mut creeps: Query<(&Transform, &mut MovingEntity), With<Creep>>,
    map: Res<T>,
) {
    for _event in events.read() {
        for (transform, mut moving_entity) in creeps.iter_mut() {
            let start = world_to_grid(transform.translation);
            if let Some((new_path, _)) = map.compute_path(&start) {
                let mut waypoints: Vec<Vec2> = new_path
                    .iter()
                    .map(|pos| Vec2::new(pos.x as f32 * 10.0, pos.y as f32 * 10.0))
                    .rev()
                    .collect();
                waypoints.pop();

                moving_entity.waypoints = waypoints;
            } else {
                // Do not update the path and let the creep continue on its current path even though it is likely to go through walls
            }
        }
    }
}

pub fn handle_turret_placement<T>(
    mut commands: Commands,
    mut events: MessageReader<PlaceTurretMessage>,
    mut game_data: ResMut<GameData>,
    mut map: ResMut<T>,
    mut new_turret_writer: MessageWriter<NewTurretMessage>,
    mut map_changed_writer: MessageWriter<MapChangedMessage>,
) where
    T: Resource + Map,
{
    for event in events.read() {
        // Check the cost of the turret to ensure we can buy one
        let (cost, range, reload_time) = match event.turret_type {
            TurretType::Basic => (50, 25.0, 1.0),
            TurretType::Bomb => (100, 20.0, 1.0),
            TurretType::Follower => (75, 50.0, 1.0),
            TurretType::Slow => (10, 50.0, 3.0),
        };

        if game_data.gold >= cost && map.is_turret_possible(&event.position) {
            // Deduct the cost of the turret from the player's gold
            game_data.gold -= cost;

            create_turret(&mut commands, &mut map, event, range, reload_time);

            // Notify other systems that a new turret has been placed (e.g., for UI updates)
            new_turret_writer.write(NewTurretMessage {
                turret_type: event.turret_type,
                position: event.position,
            });

            map_changed_writer.write(MapChangedMessage {});

            println!("Turret placed successfully!");
        } else {
            println!("Can not place turret at position: {:?}!", event.position);
        }
    }
}

fn create_turret<T>(
    commands: &mut Commands,
    map: &mut ResMut<T>,
    event: &PlaceTurretMessage,
    range: f32,
    reload_time: f32,
) where
    T: Resource + Map,
{
    // Place the base turret
    map.place_tower(&event.position);

    let turret_id = commands
        .spawn(Turret {
            turret_type: event.turret_type,
            position: event.position,
            transform: Transform::from_xyz(
                event.position.x as f32 * 10.0,
                event.position.y as f32 * 10.0,
                0.0,
            ),
            range,
            damage: 10.0,
            reload_time,
            last_fired: 0.0,
        })
        .id();

    // Place the specific turret type (which will handle the actual shooting)
    match event.turret_type {
        TurretType::Basic => {
            commands.entity(turret_id).insert(BasicTurret {});
        }
        TurretType::Bomb => {
            commands.entity(turret_id).insert(BombTurret {});
        }
        TurretType::Follower => {
            commands
                .entity(turret_id)
                .insert(BulletThrower { speed: 30.0 });
        }
        TurretType::Slow => {
            commands.entity(turret_id).insert(SlowTurret {});
        }
    }
}

pub fn spawn_creeps<T>(
    mut commands: Commands,
    time: Res<Time>,
    mut last_spawn_time: Local<f32>,
    map: Res<T>,
    mut rng: ResMut<CreepRng>,
) where
    T: Resource + Map,
{
    *last_spawn_time += time.delta_secs();

    if *last_spawn_time > 2.0 {
        *last_spawn_time = 0.0;

        let start_pos = map.get_start();
        let waypoints: Vec<Vec2> = map
            .get_path()
            .iter()
            .map(|pos| Vec2::new(pos.x as f32 * 10.0, pos.y as f32 * 10.0))
            .rev()
            .collect();

        let (speed, health) = if rng.rng.next_u32() < u32::MAX / 4 {
            (35.0, 50.0)
        } else {
            (20.0, 100.0)
        };

        commands.spawn((
            MovingEntity { speed, waypoints },
            Transform::from_translation(Vec3::new(
                start_pos.x as f32 * 10.0,
                start_pos.y as f32 * 10.0,
                0.0,
            )),
            Creep {
                health,
                max_health: 100.0,
            },
        ));
    }
}

macro_rules! shoot_n_creeps {
    ($turret: ident, $creeps: ident, $time: ident, $strategy: ident, $n_creeps: literal, $inner_function: expr) => {
        if time_to_fire(&mut $turret, &$time) {
            let turret_position = $turret.transform.translation.truncate();
            let mut ro_creeps =
                $creeps.transmute_lens::<(Entity, &Creep, &Transform, &MovingEntity)>();
            let n_creeps = find_top_creeps_within_range(
                turret_position,
                $turret.range,
                &ro_creeps.query(),
                $strategy,
                $n_creeps,
            );

            if !n_creeps.is_empty() {
                for (creep_entity, creep_position, turret_position) in n_creeps {
                    $inner_function(creep_entity, creep_position, turret_position);
                }

                $turret.last_fired = 0.0;
            }
        }
    };
}

pub fn basic_turret_system(
    time: Res<Time>,
    mut turrets: Query<(&mut Turret, Option<&Strategy>), With<BasicTurret>>,
    mut creeps: Query<(Entity, &mut Creep, &Transform, &MovingEntity)>,
    mut fire_events: MessageWriter<BasicFireMessage>,
) {
    for (mut turret, strategy) in turrets.iter_mut() {
        shoot_n_creeps!(
            turret,
            creeps,
            time,
            strategy,
            1,
            |creep_entity, creep_position, _turret_entity| {
                if let Ok((_, mut creep, _, _)) = creeps.get_mut(creep_entity) {
                    shoot_creep(&mut fire_events, &turret, &mut creep, creep_position);
                }
            }
        );
    }
}

pub fn slow_turret_system(
    time: Res<Time>,
    mut turrets: Query<(&mut Turret, Option<&Strategy>), With<SlowTurret>>,
    mut creeps: Query<(Entity, &Creep, &Transform, &MovingEntity), Without<SlowDown>>,
    mut commands: Commands,
) {
    for (mut turret, strategy) in turrets.iter_mut() {
        shoot_n_creeps!(
            turret,
            creeps,
            time,
            strategy,
            1,
            |creep_entity, _creep_position, _turret_entity| {
                commands.entity(creep_entity).insert_if_new(SlowDown {
                    time_to_live: 5.0,
                    strength: 5.0,
                });
            }
        );
    }
}

pub fn bomb_turret_system(
    time: Res<Time>,
    mut turrets: Query<&mut Turret, With<BombTurret>>,
    mut creeps: Query<(Entity, &mut Creep, &Transform)>,
    mut fire_events: MessageWriter<BasicFireMessage>,
) {
    for mut turret in turrets.iter_mut() {
        if time_to_fire(&mut turret, &time) {
            let turret_position = turret.transform.translation.truncate();
            for (_, mut creep, creep_position) in creeps.iter_mut() {
                let creep_position = creep_position.translation.truncate();
                if turret_position.distance(creep_position) <= turret.range {
                    shoot_creep(&mut fire_events, &turret, &mut creep, creep_position);
                    turret.last_fired = 0.0;
                }
            }
        }
    }
}

fn shoot_creep(
    fire_events: &mut MessageWriter<BasicFireMessage>,
    turret: &Turret,
    creep: &mut Creep,
    creep_position: Vec2,
) {
    creep.health -= turret.damage;

    fire_events.write(BasicFireMessage {
        origin: turret.position,
        target: creep_position,
    });
}

pub fn bullet_thrower_system(
    time: Res<Time>,
    mut turrets: Query<(&mut Turret, &BulletThrower, Option<&Strategy>)>,
    mut creeps: Query<(Entity, &Creep, &Transform, &MovingEntity)>,
    mut commands: Commands,
) {
    for (mut turret, bullet_thrower, strategy) in turrets.iter_mut() {
        shoot_n_creeps!(
            turret,
            creeps,
            time,
            strategy,
            2,
            |creep_entity, creep_position: Vec2, turret_position: Vec2| {
                commands.spawn((
                    FollowerBullet {
                        damage: turret.damage,
                        target: creep_entity,
                        speed: bullet_thrower.speed,
                        direction: (creep_position - turret_position).normalize(),
                        angular_velocity: 2.0,
                    },
                    Transform::from_translation(turret.transform.translation),
                ));
            }
        );
    }
}

fn time_to_fire(turret: &mut Turret, time: &Res<Time>) -> bool {
    turret.last_fired += time.delta_secs();

    if turret.last_fired >= turret.reload_time {
        return true;
    }
    false
}

fn find_top_creeps_within_range(
    turret_position: Vec2,
    turret_range: f32,
    creeps: &Query<'_, '_, (Entity, &Creep, &Transform, &MovingEntity)>,
    strategy: Option<&Strategy>,
    n: usize,
) -> Vec<(Entity, Vec2, Vec2)> {
    let mut best_creeps: TopN<CreepTuple> = TopN::new(n);
    let strategy = match strategy {
        Some(strategy) => strategy,
        None => &Strategy::Closest,
    };

    for (creep_entity, creep, creep_transform, moving_entity) in creeps.iter() {
        let creep_position = creep_transform.translation.truncate();
        let distance = turret_position.distance(creep_position);

        if distance <= turret_range {
            let value = match strategy {
                Strategy::Weakest => -creep.health,
                Strategy::Strongest => creep.health,
                Strategy::Slowest => -moving_entity.speed,
                Strategy::Fastest => moving_entity.speed,
                Strategy::Closest => -distance,
                Strategy::Furthest => distance,
            };

            best_creeps.insert(CreepTuple {
                creep: (creep_entity, creep_position, turret_position),
                value,
            });
        }
    }
    best_creeps
        .get()
        .iter()
        .map(|creep_tuple| creep_tuple.creep)
        .collect()
}

pub fn move_follower_bullets(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut FollowerBullet, &mut Transform), Without<Creep>>,
    mut creeps: Query<(&Transform, &mut Creep)>,
    time: Res<Time>,
) {
    for (entity, mut bullet, mut transform) in bullets.iter_mut() {
        if let Ok((target_transform, mut creep)) = creeps.get_mut(bullet.target) {
            let target_position = target_transform.translation.truncate();
            let bullet_position = transform.translation.truncate();

            let direction_to_target = (target_position - bullet_position).normalize();
            let angle_to_target = direction_to_target.y.atan2(direction_to_target.x);
            let current_angle = bullet.direction.y.atan2(bullet.direction.x);

            let angle_diff =
                (angle_to_target - current_angle).rem_euclid(2.0 * std::f32::consts::PI);
            let rotation_direction = if angle_diff > std::f32::consts::PI {
                -1.0
            } else {
                1.0
            };

            let rotation = rotation_direction * bullet.angular_velocity * time.delta_secs();
            bullet.direction = Vec2::new(
                bullet.direction.x * rotation.cos() - bullet.direction.y * rotation.sin(),
                bullet.direction.x * rotation.sin() + bullet.direction.y * rotation.cos(),
            );

            let new_position =
                bullet_position + bullet.direction * bullet.speed * time.delta_secs();
            transform.translation = new_position.extend(transform.translation.z);

            if transform.translation.distance(target_transform.translation) < 5.0
                && creep.health > 0.0
            {
                creep.health -= bullet.damage;
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn despawn_dead_creeps(mut commands: Commands, mut creeps: Query<(Entity, &Creep)>) {
    for (entity, creep) in creeps.iter_mut() {
        if creep.health <= 0.0 {
            commands.entity(entity).despawn_children();
            commands.entity(entity).despawn();
        }
    }
}

pub fn despawn_slowdown(
    mut commands: Commands,
    mut creeps: Query<(Entity, &mut SlowDown)>,
    time: Res<Time>,
) {
    for (entity, mut slowdown) in creeps.iter_mut() {
        slowdown.time_to_live -= time.delta_secs();
        if slowdown.time_to_live <= 0.0 {
            commands.entity(entity).remove::<SlowDown>();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use bevy::time::TimeUpdateStrategy;

    // tower_defense_plugin/src/systems.rs

    fn iterate_and_get_creep_transform(app: &mut App) -> Vec2 {
        app.update();

        let world = app.world_mut();
        let mut query = world.query::<(&MovingEntity, &Transform)>();
        if let Ok((_creep, transform)) = query.single(world) {
            transform.translation.truncate()
        } else {
            Vec2::ZERO
        }
    }

    #[test]
    fn test_move_creeps() {
        let mut app = App::new();
        let next_waypoint = Vec2::new(15.0, 4000.0);
        let fixed_delta = 0.1;
        let speed = 10.0;

        app.add_plugins(MinimalPlugins)
            .add_systems(Update, move_creeps)
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f32(
                fixed_delta,
            )));

        app.world_mut().spawn((
            MovingEntity {
                speed,
                waypoints: vec![Vec2::new(200.0, 15.0), next_waypoint],
            },
            Transform::from_translation(Vec3::new(15.0, 15.0, 0.0)),
        ));

        let transform = iterate_and_get_creep_transform(&mut app);
        assert_eq!(transform, Vec2::new(15.0, 15.0)); // No movement since delta time is 0

        let transform = iterate_and_get_creep_transform(&mut app);

        // Update the expected position based on the movement logic in `move_creeps`
        let expected_pos = Vec2::new(15.0, 15.0).move_towards(next_waypoint, fixed_delta * speed);
        assert_eq!(transform, expected_pos); // Check if the position has updated correctly
    }
}
