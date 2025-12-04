use bevy::prelude::*;

use crate::components::*;

#[derive(Message)]
pub struct PlaceTurretMessage {
    pub turret_type: TurretType,
    pub position: IVec2,
}

#[derive(Message)]
pub struct NewTurretMessage {
    pub turret_type: TurretType,
    pub position: IVec2,
}

#[derive(Message)]
pub struct MapChangedMessage;

#[derive(Message)]
pub struct BasicFireMessage {
    pub origin: IVec2,
    pub target: Vec2,
}
