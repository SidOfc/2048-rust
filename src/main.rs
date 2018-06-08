#[macro_use]
extern crate lazy_static;
extern crate rand;

// silence warning about unused methods
#[allow(dead_code)]

mod tfe;
use tfe::{Game, Helpers};

// references:
//  - https://github.com/nneonneo/2048-ai/blob/master/2048.h
//  - https://github.com/nneonneo/2048-ai/blob/master/2048.cpp

fn main() {
    let g = Game::play();
    Helpers::print(g.board);
    println!("done! score: {:?}", g.score());
}
