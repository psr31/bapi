use magneto::Context;
use sdl2::keyboard::Keycode;

use std::collections::HashSet;

pub enum StateInstructions {
    Continue, 
    GameOver,
    Restart,
}

pub trait State {
    fn update(&mut self, ctx: &mut Context, keys: &HashSet<Keycode>) -> StateInstructions;
    fn render(&mut self, ctx: &mut Context);
}