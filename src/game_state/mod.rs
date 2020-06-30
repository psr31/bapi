pub mod grid;
pub mod snake;

use grid::{GridPosition, Direction};
use snake::Snake;
use snake::Ate;

use crate::state::State;
use crate::state::StateInstructions;
use crate::GRID_SIZE;
use crate::TILE_SIZE;
use crate::GRID_BORDER;
use crate::GRID_BORDER_TOP;
use crate::GRID_PIXEL_SIZE;
use crate::SCREEN_SIZE;

use magneto::graphics;
use graphics::Text;
use graphics::Font;
use graphics::Color;
use magneto::Context;
use sdl2::keyboard::Keycode;

use std::time::{Duration, Instant};
use std::collections::HashSet;

pub struct GameState {
    snake: Snake,
    food: GridPosition,
    start_time: Instant,
    last_snake_tick: Instant,
    fps: f32,
    frame_count: usize,
    speed: u32,
    score: u32,
    stat_font: Font,
    score_text: Text,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameState {
        let snake = Snake::new();

        let font = Font::new(ctx, "./resources/snes.ttf", 85).unwrap();
        let mut score_text = Text::new("0", &font, (1.0, 1.0, 1.0).into());
        score_text.x = GRID_BORDER + 5.0;
        score_text.y = 6.0;

        GameState {
            snake: snake,
            last_snake_tick: Instant::now(),
            start_time: Instant::now(),
            fps: 0.0,
            frame_count: 0,
            food: GridPosition::new_from_random(),
            speed: 75,
            score: 0,
            stat_font: font,
            score_text: score_text,
        }
    } 

    fn draw_playing_grid(&mut self, ctx: &mut Context) {
        // Draw grid background
        magneto::graphics::draw_rect(
            ctx, 
            GRID_BORDER,
            GRID_BORDER_TOP, 
            GRID_PIXEL_SIZE.0, 
            GRID_PIXEL_SIZE.1, 
            (106, 104, 186).into(),
        );
        
        // Draw grid lines
        for i in 1..GRID_SIZE.0 as u32 {
            magneto::graphics::draw_rect(
                ctx, 
                GRID_BORDER + TILE_SIZE * i as f32, 
                GRID_BORDER_TOP, 
                1.0,
                GRID_PIXEL_SIZE.1, 
                (0.0, 0.0, 0.0).into(),
            );
        }
        
        for i in 1..GRID_SIZE.1 as u32 {
            magneto::graphics::draw_rect(
                ctx, 
                GRID_BORDER, 
                GRID_BORDER_TOP + TILE_SIZE * i as f32, 
                GRID_PIXEL_SIZE.0,
                1.0, 
                (0.0, 0.0, 0.0).into(),
            );
        }
    }

    fn draw_stats(&mut self, ctx: &mut Context) {
        let score_text = format!("score: {}", self.score);

        self.score_text.set_text(&score_text, &self.stat_font);
        self.score_text.draw(ctx, &self.stat_font);
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context, keys: &HashSet<Keycode>) -> StateInstructions {
        self.frame_count += 1;

        if self.speed >= 60 {
            self.speed = (self.start_time.elapsed().as_secs() as f32 * (-130.0/120.0) + 200.0) as u32;
        }
        
        if Instant::now() - self.last_snake_tick >= Duration::from_millis(self.speed as u64) {
            match self.snake.tick(self.food) {
                Some(Ate::Food(a)) => {
                    self.score += 1;
                    self.food = a;
                },
                Some(Ate::Snake) => {
                    return StateInstructions::GameOver;
                },
                Some(Ate::Wall) => {
                    return StateInstructions::GameOver;
                },
                _ => {},
            }
            self.last_snake_tick = Instant::now();
        }

        if keys.contains(&Keycode::Up) {
            let dir = Direction::North;
            if self.snake.dir != self.snake.last_update_dir && dir.inverse() != self.snake.dir {
                self.snake.next_dir = Some(dir);
            } else if dir.inverse() != self.snake.last_update_dir {
                self.snake.dir = dir;
            }
        }
        if keys.contains(&Keycode::Down) {
            let dir = Direction::South;
            if self.snake.dir != self.snake.last_update_dir && dir.inverse() != self.snake.dir {
                self.snake.next_dir = Some(dir);
            } else if dir.inverse() != self.snake.last_update_dir {
                self.snake.dir = dir;
            }
        }
        if keys.contains(&Keycode::Right) {
            let dir = Direction::East;
            if self.snake.dir != self.snake.last_update_dir && dir.inverse() != self.snake.dir {
                self.snake.next_dir = Some(dir);
            } else if dir.inverse() != self.snake.last_update_dir {
                self.snake.dir = dir;
            }
        }
        if keys.contains(&Keycode::Left) {
            let dir = Direction::West;
            if self.snake.dir != self.snake.last_update_dir && dir.inverse() != self.snake.dir {
                self.snake.next_dir = Some(dir);
            } else if dir.inverse() != self.snake.last_update_dir {
                self.snake.dir = dir;
            }
        }

        StateInstructions::Continue
    }

    fn render(&mut self, ctx: &mut Context) {
        self.draw_playing_grid(ctx);
        self.draw_stats(ctx);

        self.snake.render(ctx);

        graphics::draw_rect(
            ctx,
            1.0 + GRID_BORDER + self.food.x as f32 * TILE_SIZE, 1.0 + GRID_BORDER_TOP + self.food.y as f32 * TILE_SIZE, TILE_SIZE - 1.0, TILE_SIZE - 1.0,
            (1.0, 0.0, 0.0).into(),
        );
    }
}