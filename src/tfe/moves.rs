use super::Helpers;

// container for moves.
pub struct Moves {
    pub left:   Vec<u64>,
    pub right:  Vec<u64>,
    pub down:   Vec<u64>,
    pub up:     Vec<u64>,
    pub scores: Vec<u64>
}

impl Moves {
    pub fn generate() -> Self {
        // initialization of move tables
        let mut left_moves  = vec![0; 65536];
        let mut right_moves = vec![0; 65536];
        let mut up_moves    = vec![0; 65536];
        let mut down_moves  = vec![0; 65536];
        let mut scores      = vec![0; 65536];

        // debug
        // let row = 0x1111;
        for row in 0..65536 {
            // break row into cells
            let mut line = [
                (row >>  0) & 0xF,
                (row >>  4) & 0xF,
                (row >>  8) & 0xF,
                (row >> 12) & 0xF
            ];

            // calculate score for given row
            let mut s = 0;

            for i in 0..4 {
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
                    j = j + 1;
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
                i = i + 1;
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

            right_moves[row_idx] = row                           ^ result;
            left_moves[rev_idx]  = rev_row                       ^ rev_res;
            up_moves[rev_idx]    = Helpers::column_from(rev_row) ^ Helpers::column_from(rev_res);
            down_moves[row_idx]  = Helpers::column_from(row)     ^ Helpers::column_from(result);
        };

        return Moves { left: left_moves, right: right_moves, down: down_moves, up: up_moves, scores: scores };
    }
}
