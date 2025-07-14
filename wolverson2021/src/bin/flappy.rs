//! Código referente ao capítulo 3 do livro
//! Wolverson, H. (2021). Hands-On Rust (1st ed). Pragmatic Programmers, LLC.
enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    mode: GameMode
}

impl State {
    fn new() -> Self {
        State { mode: GameMode::Menu }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut prelude::BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx), //TODO
            GameMode::End => self.dead(ctx), //TODO
            GameMode::Playing => self.play(ctx), //TODO
        }        
    }
}

use bracket_lib::{
    prelude::{main_loop, BError, BTermBuilder, GameState},
    *,
};
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new())
}
