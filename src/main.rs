const GRID_SIZE: (f32, f32) = (16.0, 16.0);
const TILE_SIZE: f32 = 34.0;
const GRID_BORDER: f32 = 25.0;
const GRID_PIXEL_SIZE: (f32, f32) = (TILE_SIZE*GRID_SIZE.0, TILE_SIZE*GRID_SIZE.1);
const SCREEN_SIZE: (f32, f32) = (2.0*GRID_BORDER+GRID_PIXEL_SIZE.0, 2.0*GRID_BORDER+GRID_PIXEL_SIZE.1);

use std::collections::HashSet;

use magneto::Context;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod state;
use state::State;
use state::StateInstructions;

// States
mod game_state;
use game_state::GameState;
mod game_over;
use game_over::GameOver;

fn main() {
    let ctx = &mut Context::new("bapi - a snake game by psr31", SCREEN_SIZE.0 as u32, SCREEN_SIZE.1 as u32);

    let mut current_state: Box<State>;
    current_state = Box::new(GameState::new());

    let mut event_pump = ctx.sdl_context.event_pump().unwrap();
    let mut keys = HashSet::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(a), .. } => {
                    keys.insert(a);
                },
                Event::KeyUp { keycode: Some(a), .. } => {
                    keys.remove(&a);
                },
                _ => {},
            }
        }

        match current_state.update(ctx, &keys) {
            StateInstructions::Continue => (),
            StateInstructions::GameOver => {
                current_state = Box::new(GameOver::new());
                continue 'running;
            },
            StateInstructions::Restart => {
                current_state = Box::new(GameState::new());
                continue 'running;
            },
        }

        magneto::graphics::clear(44.0 / 255.0, 43.0 / 255.0, 138.0 / 255.0, 1.0);
        current_state.render(ctx);
        magneto::graphics::swap(ctx);
    }
}
