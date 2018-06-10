use rand::{thread_rng, Rng};
use super::moves::{Moves, Direction};
use super::masks::ROW_MASK;
use super::helpers::Helpers;
use std::time::Duration;
use std::thread;

lazy_static! {
    static ref MOVES: Moves = Moves::generate();
}

// game container.
pub struct Game {
    pub board: u64,
    pub mv: u64
}

impl Game {
    pub fn new() -> Self {
        let mut game = Game { board: 0x0000_0000_0000_0000_u64, mv: 0 };

        game.board |= game.spawn_tile();
        game.board |= game.spawn_tile();

        game
    }

    pub fn play() -> Self {
        let mut game = Self::new();

        loop {
            let mut moved = false;
            for dir in &Direction::vec() {
                let result_board = game.execute(&dir);
                if game.board != result_board {
                    // let empty   = game.count_empty();
                    moved       = true;
                    game.mv    += 1;
                    game.board  = result_board;
                    game.board |= game.spawn_tile();

                    // Helpers::print(game.board);
                    // if cntr > 1 { print!("\r") }
                    // println!("moves: {:5} score: {:5} empty: {:5} move: {:?}", game.mv, game.score(), empty, &dir);
                    // thread::sleep(Duration::from_millis(500))

                    break;
                }
            }

            if !moved { break }
        }

        game
    }

    pub fn execute(&self, direction: &Direction) -> u64 {
        match direction {
            Direction::Left  => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::Up    => self.move_up(),
            Direction::Down  => self.move_down()
        }
    }

    pub fn score(&self) -> u64 {
        MOVES.scores[((self.board >>  0) & ROW_MASK) as usize] +
        MOVES.scores[((self.board >> 16) & ROW_MASK) as usize] +
        MOVES.scores[((self.board >> 32) & ROW_MASK) as usize] +
        MOVES.scores[((self.board >> 48) & ROW_MASK) as usize]
    }

    pub fn move_up(&self) -> u64 {
        let mut result: u64 = self.board;
        let transposed      = Helpers::transpose(self.board);

        result ^= MOVES.up[((transposed >>  0) & ROW_MASK) as usize] <<  0;
        result ^= MOVES.up[((transposed >> 16) & ROW_MASK) as usize] <<  4;
        result ^= MOVES.up[((transposed >> 32) & ROW_MASK) as usize] <<  8;
        result ^= MOVES.up[((transposed >> 48) & ROW_MASK) as usize] << 12;

        result
    }

    pub fn move_down(&self) -> u64 {
        let mut result: u64 = self.board;
        let transposed      = Helpers::transpose(self.board);

        result ^= MOVES.down[((transposed >>  0) & ROW_MASK) as usize] <<  0;
        result ^= MOVES.down[((transposed >> 16) & ROW_MASK) as usize] <<  4;
        result ^= MOVES.down[((transposed >> 32) & ROW_MASK) as usize] <<  8;
        result ^= MOVES.down[((transposed >> 48) & ROW_MASK) as usize] << 12;

        result
    }

    pub fn move_right(&self) -> u64 {
        let mut result: u64 = self.board;

        result ^= MOVES.right[((self.board >>  0) & ROW_MASK) as usize] <<  0;
        result ^= MOVES.right[((self.board >> 16) & ROW_MASK) as usize] << 16;
        result ^= MOVES.right[((self.board >> 32) & ROW_MASK) as usize] << 32;
        result ^= MOVES.right[((self.board >> 48) & ROW_MASK) as usize] << 48;

        result
    }

    pub fn move_left(&self) -> u64 {
        let mut result: u64 = self.board;

        result ^= MOVES.left[((self.board >>  0) & ROW_MASK) as usize] <<  0;
        result ^= MOVES.left[((self.board >> 16) & ROW_MASK) as usize] << 16;
        result ^= MOVES.left[((self.board >> 32) & ROW_MASK) as usize] << 32;
        result ^= MOVES.left[((self.board >> 48) & ROW_MASK) as usize] << 48;

        result
    }

    fn tile() -> u64 {
        if thread_rng().gen_range(0, 10) == 10 { 2 } else { 1 }
    }

    fn spawn_tile(&self) -> u64 {
        let mut tmp = self.board;
        let mut idx = thread_rng().gen_range(0, self.count_empty());
        let mut t   = Self::tile();

        loop {
            while (tmp & 0xF) != 0 {
                tmp >>= 4;
                t   <<= 4;
            }

            if idx == 0 { break } else { idx -= 1 }

            tmp >>= 4;
            t   <<= 4
        }

        t
    }

    fn count_empty(&self) -> u32 {
        let mut empty = 0;

        for i in 0..16 { if ((self.board >> (i * 4)) & 0xF) == 0 { empty += 1 } }

        empty
    }
}
