use bevy::prelude::*;

#[derive(Resource)]
pub struct GameData {
    pub score: i32,
    pub lives: i32,
    pub gold: i32,
}

impl Default for GameData {
    /// Create a new grid with default values
    fn default() -> Self {
        Self {
            score: 0,
            lives: 10,
            gold: 500,
        }
    }
}
