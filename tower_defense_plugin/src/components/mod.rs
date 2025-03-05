use bevy::prelude::*;

#[derive(Bundle)]
pub struct Creep {
    pub path: MovementPath,
    pub velocity: Velocity,
}

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Velocity {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Health {
    pub current_health: f32,
    pub max_health: f32,
}

#[derive(Component)]
pub struct Damage {
    pub amount: f32,
}

#[derive(Component)]
pub struct AttackRange {
    pub radius: f32,
}

#[derive(Component)]
pub struct AttackCooldown {
    pub cooldown_time: f32,
    pub time_since_last_attack: f32,
}

#[derive(Component)]
pub struct MovementPath {
    pub waypoints: Vec<Vec2>, // Vector of 2D waypoints
}

#[derive(Component)]
pub struct Resources {
    pub currency: f32,
}

#[derive(Component)]
pub struct TowerType {
    pub tower_id: u32, // Can use an enum or ID to differentiate tower types
}

#[derive(Component)]
pub struct ProjectileType {
    pub projectile_id: u32, // Can be used to determine projectile behavior
}

#[derive(Component)]
pub struct WaveInfo {
    pub wave_number: u32,
    pub total_creeps: u32,
}

#[derive(Component)]
pub struct TileOccupancy {
    pub occupied: bool,
}

#[derive(Component)]
pub struct Score {
    pub points: u32,
}

#[derive(Component)]
pub struct UpgradeLevel {
    pub level: u32,
    pub upgrade_cost: f32,
}

#[derive(Bundle)]
pub struct GameStart {
    resources: Resources,
    score: Score,
}
