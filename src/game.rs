use super::rand::{thread_rng, Rng};
use super::direction::Direction;

/// Struct that contains all available moves per row for up, down, right and left.
/// Also stores the score for a given row.
///
/// Moves are stored as power values for tiles.
/// if a power value is `> 0`, print the tile value using `2 << tile` where tile is any 4-bit
/// "nybble" otherwise print a `0` instead.
struct Moves {
    pub left:   Vec<u64>,
    pub right:  Vec<u64>,
    pub down:   Vec<u64>,
    pub up:     Vec<u64>,
    pub scores: Vec<u64>
}

impl Moves {
    /// Returns the 4th bit from each row in given board OR'd.
    pub fn column_from(board: u64) -> u64 {
        (board | (board << 12) | (board << 24) | (board << 36)) & COL_MASK
    }
}

lazy_static! {
    /// Constructs a new `tfe::Moves`.
    ///
    /// `Moves` stores `right`, `left`, `up`, and `down` moves per row.
    ///  e.g. left: `0x0011 -> 0x2000` and right: `0x0011 -> 0x0002`.
    ///
    ///  Also stores the `scores` per row.
    ///  The score of a row is the sum of the tile and all intermediate tile merges.
    ///  e.g. row `0x0002` has a score of `4` and row `0x0003` has a score of `16`.
    static ref MOVES: Moves = {
                // initialization of move tables
        let mut left_moves  = vec![0; 65536];
        let mut right_moves = vec![0; 65536];
        let mut up_moves    = vec![0; 65536];
        let mut down_moves  = vec![0; 65536];
        let mut scores      = vec![0; 65536];

        for row in 0 .. 65536 {
            // break row into cells
            let mut line = [
                (row >>  0) & 0xF,
                (row >>  4) & 0xF,
                (row >>  8) & 0xF,
                (row >> 12) & 0xF
            ];

            // calculate score for given row
            let mut s = 0;

            for i in 0 .. 4 {
                if line[i] >= 2 { s += (line[i] - 1) * (1 << line[i]) }
            }

            scores[row as usize] = s;

            let mut i = 0;

            // perform a move to the left using current {row} as board
            // generates 4 output moves for up, down, left and right by transposing and reversing
            // this result.
            while i < 3 {
                // initial counter for the cell next to the current one (j)
                let mut j = i + 1;

                // find the next non-zero cell index
                while j < 4 {
                    if line[j] != 0 { break };
                    j += 1;
                };

                // if j is out of bounds (> 3), all other cells are empty and we are done looping
                if j == 4 { break };

                // this is the part responsible for skipping empty (0 value) cells
                // if the current cell is zero, shift the next non-zero cell to position i
                // and retry this entry until line[i] becomes non-zero
                if line[i] == 0 {
                    line[i] = line[j];
                    line[j] = 0;
                    continue;

                // otherwise, if the current cell and next cell are the same, merge them
                } else if line[i] == line[j] {
                    if line[i] != 0xF { line[i] += 1 };
                    line[j] = 0;
                }

                // finally, move to the next (or current, if i was 0) row
                i += 1;
            }

            // put the new row after merging back together into a "merged" row
            let result = (line[0] <<  0) |
                         (line[1] <<  4) |
                         (line[2] <<  8) |
                         (line[3] << 12);

            // right and down use normal row and result variables.
            // for left and up, we create a reverse of the row and result.
            let rev_row = (row    >> 12) & 0x000F | (row    >> 4) & 0x00F0 | (row    << 4) & 0x0F00 | (row    << 12) & 0xF000;
            let rev_res = (result >> 12) & 0x000F | (result >> 4) & 0x00F0 | (result << 4) & 0x0F00 | (result << 12) & 0xF000;

            // results are keyed by row / reverse row index.
            let row_idx = row     as usize;
            let rev_idx = rev_row as usize;

            right_moves[row_idx] = row                         ^ result;
            left_moves[rev_idx]  = rev_row                     ^ rev_res;
            up_moves[rev_idx]    = Moves::column_from(rev_row) ^ Moves::column_from(rev_res);
            down_moves[row_idx]  = Moves::column_from(row)     ^ Moves::column_from(result);
        };

        Moves { left: left_moves, right: right_moves, down: down_moves, up: up_moves, scores: scores }
    };
}

