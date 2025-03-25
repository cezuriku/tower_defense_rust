use bevy::prelude::*;

#[derive(Component)]
pub struct Creep {
    pub health: f32,
    pub max_health: f32,
}

#[derive(Bundle)]
pub struct CreepBundle {
    pub moving_entity: MovingEntity,
    pub transform: Transform,
    pub creep: Creep,
}

#[derive(Component)]
pub struct MovingEntity {
    pub waypoints: Vec<Vec2>,
    pub speed: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TurretType {
    Basic,
    Bomb,
    Follower,
    Slow,
}

#[derive(Component)]
pub struct Turret {
    pub turret_type: TurretType,
    pub position: IVec2,
    pub transform: Transform,
    pub range: f32,
    pub damage: f32,
    pub reload_time: f32,
    pub last_fired: f32,
}

#[derive(Component)]
pub struct BasicTurret {}

#[derive(Component)]
pub struct BombTurret {}

#[derive(Component)]
pub struct SlowTurret {}

#[derive(Component)]
pub struct BulletThrower {
    pub speed: f32,
}

#[derive(Component)]
pub struct FollowerBullet {
    pub direction: Vec2,
    pub target: Entity,
    pub damage: f32,
    pub speed: f32,
    pub angular_velocity: f32,
}

#[derive(Component)]
pub struct SlowDown {
    pub time_to_live: f32,
    pub strength: f32,
}
