#[allow(dead_code)]

#[macro_use]
extern crate lazy_static;

mod moves;
mod game;

pub use moves::{Moves, Direction};
pub use game::Game;