/// A mask with a single section of 16 bits set to 0.
/// Used to extract a "horizontal slice" out of a 64 bit integer.
pub static ROW_MASK: u64 = 0xFFFF;

/// A `u64` mask with 4 sections each starting after the n * 16th bit.
/// Used to extract a "vertical slice" out of a 64 bit integer.
pub static COL_MASK: u64 = 0x000F_000F_000F_000F_u64;

/// Struct used to play a single game of 2048.
///
/// `tfe::Game` uses a single `u64` as board value.
/// The board itself is divided into rows (x4 16 bit "row" per "board") which are
/// divided into tiles (4x 4 bit "nybbles" per "row").
///
/// All manipulations are done using bit-shifts and a precomputed table of moves and scores.
/// Every move is stored as four lookups total, one for each row. The result of XOR'ing each row
/// back into the board at the right position is the output board.
pub struct Game { pub board: u64 }
impl Game {
    /// Constructs a new `tfe::Game`.
    ///
    /// `Game` stores a board internally as a `u64`.
    ///
    /// # Examples
    ///
    /// Simple example:
    ///
    /// ```
    /// use tfe::Game;
    ///
    /// let mut game = Game::new();
    /// # println!("{:016x}", game.board);
    /// ```
    ///
    /// Accessing board value:
    ///
    /// ```
    /// use tfe::Game;
    ///
    /// let mut game = Game::new();
    /// println!("{:016x}", game.board);
    /// ```
    pub fn new() -> Self {
        let mut game = Game { board: 0x0000_0000_0000_0000_u64 };

        game.board |= Self::spawn_tile(game.board);
        game.board |= Self::spawn_tile(game.board);

        game
    }

