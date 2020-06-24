use magneto::Context;
use magneto::graphics;

use sdl2::keyboard::Keycode;

use crate::GRID_SIZE;
use crate::TILE_SIZE;
use crate::GRID_BORDER;
use crate::GRID_PIXEL_SIZE;
use crate::SCREEN_SIZE;
use super::grid::Direction;
use super::grid::GridPosition;

use std::collections::LinkedList;

pub enum Ate {
    Wall,
    Snake,
    Food(GridPosition),
}

pub struct Snake {
    pub segments: LinkedList<GridPosition>,
    pub dir: Direction,
    pub last_update_dir: Direction,
    pub next_dir: Option<Direction>,
}

impl Snake {
    pub fn new() -> Snake {
        let mut segs = LinkedList::new();
        segs.push_front(GridPosition::new(4, 5));
        segs.push_front(GridPosition::new(5, 5));
        segs.push_front(GridPosition::new(6, 5));

        Snake {
            segments: segs,
            dir: Direction::East,
            last_update_dir: Direction::East,
            next_dir: None,
        }
    }


    pub fn render(&self, ctx: &mut Context) {
        graphics::draw_rect(
            ctx,
            1.0 + GRID_BORDER + self.segments.front().unwrap().x as f32 * TILE_SIZE,
            1.0 + GRID_BORDER + self.segments.front().unwrap().y as f32 * TILE_SIZE,
            TILE_SIZE - 1.0,
            TILE_SIZE - 1.0,
            (29.0 / 255.0, 92.0 / 255.0, 42.0 / 255.0)
        );
        for seg in self.segments.iter().skip(1) {
            graphics::draw_rect(
                ctx,
                1.0 + GRID_BORDER + seg.x as f32 * TILE_SIZE,
                1.0 + GRID_BORDER + seg.y as f32 * TILE_SIZE,
                TILE_SIZE - 1.0,
                TILE_SIZE - 1.0,
                (50.0 / 255.0, 120.0 / 255.0, 58.0 / 255.0)
            );
        }
    }

    // Called on every update of the snake
    pub fn tick(&mut self, food: GridPosition) -> Option<Ate> {
        if self.last_update_dir == self.dir && self.next_dir.is_some() {
            self.dir = self.next_dir.unwrap();
            self.next_dir = None;
        }

        // Update the snakes segments
        let new_head = GridPosition::new_from_move(*self.segments.front().unwrap(), self.dir);
        self.segments.push_front(new_head);
        

        let mut ate = None;

        // check for collisions
        if *self.segments.front().unwrap() == food {
            ate = Some(Ate::Food(GridPosition::new_from_random_no_overlap(&self.segments)));
        } else if self.check_self_collision() {
            ate = Some(Ate::Snake);
        } else if self.check_wall_collision() {
            ate = Some(Ate::Wall);
        } else {
            self.segments.pop_back();
        }

        self.last_update_dir = self.dir;
        
        ate
    }

    pub fn check_self_collision(&self) -> bool {
        let head = self.segments.front().unwrap();
        for seg in self.segments.iter().skip(1) {
            if head == seg {
                return true;
            }
        }
        return false;
    }

    pub fn check_wall_collision(&self) -> bool {
        let head = self.segments.front().unwrap();
        if head.x == GRID_SIZE.0 as i16 {
            return true;
        } else if head.x < 0 {
            return true;
        } else if head.y == GRID_SIZE.1 as i16 {
            return true;
        } else if head.y < 0 {
            return true;
        }
        return false;
    }
}