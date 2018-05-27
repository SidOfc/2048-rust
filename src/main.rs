#![allow(dead_code)]

// references:
//  - https://github.com/nneonneo/2048-ai/blob/master/2048.h
//  - https://github.com/nneonneo/2048-ai/blob/master/2048.cpp

//  module tfe - [t]wenty_[f]orty_[e]ight.
//  namespace for 2048 game module.
mod tfe {
    // a 64bit mask with a single section of 16 bits set to 0.
    // used to extract a "horizontal slice" out of a 64 bit integer.
    static ROW_MASK: u64 = 0x0000_0000_0000_FFFF_u64;

    // a 64bit mask with the leftmost 4 bits set to to 1.
    // used to extract single cell.
    static VAL_MASK: u64 = 0x0000_0000_0000_000F_u64;

    // a 64bit mask with 4 sections each starting after the n * 16th bit.
    // used to extract a "vertical slice" out of a 64 bit integer.
    static COL_MASK: u64 = 0x000F_000F_000F_000F_u64;

    #[derive(Debug)]
    pub struct Moves {
        pub left: Vec<u64>,
        pub right: Vec<u64>,
        pub down: Vec<u64>,
        pub up: Vec<u64>
    }

    // game state.
    // includes margin property to offset printing the board
    // from the left edge of the screen.
    #[derive(Debug)]
    pub struct Game {
        pub board: u64,
        pub moves: Moves
    }

    // game functions.
    impl Game {
        // print board from self.board
        pub fn print(&self) {
            let spacer: String  = " ".repeat(10);

            // map 4 bits to one digit, 64 bits / 16 cells / 4 bits per cell.
            let cells: Vec<u64> = (0..16).rev().map(|n| 1_u64 << (self.board >> (n << 2) & VAL_MASK))
                                               .map(|r| if r > 1 { r } else { 0 }).collect();

            // print top area.
            println!("{}*-------------------------------------------*", spacer);
            println!("{}|   _____________________________________   |", spacer);
            println!("{}|   |        |        |        |        |   |", spacer);

            // print middle area.
            println!("{}|   |{:^8}|{:^8}|{:^8}|{:^8}|   |",             spacer, cells[0], cells[1], cells[2], cells[3]);
            println!("{}|   |--------|--------|--------|--------|   |", spacer);
            println!("{}|   |{:^8}|{:^8}|{:^8}|{:^8}|   |",             spacer, cells[4], cells[5], cells[6], cells[7]);
            println!("{}|   |--------|--------|--------|--------|   |", spacer);
            println!("{}|   |{:^8}|{:^8}|{:^8}|{:^8}|   |",             spacer, cells[8], cells[9], cells[10], cells[11]);
            println!("{}|   |--------|--------|--------|--------|   |", spacer);
            println!("{}|   |{:^8}|{:^8}|{:^8}|{:^8}|   |",             spacer, cells[12], cells[13], cells[14], cells[15]);

            // print bottom area.
            println!("{}|   |________|________|________|________|   |", spacer);
            println!("{}|                                           |", spacer);
            println!("{}*-------------------------------------------*", spacer);
        }
    }
}

fn main() {
    // initialization of move tables
    let mut left_moves  = vec![0; 65536];
    let mut right_moves = vec![0; 65536];
    let mut up_moves    = vec![0; 65536];
    let mut down_moves  = vec![0; 65536];

    for row in 0..65536 {
        // break row into cells
        let mut line = [
            (row >>  0) & 0xf,
            (row >>  4) & 0xf,
            (row >>  8) & 0xf,
            (row >> 12) & 0xf
        ];

        let mut i = 0;

        // loop upto (including) the second to last cell from left to right (i)
        while i < 3 {
            // initial counter for the cell next to the current one (j)
            let mut j = i + 1;

            // find the next non-zero cell index
            while j < 4 { if line[j] != 0 { break } else { j = j + 1} };

            // if j is out of bounds (> 3), all other cells are empty and we are done looping
            if j == 4 { break };

            // if the current cell is zero, shift the next non-zero cell to position i
            // also decrement i by 1 to check the entry that we just moved in the next iteration
            if line[i] == 0 {
                line[i] = line[j];
                line[j] = 0;
                if i > 0 { i = i - 1 };

            // otherwise, if the current cell and next cell are the same, merge them
            } else if line[i] == line[j] {
                if line[i] != 0xf { line[i] += 1 };
                line[j] = 0;
            }

            // finally, move to the next (or current, if i was 0) row
            i = i + 1;
        }

        // put the new row after merging back together into a "merged" row
        let result: u64 = (line[0] <<  0) |
                          (line[1] <<  4) |
                          (line[2] <<  8) |
                          (line[3] << 12);

        // add xorred result to table so we can xor it back into the result board during runtime
        // for now, only left table is implemented, the other directions require additional
        // functions so we'll focus on left for now
        left_moves[(row as usize)] = row ^ result;
    };

    let moves    = tfe::Moves { left: left_moves, right: right_moves, down: down_moves, up: up_moves };
    let mut game = tfe::Game  { board: 0x0000_0011_0000_0000_u64, moves: moves };

    println!("{:?}", game);
}
