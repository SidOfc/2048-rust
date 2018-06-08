use super::moves::Moves;
use super::masks::ROW_MASK;
use super::helpers::Helpers;

// game state.
// includes margin property to offset printing the board
// from the left edge of the screen.
pub struct Game<'a> {
    pub board: u64,
    pub moves: &'a Moves
}

// game functions.
impl<'a> Game<'a> {
    pub fn move_up(&mut self) {
        let mut result: u64 = self.board;
        let transposed      = Helpers::transpose(self.board);

        result ^= self.moves.up[((transposed >>  0) & ROW_MASK) as usize] <<  0;
        result ^= self.moves.up[((transposed >> 16) & ROW_MASK) as usize] <<  4;
        result ^= self.moves.up[((transposed >> 32) & ROW_MASK) as usize] <<  8;
        result ^= self.moves.up[((transposed >> 48) & ROW_MASK) as usize] << 12;

        self.board = result;
    }

    pub fn move_down(&mut self) {
        let mut result: u64 = self.board;
        let transposed      = Helpers::transpose(self.board);

        result ^= self.moves.down[((transposed >>  0) & ROW_MASK) as usize] <<  0;
        result ^= self.moves.down[((transposed >> 16) & ROW_MASK) as usize] <<  4;
        result ^= self.moves.down[((transposed >> 32) & ROW_MASK) as usize] <<  8;
        result ^= self.moves.down[((transposed >> 48) & ROW_MASK) as usize] << 12;

        self.board = result;
    }

    pub fn move_right(&mut self) {
        let mut result: u64 = self.board;

        result ^= self.moves.right[((self.board >>  0) & ROW_MASK) as usize] <<  0;
        result ^= self.moves.right[((self.board >> 16) & ROW_MASK) as usize] << 16;
        result ^= self.moves.right[((self.board >> 32) & ROW_MASK) as usize] << 32;
        result ^= self.moves.right[((self.board >> 48) & ROW_MASK) as usize] << 48;

        self.board = result;
    }

    pub fn move_left(&mut self) {
        let mut result: u64 = self.board;

        result ^= self.moves.left[((self.board >>  0) & ROW_MASK) as usize] <<  0;
        result ^= self.moves.left[((self.board >> 16) & ROW_MASK) as usize] << 16;
        result ^= self.moves.left[((self.board >> 32) & ROW_MASK) as usize] << 32;
        result ^= self.moves.left[((self.board >> 48) & ROW_MASK) as usize] << 48;

        self.board = result;
    }
}
