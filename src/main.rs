#[macro_use]
extern crate lazy_static;

// silence warning about unused methods
#[allow(dead_code)]

mod tfe;
use tfe::{Game, Helpers};

// references:
//  - https://github.com/nneonneo/2048-ai/blob/master/2048.h
//  - https://github.com/nneonneo/2048-ai/blob/master/2048.cpp

fn main() {
    let mut g = Game::new();

    Helpers::print(g.board);
    g.move_up();
    Helpers::print(g.board);
}