    /// Like `new` but takes a closure that accepts two parameters and returns
    /// a `Direction`. The parameters passed to the closure:
    ///
    /// - `u64`: The current board
    /// - `&Vec<Direction>`: A list of attempted moves that had no effect.
    /// Gets cleared when a move succeeds.
    ///
    /// # Examples
    ///
    /// Simple example:
    ///
    /// ```
    /// use tfe::{Game, Direction};
    ///
    /// let game = Game::play(|_board, failed| Direction::sample_without(failed));
    /// ```
    ///
    /// In this example, the variable `game` will have a value of a single `Game` played to
    /// completion. A game is over when it has no moves left. This is true when all possible
    /// moves return the same resulting board as before the move was executed.
    ///
    /// The `failed: &Vec<Direction>` will contain **at most** 3 items, when the 4th item is added
    /// the game ends automatically without calling the closure again.
    pub fn play<F: Fn(u64, &Vec<Direction>) -> Direction>(mv: F) -> Self {
        let mut game = Self::new();
        let mut attempted: Vec<Direction> = Vec::with_capacity(4);

        loop {
            let mv = mv(game.board, &attempted);
            if !attempted.iter().any(|dir| dir == &mv) {
                let result_board = Self::execute(game.board, &mv);

                if game.board == result_board {
                    if attempted.len() == 3 { break }
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

    /// Returns `board` moved in given `direction`.
    ///
    /// - When `Direction::Left`, return board moved left
    /// - When `Direction::Right`, return board moved right
    /// - When `Direction::Down`, return board moved down
    /// - When `Direction::Up`, return board moved up
    ///
    /// # Examples
    ///
    /// Simple example:
    ///
    /// ```
    /// use tfe::{Game, Direction};
    ///
    /// let board = 0x0000_0000_0022_1100;
    /// let moved = Game::execute(board, &Direction::Left);
    ///
    /// // | 0 | 0 | 0 | 0 |      | 0 | 0 | 0 | 0 |
    /// // | 0 | 0 | 0 | 0 |  =>  | 0 | 0 | 0 | 0 |
    /// // | 0 | 0 | 4 | 4 |      | 8 | 0 | 0 | 0 |
    /// // | 2 | 2 | 0 | 0 |      | 4 | 0 | 0 | 0 |
    ///
    /// assert_eq!(board, 0x0000_0000_0022_1100);
    /// assert_eq!(moved, 0x0000_0000_3000_2000);
    /// ```
    pub fn execute(board: u64, direction: &Direction) -> u64 {
        match direction {
            Direction::Left  => Self::move_left(board),
            Direction::Right => Self::move_right(board),
            Direction::Down  => Self::move_down(board),
            Direction::Up    => Self::move_up(board)
        }
    }

    /// Returns a transposed board where rows are transformed into columns and vice versa.
    ///
    /// ```
    /// use tfe::Game;
    ///
    /// // | F | E | D | C |       | F | B | 7 | 3 |
    /// // | B | A | 9 | 8 |   =>  | E | A | 6 | 2 |
    /// // | 7 | 6 | 5 | 4 |       | D | 9 | 5 | 1 |
    /// // | 3 | 2 | 1 | 0 |       | C | 8 | 4 | 0 |
    ///
    /// assert_eq!(Game::transpose(0xFEDC_BA98_7654_3210), 0xFB73_EA62_D951_C840);
    /// ```
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

    /// Returns a `u64` board moved up.
    /// This is the same as calling `Game::execute(board, &Direction::Up)`;
    ///
    /// # Examples
    ///
    /// ```
    /// use tfe::Game;
    ///
    /// let board  = 0x0000_0000_0000_0011_u64;
    /// let result = Game::move_up(board);
    ///
    /// // | 0 | 0 | 0 | 0 |      | 0 | 0 | 1 | 1 |
    /// // | 0 | 0 | 0 | 0 |  =>  | 0 | 0 | 0 | 0 |
    /// // | 0 | 0 | 0 | 0 |      | 0 | 0 | 0 | 0 |
    /// // | 0 | 0 | 1 | 1 |      | 0 | 0 | 0 | 0 |
    ///
    /// assert_eq!(result, 0x0011_0000_0000_0000);
    /// ```
    pub fn move_up(board: u64) -> u64 {
        let mut result = board;
        let transposed = Self::transpose(board);

        result ^= MOVES.up[((transposed >>  0) & ROW_MASK) as usize] <<  0;
        result ^= MOVES.up[((transposed >> 16) & ROW_MASK) as usize] <<  4;
        result ^= MOVES.up[((transposed >> 32) & ROW_MASK) as usize] <<  8;
        result ^= MOVES.up[((transposed >> 48) & ROW_MASK) as usize] << 12;

        result
    }

    /// Returns a `u64` board moved down.
    /// This is the same as calling `Game::execute(board, &Direction::Down)`;
    ///
    /// # Examples
    ///
    /// ```
    /// use tfe::Game;
    ///
    /// let board  = 0x0011_0000_0000_0011_u64;
    /// let result = Game::move_down(board);
    ///
    /// // | 0 | 0 | 1 | 1 |      | 0 | 0 | 0 | 0 |
    /// // | 0 | 0 | 0 | 0 |  =>  | 0 | 0 | 0 | 0 |
    /// // | 0 | 0 | 0 | 0 |      | 0 | 0 | 0 | 0 |
    /// // | 0 | 0 | 1 | 1 |      | 0 | 0 | 2 | 2 |
    ///
    /// assert_eq!(result, 0x0000_0000_0000_0022);
    /// ```
    pub fn move_down(board: u64) -> u64 {
        let mut result = board;
        let transposed = Self::transpose(board);

        result ^= MOVES.down[((transposed >>  0) & ROW_MASK) as usize] <<  0;
        result ^= MOVES.down[((transposed >> 16) & ROW_MASK) as usize] <<  4;
        result ^= MOVES.down[((transposed >> 32) & ROW_MASK) as usize] <<  8;
        result ^= MOVES.down[((transposed >> 48) & ROW_MASK) as usize] << 12;

        result
    }

    /// Returns a `u64` board moved right.
    /// This is the same as calling `Game::execute(board, &Direction::Right)`;
    ///
    /// # Examples
    ///
    /// ```
    /// use tfe::Game;
    ///
    /// let board  = 0x0000_0000_0000_2211_u64;
    /// let result = Game::move_right(board);
    ///
    /// // | 0 | 0 | 0 | 0 |      | 0 | 0 | 0 | 0 |
    /// // | 0 | 0 | 0 | 0 |  =>  | 0 | 0 | 0 | 0 |
    /// // | 0 | 0 | 0 | 0 |      | 0 | 0 | 0 | 0 |
    /// // | 2 | 2 | 1 | 1 |      | 0 | 0 | 3 | 2 |
    ///
    /// assert_eq!(result, 0x0000_0000_0000_0032);
    /// ```
    pub fn move_right(board: u64) -> u64 {
        let mut result = board;

        result ^= MOVES.right[((board >>  0) & ROW_MASK) as usize] <<  0;
        result ^= MOVES.right[((board >> 16) & ROW_MASK) as usize] << 16;
        result ^= MOVES.right[((board >> 32) & ROW_MASK) as usize] << 32;
        result ^= MOVES.right[((board >> 48) & ROW_MASK) as usize] << 48;

        result
    }

    /// Returns a `u64` board moved left.
    /// This is the same as calling `Game::execute(board, &Direction::Left)`;
    ///
    /// # Examples
    ///
    /// ```
    /// use tfe::Game;
    ///
    /// let board  = 0x0000_0000_0000_2211_u64;
    /// let result = Game::move_left(board);
    ///
    /// // | 0 | 0 | 0 | 0 |      | 0 | 0 | 0 | 0 |
    /// // | 0 | 0 | 0 | 0 |  =>  | 0 | 0 | 0 | 0 |
    /// // | 0 | 0 | 0 | 0 |      | 0 | 0 | 0 | 0 |
    /// // | 2 | 2 | 1 | 1 |      | 3 | 2 | 0 | 0 |
    ///
    /// assert_eq!(result, 0x0000_0000_0000_3200);
    /// ```
    pub fn move_left(board: u64) -> u64 {
        let mut result: u64 = board;

        result ^= MOVES.left[((board >>  0) & ROW_MASK) as usize] <<  0;
        result ^= MOVES.left[((board >> 16) & ROW_MASK) as usize] << 16;
        result ^= MOVES.left[((board >> 32) & ROW_MASK) as usize] << 32;
        result ^= MOVES.left[((board >> 48) & ROW_MASK) as usize] << 48;

        result
    }

    /// Returns the count of tiles with a value of `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tfe::Game;
    ///
    /// let board  = 0x0000_0000_0000_2211_u64;
    /// let result = Game::count_empty(board);
    ///
    /// assert_eq!(result, 12);
    /// ```
    pub fn count_empty(board: u64) -> u32 {
        let mut empty = 0;

        for i in 0 .. 16 { if ((board >> (i * 4)) & 0xF) == 0 { empty += 1 } }

        empty
    }

    /// Returns the sum of 4 lookups in `table` for each "row" in `board`.
    pub fn table_helper(board: u64, table: &Vec<u64>) -> u64 {
        table[((board >>  0) & ROW_MASK) as usize] +
        table[((board >> 16) & ROW_MASK) as usize] +
        table[((board >> 32) & ROW_MASK) as usize] +
        table[((board >> 48) & ROW_MASK) as usize]
    }

    /// Returns the score of a given `board`.
    /// The score of a single tile is the sum of the tile value and all intermediate merged tiles.
    pub fn score(board: u64) -> u64 {
        Self::table_helper(board, &MOVES.scores)
    }

    /// Returns a `2` with 90% chance and `4` with 10% chance.
    pub fn tile() -> u64 {
        if thread_rng().gen_range(0, 10) == 10 { 2 } else { 1 }
    }

    /// Returns a `1` shifted to the position of any `0` bit in `board` randomly.
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
}
