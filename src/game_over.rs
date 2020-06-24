use crate::state::{State, StateInstructions};

use magneto::Context;
use sdl2::keyboard::Keycode;

use std::collections::HashSet;


pub struct GameOver {

}

impl GameOver {
    pub fn new() -> GameOver {
        GameOver {

        }
    }
}

impl State for GameOver {
    fn update(&mut self, ctx: &mut Context, keys: &HashSet<Keycode>) -> StateInstructions {
        if keys.contains(&Keycode::Space) {
            StateInstructions::Restart
        } else {
            StateInstructions::Continue
        }
    }

    fn render(&mut self, ctx: &mut Context) {
    
    }
}