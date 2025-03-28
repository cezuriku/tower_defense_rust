use bevy::math::{IVec2, Vec2, Vec3};

pub fn grid_to_world(ivec2: IVec2) -> Vec2 {
    Vec2::new(ivec2.x as f32 * 10.0, ivec2.y as f32 * 10.0)
}

pub fn world_to_grid(vec3: Vec3) -> IVec2 {
    IVec2::new((vec3.x / 10.0) as i32, (vec3.y / 10.0) as i32)
}

pub mod creep_tuple;
pub mod top_n;
