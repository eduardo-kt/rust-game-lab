//! Código referente ao capítulo 3 do livro 
//! Wolverson, H. (2021). Hands-On Rust (1st ed). Pragmatic Programmers, LLC.

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut prelude::BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, Bracket Terminal");
    }
}

    println!("Hello world flappy dragon");
}