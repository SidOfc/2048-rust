// references:
//  - https://github.com/nneonneo/2048-ai/blob/master/2048.h
//  - https://github.com/nneonneo/2048-ai/blob/master/2048.cpp
//  - https://stackoverflow.com/questions/22342854/what-is-the-optimal-algorithm-for-the-game-2048

#[allow(dead_code)]

#[macro_use]
extern crate lazy_static;

mod moves;
mod game;

pub use moves::{Moves, Direction};
pub use game::Game;

