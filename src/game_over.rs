use crate::state::{State, StateInstructions};
use crate::SCREEN_SIZE;

use magneto::Context;
use magneto::graphics;
use graphics::Font;
use graphics::Text;
use sdl2::keyboard::Keycode;

use std::collections::HashSet;


pub struct GameOver {
    font_large: Font,
    font_small: Font,
    game_over_text: Text,
    instruction_text: Text,
}

impl GameOver {
    pub fn new(ctx: &mut Context) -> GameOver {
        let seperation = 10.0;

        let font_large = Font::new(ctx, "./resources/snes.ttf", 200).unwrap();

        let mut game_over_text = Text::new("Game Over", &font_large, (1.0, 1.0, 1.0).into());
        game_over_text.x = (SCREEN_SIZE.0 / 2.0) - (game_over_text.width / 2.0);

        let font_small = Font::new(ctx, "./resources/snes.ttf", 80).unwrap();

        let mut instruct_text = Text::new("press the spacebar to play again", &font_small, (1.0, 1.0, 1.0).into());
        instruct_text.x = (SCREEN_SIZE.0 / 2.0) - (instruct_text.width / 2.0);

        let total_height = game_over_text.height + seperation + instruct_text.height;
        game_over_text.y = (SCREEN_SIZE.1 - total_height) / 2.0;
        instruct_text.y = game_over_text.y + game_over_text.height + 10.0;

        GameOver {
            font_large: font_large,
            font_small: font_small,
            game_over_text: game_over_text,
            instruction_text: instruct_text,
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
        self.game_over_text.draw(ctx, &self.font_large);
        self.instruction_text.draw(ctx, &self.font_small);
    }
}