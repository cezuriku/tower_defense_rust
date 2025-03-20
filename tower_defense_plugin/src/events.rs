use bevy::prelude::*;

use crate::components::*;

#[derive(Event)]
pub struct PlaceTurretEvent {
    pub turret_type: TurretType,
    pub position: IVec2,
}

#[derive(Event)]
pub struct NewTurretEvent {
    pub turret_type: TurretType,
    pub position: IVec2,
}

#[derive(Event)]
pub struct MapChangedEvent;

#[derive(Event)]
pub struct BasicFireEvent {
    pub origin: IVec2,
    pub target: Vec2,
}
