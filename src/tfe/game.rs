use rand::{thread_rng, Rng};
use super::moves::{Moves, Direction};
use super::masks::ROW_MASK;
use super::helpers::Helpers;
use std::time::Duration;
use std::thread;

lazy_static! {
    static ref MOVES: Moves = Moves::generate();
}

// game state.
// includes margin property to offset printing the board
// from the left edge of the screen.
pub struct Game {
    pub board: u64
}

// game functions.
impl Game {
    pub fn new() -> Self {
        let mut game = Game { board: 0x0000_0000_0000_0000_u64 };

        game.spawn_tile()
            .spawn_tile();

        game
    }

    pub fn play() -> Self {
        let mut game = Self::new();
        let mut cntr = 0;
        let mut cncl = 0;

        loop {
            let b = game.board;
            let m = game.move_random();
            cntr += 1;
            cncl += 1;
            if cntr > 100_000_000 { break };
            if cncl > 100 { break };
            if game.count_empty() == 0 { break };

            if game.board == b { continue } else { cncl = 0 };
            println!("moves: {:?} move: {:?} score: {:?} empty: {:?}", cntr, m, game.score(), game.count_empty());

            thread::sleep(Duration::from_millis(100));
        }

        game
    }

    pub fn execute(&mut self, direction: Direction) -> &mut Self {
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

    pub fn move_random(&mut self) -> &'static str {
        match thread_rng().gen_range(0, 4) {
            0 => {
                self.execute(Direction::Left);
                "left"
            },
            1 => {
                self.execute(Direction::Right);
                "right"
            },
            2 => {
                self.execute(Direction::Up);
                "up"
            },
            3 => {
                self.execute(Direction::Down);
                "down"
            }
            _ => "none"
        }
    }

    pub fn move_up(&mut self) -> &mut Self {
        let mut result: u64 = self.board;
        let transposed      = Helpers::transpose(self.board);

        result ^= MOVES.up[((transposed >>  0) & ROW_MASK) as usize] <<  0;
        result ^= MOVES.up[((transposed >> 16) & ROW_MASK) as usize] <<  4;
        result ^= MOVES.up[((transposed >> 32) & ROW_MASK) as usize] <<  8;
        result ^= MOVES.up[((transposed >> 48) & ROW_MASK) as usize] << 12;

        if self.board != result {
            self.board = result;
            self.spawn_tile();
        }

        self
    }

    pub fn move_down(&mut self) -> &mut Self {
        let mut result: u64 = self.board;
        let transposed      = Helpers::transpose(self.board);

        result ^= MOVES.down[((transposed >>  0) & ROW_MASK) as usize] <<  0;
        result ^= MOVES.down[((transposed >> 16) & ROW_MASK) as usize] <<  4;
        result ^= MOVES.down[((transposed >> 32) & ROW_MASK) as usize] <<  8;
        result ^= MOVES.down[((transposed >> 48) & ROW_MASK) as usize] << 12;

        if self.board != result {
            self.board = result;
            self.spawn_tile();
        }

        self
    }

    pub fn move_right(&mut self) -> &mut Self {
        let mut result: u64 = self.board;

        result ^= MOVES.right[((self.board >>  0) & ROW_MASK) as usize] <<  0;
        result ^= MOVES.right[((self.board >> 16) & ROW_MASK) as usize] << 16;
        result ^= MOVES.right[((self.board >> 32) & ROW_MASK) as usize] << 32;
        result ^= MOVES.right[((self.board >> 48) & ROW_MASK) as usize] << 48;

        if self.board != result {
            self.board = result;
            self.spawn_tile();
        }

        self
    }

    pub fn move_left(&mut self) -> &mut Self {
        let mut result: u64 = self.board;

        result ^= MOVES.left[((self.board >>  0) & ROW_MASK) as usize] <<  0;
        result ^= MOVES.left[((self.board >> 16) & ROW_MASK) as usize] << 16;
        result ^= MOVES.left[((self.board >> 32) & ROW_MASK) as usize] << 32;
        result ^= MOVES.left[((self.board >> 48) & ROW_MASK) as usize] << 48;

        if self.board != result {
            self.board = result;
            self.spawn_tile();
        }

        self
    }

    fn tile() -> u64 {
        if thread_rng().gen_range(0, 10) == 10 { 2 } else { 1 }
    }

    fn spawn_tile(&mut self) -> &mut Self {
        let mut tmp = self.board;
        let mut idx = thread_rng().gen_range(0, self.count_empty());
        let mut t   = Self::tile();

        loop {
            while (tmp & 0xF) != 0 {
                tmp >>= 4;
                t   <<= 4;
            }

            if idx == 0 { break } else { idx -= 1 };

            tmp >>= 4;
            t   <<= 4;
        }

        self.board |= t;

        self
    }

    fn count_empty(&self) -> u32 {
        self.board.count_zeros() / 4
    }
}
