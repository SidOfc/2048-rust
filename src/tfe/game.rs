use super::moves::Moves;
use super::masks::ROW_MASK;
use super::helpers::Helpers;

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
        return Game { board: 0x0001_0001_0002_0002_u64 };
    }

    pub fn move_up(&mut self) {
        let mut result: u64 = self.board;
        let transposed      = Helpers::transpose(self.board);

        result ^= MOVES.up[((transposed >>  0) & ROW_MASK) as usize] <<  0;
        result ^= MOVES.up[((transposed >> 16) & ROW_MASK) as usize] <<  4;
        result ^= MOVES.up[((transposed >> 32) & ROW_MASK) as usize] <<  8;
        result ^= MOVES.up[((transposed >> 48) & ROW_MASK) as usize] << 12;

        self.board = result;
    }

    pub fn move_down(&mut self) {
        let mut result: u64 = self.board;
        let transposed      = Helpers::transpose(self.board);

        result ^= MOVES.down[((transposed >>  0) & ROW_MASK) as usize] <<  0;
        result ^= MOVES.down[((transposed >> 16) & ROW_MASK) as usize] <<  4;
        result ^= MOVES.down[((transposed >> 32) & ROW_MASK) as usize] <<  8;
        result ^= MOVES.down[((transposed >> 48) & ROW_MASK) as usize] << 12;

        self.board = result;
    }

    pub fn move_right(&mut self) {
        let mut result: u64 = self.board;

        result ^= MOVES.right[((self.board >>  0) & ROW_MASK) as usize] <<  0;
        result ^= MOVES.right[((self.board >> 16) & ROW_MASK) as usize] << 16;
        result ^= MOVES.right[((self.board >> 32) & ROW_MASK) as usize] << 32;
        result ^= MOVES.right[((self.board >> 48) & ROW_MASK) as usize] << 48;

        self.board = result;
    }

    pub fn move_left(&mut self) {
        let mut result: u64 = self.board;

        result ^= MOVES.left[((self.board >>  0) & ROW_MASK) as usize] <<  0;
        result ^= MOVES.left[((self.board >> 16) & ROW_MASK) as usize] << 16;
        result ^= MOVES.left[((self.board >> 32) & ROW_MASK) as usize] << 32;
        result ^= MOVES.left[((self.board >> 48) & ROW_MASK) as usize] << 48;

        self.board = result;
    }
}
