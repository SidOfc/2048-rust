use super::masks::{COL_MASK, ROW_MASK};
use super::moves::{Moves, Direction};
use rand::{thread_rng, Rng};

lazy_static! { static ref MOVES: Moves = Moves::generate(); }

// game container.
pub struct Game { pub board: u64 }
impl Game {
    pub fn new() -> Self {
        let mut game = Game { board: 0x0000_0000_0000_0000_u64 };

        game.board |= Self::spawn_tile(game.board);
        game.board |= Self::spawn_tile(game.board);

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
                let result_board = Self::execute(game.board, &mv);

                if game.board == result_board {
                    attempted.push(mv);
                } else {
                    game.board  = result_board;
                    game.board |= Self::spawn_tile(game.board);
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

    pub fn board_info(board: u64, table: &Vec<u64>) -> u64 {
        table[((board >>  0) & ROW_MASK) as usize] +
        table[((board >> 16) & ROW_MASK) as usize] +
        table[((board >> 32) & ROW_MASK) as usize] +
        table[((board >> 48) & ROW_MASK) as usize]
    }

    pub fn score(board: u64) -> u64 {
        Self::board_info(board, &MOVES.scores)
    }

    pub fn transpose(board: u64) -> u64 {
        let a1 = board & 0xF0F0_0F0F_F0F0_0F0F_u64;
        let a2 = board & 0x0000_F0F0_0000_F0F0_u64;
        let a3 = board & 0x0F0F_0000_0F0F_0000_u64;

        let a  = a1 | (a2 << 12) | (a3 >> 12);

        let b1 = a & 0xFF00_FF00_00FF_00FF_u64;
        let b2 = a & 0x00FF_00FF_0000_0000_u64;
        let b3 = a & 0x0000_0000_FF00_FF00_u64;

        b1 | (b2 >> 24) | (b3 << 24)
    }

    pub fn column_from(row: u64) -> u64 {
        (row | (row << 12) | (row << 24) | (row << 36)) & COL_MASK
    }


    pub fn tile() -> u64 {
        if thread_rng().gen_range(0, 10) == 10 { 2 } else { 1 }
    }

    pub fn spawn_tile(board: u64) -> u64 {
        let mut tmp = board;
        let mut idx = thread_rng().gen_range(0, Self::count_empty(board));
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

    pub fn count_empty(board: u64) -> u32 {
        let mut empty = 0;

        for i in 0..16 { if ((board >> (i * 4)) & 0xF) == 0 { empty += 1 } }

        empty
    }

    fn move_up(board: u64) -> u64 {
        let mut result = board;
        let transposed = Self::transpose(board);

        result ^= MOVES.up[((transposed >>  0) & ROW_MASK) as usize] <<  0;
        result ^= MOVES.up[((transposed >> 16) & ROW_MASK) as usize] <<  4;
        result ^= MOVES.up[((transposed >> 32) & ROW_MASK) as usize] <<  8;
        result ^= MOVES.up[((transposed >> 48) & ROW_MASK) as usize] << 12;

        result
    }

    fn move_down(board: u64) -> u64 {
        let mut result = board;
        let transposed = Self::transpose(board);

        result ^= MOVES.down[((transposed >>  0) & ROW_MASK) as usize] <<  0;
        result ^= MOVES.down[((transposed >> 16) & ROW_MASK) as usize] <<  4;
        result ^= MOVES.down[((transposed >> 32) & ROW_MASK) as usize] <<  8;
        result ^= MOVES.down[((transposed >> 48) & ROW_MASK) as usize] << 12;

        result
    }

    fn move_right(board: u64) -> u64 {
        let mut result = board;

        result ^= MOVES.right[((board >>  0) & ROW_MASK) as usize] <<  0;
        result ^= MOVES.right[((board >> 16) & ROW_MASK) as usize] << 16;
        result ^= MOVES.right[((board >> 32) & ROW_MASK) as usize] << 32;
        result ^= MOVES.right[((board >> 48) & ROW_MASK) as usize] << 48;

        result
    }

    fn move_left(board: u64) -> u64 {
        let mut result: u64 = board;

        result ^= MOVES.left[((board >>  0) & ROW_MASK) as usize] <<  0;
        result ^= MOVES.left[((board >> 16) & ROW_MASK) as usize] << 16;
        result ^= MOVES.left[((board >> 32) & ROW_MASK) as usize] << 32;
        result ^= MOVES.left[((board >> 48) & ROW_MASK) as usize] << 48;

        result
    }
}
