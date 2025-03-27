use bevy::{ecs::entity::Entity, math::Vec2};

pub struct CreepTuple {
    pub creep: (Entity, Vec2, Vec2),
    pub value: f32,
}

impl Ord for CreepTuple {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.total_cmp(&other.value)
    }
}

impl PartialOrd for CreepTuple {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for CreepTuple {}

impl PartialEq for CreepTuple {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Clone for CreepTuple {
    fn clone(&self) -> Self {
        CreepTuple {
            creep: self.creep,
            value: self.value,
        }
    }
}
