//! A 2048 implementation that uses bit-shifting and a pre-computed move table
//! this implementation is designed to provide low overhead when testing an algorithm on a large
//! amount of games. On a mid-2015 MBP Retina (2.5GHz i7) 10,000,000 games take about 80 seconds to
//! complete running on 8 threads (1,250,000 games per thread) by executing random moves, avg score ~2k.
//!
//! The board itself is encoded as a `u64`. This means that each tile has 4 bits (64 / 16 = 4) to store its
//! value. Since the maximum value of setting all four bits to 1 is `15` we cannot use it to
//! display the value directly. Instead we use these 4 bits as the power value: `2 << 15 = 65536`, `2
//! << 14 = 32768`, `2 << 13 = 16384`, `2 << 12 = 8192`, etc...
//!
//! A simple way to play the game automatically is to use `tfe::Game::play`:
//!
//! ```
//! extern crate tfe;
//! use tfe::{Game, Direction};
//!
//! let game = Game::play(|board, failed| Direction::sample_without(failed));
//! println!("score: {:<6} board hex: {:016x}", Game::score(game.board), game.board);
//! ```
//!
//! The `play` method takes a callback that accepts a `board: u64` and `failed: &Vec<Direction>` as
//! parameters and returns the next `Direction` to move in.
//! A special `Direction::None` can be used to indicate that no move was possible, the game will
//! quit automatically when `Direction::None` is encountered.
//! The game will also terminate if each distinct move has been attempted and failed without any
//! successfull move in between the failed moves.
//!
//! ---
//!
//! references:
//!  - https://github.com/nneonneo/2048-ai/blob/master/2048.h
//!  - https://github.com/nneonneo/2048-ai/blob/master/2048.cpp
//!  - https://stackoverflow.com/questions/22342854/what-is-the-optimal-algorithm-for-the-game-2048

#[allow(dead_code)]

#[macro_use]
extern crate lazy_static;

mod moves;
mod game;

pub use moves::{Moves, Direction};
pub use game::Game;

