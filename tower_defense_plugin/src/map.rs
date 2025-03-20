use bevy::{math::ivec2, prelude::*};
use pathfinding::prelude::astar;

pub const GRID_WIDTH: usize = 10;
pub const GRID_HEIGHT: usize = 10;

pub trait Map {
    fn place_tower(&mut self, pos: &IVec2) -> bool;
    fn remove_tower(&mut self, pos: &IVec2);
    fn is_turret_possible(&self, pos: &IVec2) -> bool;
    fn get_path(&self) -> &Vec<IVec2>;
    fn get_start(&self) -> IVec2;
    fn get_end(&self) -> IVec2;
}
pub trait DynamicMap {
    fn compute_path(&self, start: &IVec2) -> Option<(Vec<IVec2>, u32)>;
}

#[derive(Resource)]
pub struct SimpleMap {
    pub cells: [[u8; GRID_HEIGHT]; GRID_WIDTH],
    pub start: IVec2,
    pub end: IVec2,
    pub path: Vec<IVec2>,
}

impl Default for SimpleMap {
    fn default() -> Self {
        Self {
            cells: [
                [0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 1, 0, 1, 1, 1, 1, 1, 1, 0],
                [0, 1, 0, 1, 0, 0, 0, 0, 1, 0],
                [0, 1, 0, 1, 1, 1, 1, 0, 1, 0],
                [0, 1, 0, 0, 0, 0, 1, 0, 1, 0],
                [0, 1, 0, 0, 0, 0, 1, 0, 1, 0],
                [0, 1, 0, 1, 1, 1, 1, 0, 1, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 1, 0],
                [0, 1, 1, 1, 1, 1, 1, 1, 1, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            start: ivec2(0, 1),
            end: ivec2(6, 3),
            path: vec![
                IVec2 { x: 0, y: 1 },
                IVec2 { x: 8, y: 1 },
                IVec2 { x: 8, y: 8 },
                IVec2 { x: 1, y: 8 },
                IVec2 { x: 1, y: 3 },
                IVec2 { x: 3, y: 3 },
                IVec2 { x: 3, y: 6 },
                IVec2 { x: 6, y: 6 },
                IVec2 { x: 6, y: 3 },
            ],
        }
    }
}

impl Map for SimpleMap {
    fn place_tower(&mut self, pos: &IVec2) -> bool {
        if !self.is_empty(pos) {
            return false;
        }
        self.cells[pos.x as usize][pos.y as usize] = u8::MAX;
        true
    }

    fn remove_tower(&mut self, pos: &IVec2) {
        self.cells[pos.x as usize][pos.y as usize] = 0;
    }

    fn is_turret_possible(&self, pos: &IVec2) -> bool {
        self.is_empty(pos)
    }

    fn get_path(&self) -> &Vec<IVec2> {
        self.path.as_ref()
    }

    fn get_start(&self) -> IVec2 {
        self.start
    }

    fn get_end(&self) -> IVec2 {
        self.end
    }
}

impl SimpleMap {
    fn is_empty(&self, pos: &IVec2) -> bool {
        pos.x >= 0
            && pos.y >= 0
            && pos.x < GRID_WIDTH as i32
            && pos.y < GRID_HEIGHT as i32
            && self.cells[pos.x as usize][pos.y as usize] == 0
    }
}

#[derive(Resource)]
pub struct FreeMap {
    pub cells: [[u8; GRID_HEIGHT]; GRID_WIDTH],
    pub start: IVec2,
    pub end: IVec2,
    pub path: Vec<IVec2>,
}

impl Default for FreeMap {
    fn default() -> Self {
        Self {
            cells: [[0; GRID_HEIGHT]; GRID_WIDTH],
            start: ivec2(0, 0),
            end: ivec2(9, 9),
            path: vec![
                IVec2 { x: 0, y: 0 },
                IVec2 { x: 1, y: 1 },
                IVec2 { x: 2, y: 2 },
                IVec2 { x: 3, y: 3 },
                IVec2 { x: 4, y: 4 },
                IVec2 { x: 5, y: 5 },
                IVec2 { x: 6, y: 6 },
                IVec2 { x: 7, y: 7 },
                IVec2 { x: 8, y: 8 },
                IVec2 { x: 9, y: 9 },
            ],
        }
    }
}

impl Map for FreeMap {
    fn place_tower(&mut self, pos: &IVec2) -> bool {
        if !self.is_empty(pos) {
            return false;
        }
        self.cells[pos.x as usize][pos.y as usize] = u8::MAX;
        self.recompute_path();
        true
    }

    fn remove_tower(&mut self, pos: &IVec2) {
        self.cells[pos.x as usize][pos.y as usize] = 0;
    }

    fn is_turret_possible(&self, pos: &IVec2) -> bool {
        if pos.x >= 0
            && pos.y >= 0
            && pos.x < GRID_WIDTH as i32
            && pos.y < GRID_HEIGHT as i32
            && self.cells[pos.x as usize][pos.y as usize] != u8::MAX
            && *pos != self.end
            && *pos != self.start
            && astar(
                &self.start,
                |p| self.successors_except(p, pos),
                |p| Self::distance(p, &self.end),
                |p| *p == self.end,
            )
            .is_some()
        {
            return true;
        }
        false
    }

    fn get_path(&self) -> &Vec<IVec2> {
        self.path.as_ref()
    }

    fn get_start(&self) -> IVec2 {
        self.start
    }

    fn get_end(&self) -> IVec2 {
        self.end
    }
}

impl DynamicMap for FreeMap {
    fn compute_path(&self, start: &IVec2) -> Option<(Vec<IVec2>, u32)> {
        astar(
            start,
            |p| self.successors(p),
            |p| Self::distance(p, &self.end),
            |p| *p == self.end,
        )
    }
}

impl FreeMap {
    fn distance(start: &IVec2, end: &IVec2) -> u32 {
        start.x.abs_diff(end.x).pow(2) + start.y.abs_diff(end.y).pow(2)
    }

    fn is_empty(&self, pos: &IVec2) -> bool {
        pos.x >= 0
            && pos.y >= 0
            && pos.x < GRID_WIDTH as i32
            && pos.y < GRID_HEIGHT as i32
            && self.cells[pos.x as usize][pos.y as usize] != u8::MAX
    }

    fn successors_except(&self, pos: &IVec2, except: &IVec2) -> Vec<(IVec2, u32)> {
        if except == pos {
            return vec![];
        }
        self.successors(pos)
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
            self.is_empty(&IVec2 {
                x: p.x + pos.x,
                y: pos.y,
            }) && self.is_empty(&IVec2 {
                x: pos.x,
                y: p.y + pos.y,
            }) && self.is_empty(&(p + pos))
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
        .filter(|p| self.is_empty(p))
        .map(|p| (p, 50));

        straight.chain(diag).collect()
    }

    pub fn recompute_path(&mut self) {
        if let Some((path, _)) = self.compute_path(&self.start) {
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
        let mut map = FreeMap::default();
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
        let mut map = FreeMap {
            start: ivec2(0, 0),
            end: ivec2(0, 2),
            ..FreeMap::default()
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
        let mut map = FreeMap {
            start: ivec2(0, 0),
            end: ivec2(2, 2),
            ..FreeMap::default()
        };
        map.place_tower(&IVec2 { x: 0, y: 1 }); // This is the tower (x in the example)
        map.place_tower(&IVec2 { x: 1, y: 0 }); // This is the tower (x in the example)
        map.recompute_path();
        assert_eq!(map.path, vec![]);
    }
}
