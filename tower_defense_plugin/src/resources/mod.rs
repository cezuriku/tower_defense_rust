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
    pub start: IVec2,
    pub end: IVec2,
    pub path: Vec<IVec2>,
}

impl Map {
    /// Create a new grid with default values
    pub fn new() -> Self {
        Self {
            cells: [[0; GRID_HEIGHT]; GRID_WIDTH],
            start: ivec2(0, 0),
            end: ivec2(9, 9),
            path: vec![],
        }
    }

    pub fn place_tower(&mut self, pos: &IVec2) -> bool {
        if !self.is_valid(pos) {
            return false;
        }
        self.cells[pos.x as usize][pos.y as usize] = u8::MAX;
        true
    }

    pub fn remove_tower(&mut self, pos: &IVec2) {
        self.cells[pos.x as usize][pos.y as usize] = 0;
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

    pub fn recompute_path(&mut self) {
        if let Some((path, _)) = astar(
            &self.start,
            |p| self.successors(p),
            |p| Self::distance(p, &self.end),
            |p| *p == self.end,
        ) {
            self.path = path
        } else {
            self.path = vec![]
        }
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
        let mut map = Map {
            start: ivec2(0, 0),
            end: ivec2(0, 2),
            ..Map::default()
        };
        map.place_tower(&IVec2 { x: 0, y: 1 }); // This is the tower (x in the example)
        map.recompute_path();
        assert_eq!(
            map.path,
            vec!(
                IVec2 { x: 0, y: 0 }, // start
                IVec2 { x: 1, y: 0 },
                IVec2 { x: 1, y: 1 },
                IVec2 { x: 1, y: 2 },
                IVec2 { x: 0, y: 2 }, // end
            ),
        );
    }

    #[test]
    fn impossible_pathfinding() {
        let mut map = Map {
            start: ivec2(0, 0),
            end: ivec2(2, 2),
            ..Map::new()
        };
        map.place_tower(&IVec2 { x: 0, y: 1 }); // This is the tower (x in the example)
        map.place_tower(&IVec2 { x: 1, y: 0 }); // This is the tower (x in the example)
        map.recompute_path();
        assert_eq!(map.path, vec![]);
    }
}
