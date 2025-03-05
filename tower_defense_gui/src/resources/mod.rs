use bevy::prelude::*;

#[derive(Resource)]
pub struct TowerAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}

#[derive(Resource)]
pub struct PathAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
    pub start_mesh: Handle<Mesh>,
    pub start_material: Handle<ColorMaterial>,
    pub end_mesh: Handle<Mesh>,
    pub end_material: Handle<ColorMaterial>,
}

#[derive(Event)]
pub struct UpdatePath {}
