use bevy::{ecs::system::Resource, math::*, time::Timer};
use pathfinding::prelude::astar;

/// Grid size constants
pub const GRID_WIDTH: usize = 10;
pub const GRID_HEIGHT: usize = 10;

#[derive(Resource)]
pub struct GreetTimer(pub Timer);

#[derive(Resource)]
pub struct Map {
    pub cells: [[u8; GRID_HEIGHT]; GRID_WIDTH],
}

impl Map {
    /// Create a new grid with default values
    pub fn new() -> Self {
        Self {
            cells: [[0; GRID_HEIGHT]; GRID_WIDTH],
        }
    }

    pub fn place_tower(&mut self, pos: &IVec2) -> bool {
        if !self.is_valid(pos) {
            return false;
        }
        self.cells[pos.x as usize][pos.y as usize] = u8::MAX;
        true
    }
    fn distance(start: &IVec2, end: &IVec2) -> u32 {
        start.x.abs_diff(end.x).pow(2) + start.y.abs_diff(end.y).pow(2)
    }

    fn is_valid(&self, pos: &IVec2) -> bool {
        pos.x >= 0
            && pos.y >= 0
            && pos.x < GRID_WIDTH as i32
            && pos.y < GRID_HEIGHT as i32
            && self.cells[pos.x as usize][pos.y as usize] != u8::MAX
    }

    fn successors(&self, pos: &IVec2) -> Vec<(IVec2, u32)> {
        let diag = vec![
            IVec2 { x: 1, y: 1 },
            IVec2 { x: -1, y: 1 },
            IVec2 { x: 1, y: -1 },
            IVec2 { x: -1, y: -1 },
        ]
        .into_iter()
        .filter(|p| {
            self.is_valid(&IVec2 {
                x: p.x + pos.x,
                y: pos.y,
            }) && self.is_valid(&IVec2 {
                x: pos.x,
                y: p.y + pos.y,
            }) && self.is_valid(&(p + pos))
        })
        .map(|p| (p + pos, 75));

        let straight = vec![
            IVec2 {
                x: pos.x + 1,
                y: pos.y,
            },
            IVec2 {
                x: pos.x - 1,
                y: pos.y,
            },
            IVec2 {
                x: pos.x,
                y: pos.y + 1,
            },
            IVec2 {
                x: pos.x,
                y: pos.y - 1,
            },
        ]
        .into_iter()
        .filter(|p| self.is_valid(p))
        .map(|p| (p, 50));

        straight.chain(diag).collect()
    }

    pub fn find_path(&self, start: &IVec2, end: &IVec2) -> Option<(Vec<IVec2>, u32)> {
        astar(
            start,
            |p| self.successors(p),
            |p| Self::distance(p, end),
            |p| *p == *end,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn place_tower_twice() {
        let mut map = Map::new();
        assert!(map.place_tower(&IVec2 { x: 0, y: 1 }));
        assert!(!map.place_tower(&IVec2 { x: 0, y: 1 }));
    }

    /*
     This test find the shortest path in this maze
     sxe....
     ooo....
     .......
    */
    #[test]
    fn easy_pathfinding() {
        let mut map = Map::new();
        map.place_tower(&IVec2 { x: 0, y: 1 }); // This is the tower (x in the example)
        let result = map.find_path(
            &IVec2 { x: 0, y: 0 }, // This is the start (s in the example)
            &IVec2 { x: 0, y: 2 }, // This is the end (e in the example)
        );
        assert_eq!(
            result,
            Some((
                vec!(
                    IVec2 { x: 0, y: 0 }, // start
                    IVec2 { x: 1, y: 0 },
                    IVec2 { x: 1, y: 1 },
                    IVec2 { x: 1, y: 2 },
                    IVec2 { x: 0, y: 2 }, // end
                ),
                4
            )),
        );
        assert!(result.is_some())
    }

    #[test]
    fn impossible_pathfinding() {
        let mut map = Map::new();
        map.place_tower(&IVec2 { x: 0, y: 1 }); // This is the tower (x in the example)
        map.place_tower(&IVec2 { x: 1, y: 0 }); // This is the tower (x in the example)
        let result = map.find_path(
            &IVec2 { x: 0, y: 0 }, // This is the start (s in the example)
            &IVec2 { x: 2, y: 2 }, // This is the end (e in the example)
        );
        assert_eq!(result, None);
    }
}
