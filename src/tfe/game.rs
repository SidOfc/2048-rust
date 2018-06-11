use rand::{thread_rng, Rng};
use super::moves::{Moves, Direction};
use super::masks::ROW_MASK;
use super::helpers::Helpers;

lazy_static! {
    static ref MOVES: Moves = Moves::generate();
}

// game container.
pub struct Game {
    pub board: u64
}

impl Game {
    pub fn new() -> Self {
        let mut game = Game { board: 0x0000_0000_0000_0000_u64 };

        game.board |= game.spawn_tile();
        game.board |= game.spawn_tile();

        game
    }

    pub fn play<'a, F: Fn(u64, &Vec<Direction>) -> Direction>(mv: F) -> Self {
        let mut game = Self::new();
        let mut attempted: Vec<Direction> = Vec::with_capacity(4);

        loop {
            let mv = mv(game.board, &attempted);

            if mv == Direction::None || attempted.len() == 4 {
                break
            } else if !attempted.iter().any(|dir| dir == &mv) {
                let result_board = Game::execute(game.board, &mv);

                if game.board == result_board {
                    attempted.push(mv);
                } else {
                    game.board  = result_board;
                    game.board |= game.spawn_tile();
                    attempted.clear();
                }
            }
        }

        game
    }

    pub fn execute(board: u64, direction: &Direction) -> u64 {
        match direction {
            Direction::Left  => Self::move_left(board),
            Direction::Right => Self::move_right(board),
            Direction::Down  => Self::move_down(board),
            Direction::Up    => Self::move_up(board),
            Direction::None  => board
        }
    }

    pub fn score(&self) -> u64 {
        MOVES.scores[((self.board >>  0) & ROW_MASK) as usize] +
        MOVES.scores[((self.board >> 16) & ROW_MASK) as usize] +
        MOVES.scores[((self.board >> 32) & ROW_MASK) as usize] +
        MOVES.scores[((self.board >> 48) & ROW_MASK) as usize]
    }

    pub fn move_up(board: u64) -> u64 {
        let mut result = board;
        let transposed = Helpers::transpose(board);

        result ^= MOVES.up[((transposed >>  0) & ROW_MASK) as usize] <<  0;
        result ^= MOVES.up[((transposed >> 16) & ROW_MASK) as usize] <<  4;
        result ^= MOVES.up[((transposed >> 32) & ROW_MASK) as usize] <<  8;
        result ^= MOVES.up[((transposed >> 48) & ROW_MASK) as usize] << 12;

        result
    }

    pub fn move_down(board: u64) -> u64 {
        let mut result = board;
        let transposed = Helpers::transpose(board);

        result ^= MOVES.down[((transposed >>  0) & ROW_MASK) as usize] <<  0;
        result ^= MOVES.down[((transposed >> 16) & ROW_MASK) as usize] <<  4;
        result ^= MOVES.down[((transposed >> 32) & ROW_MASK) as usize] <<  8;
        result ^= MOVES.down[((transposed >> 48) & ROW_MASK) as usize] << 12;

        result
    }

    pub fn move_right(board: u64) -> u64 {
        let mut result = board;

        result ^= MOVES.right[((board >>  0) & ROW_MASK) as usize] <<  0;
        result ^= MOVES.right[((board >> 16) & ROW_MASK) as usize] << 16;
        result ^= MOVES.right[((board >> 32) & ROW_MASK) as usize] << 32;
        result ^= MOVES.right[((board >> 48) & ROW_MASK) as usize] << 48;

        result
    }

    pub fn move_left(board: u64) -> u64 {
        let mut result: u64 = board;

        result ^= MOVES.left[((board >>  0) & ROW_MASK) as usize] <<  0;
        result ^= MOVES.left[((board >> 16) & ROW_MASK) as usize] << 16;
        result ^= MOVES.left[((board >> 32) & ROW_MASK) as usize] << 32;
        result ^= MOVES.left[((board >> 48) & ROW_MASK) as usize] << 48;

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
