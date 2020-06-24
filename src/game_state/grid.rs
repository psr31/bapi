use rand::Rng;

use std::collections::LinkedList;

use crate::GRID_SIZE;
use crate::TILE_SIZE;
use crate::GRID_BORDER;
use crate::GRID_PIXEL_SIZE;
use crate::SCREEN_SIZE;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn inverse(&self) -> Self {
        match *self {
            Direction::East => {
                Direction::West
            },
            Direction::West => {
                Direction::East
            },
            Direction::North => {
                Direction::South
            },
            Direction::South => {
                Direction::North
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct GridPosition {
    pub x: i16,
    pub y: i16,
}

impl GridPosition {
    pub fn new(x: i16, y: i16) -> GridPosition {
        GridPosition { x, y }
    }

    pub fn new_from_move(pos: GridPosition, dir: Direction) -> GridPosition {
        match dir {
            Direction::North => { GridPosition::new(pos.x, pos.y - 1) },
            Direction::South => { GridPosition::new(pos.x, pos.y + 1) },
            Direction::East  => { GridPosition::new(pos.x + 1, pos.y) },
            Direction::West  => { GridPosition::new(pos.x - 1, pos.y) },
        }
    }

    pub fn new_from_random() -> GridPosition {
        let mut rng = rand::thread_rng();
        GridPosition::new((rng.gen_range(0, GRID_SIZE.0 as u32)) as i16, (rng.gen_range(0, GRID_SIZE.1 as u32)) as i16)
    }

    pub fn new_from_random_no_overlap(segs: &LinkedList<GridPosition>) -> GridPosition {
        'val: loop {
            let p = Self::new_from_random();
            for s in segs.iter() {
                if p == *s {
                    continue 'val;
                }
            }
            return p;
        }
    }
}