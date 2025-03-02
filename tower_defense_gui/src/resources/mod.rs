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
}

#[derive(Event)]
pub struct NewTarget {
    pub pos: IVec2,
}
