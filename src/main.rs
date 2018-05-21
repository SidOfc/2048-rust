#![allow(dead_code)]

// references:
//  - https://github.com/nneonneo/2048-ai/blob/master/2048.h
//  - https://github.com/nneonneo/2048-ai/blob/master/2048.cpp
//
//  module tfe - [t]wenty_[f]orty_[e]ight.
//  namespace for 2048 game module.
mod tfe {
    // a 64bit mask with a single section of 16 bits set to 0.
    // used to extract a "horizontal slice" out of a 64 bit integer.
    const ROW_MASK: u64 = !0b1111_1111_1111_1111u64;

    // a 64bit mask with the leftmost 4 bits set to to 1.
    // used to extract single tile.
    const VAL_MASK: u64 = 0b1111u64;

    // a 64bit mask with 4 sections each starting after the n * 16th bit.
    // used to extract a "vertical slice" out of a 64 bit integer.
    const COL_MASK: u64 = !(0b1111u64 | (0b1111u64 << 16) | (0b1111u64 << 32) | (0b1111u64 << 48));

    // generic container for a game.
    // includes margin property to offset printing the board from the left edge of the screen.
    pub struct Container {
        pub board:  u64,
        pub margin: usize,
    }

    // container functions.
    impl Container {
        // print board from self.board with offset self.margin.
        pub fn print(&self) {
            let spacer: String  = " ".repeat(self.margin);

            // map 4 bits to one digit, 64 bits / 16 tiles / 4 bits per tile.
            let tiles: Vec<u64> = (0..16).map(|n| 1u64 << (self.board >> ((15 - n) << 2) & VAL_MASK))
                                         .map(|r| if r > 1 { r } else { 0 }).collect();

            // print top area
            println!("{}*-------------------------------------------*", spacer);
            println!("{}|   _____________________________________   |", spacer);

            // print game area
            for row in tiles.chunks(4) {
                println!("{}|   |        |        |        |        |   |", spacer);
                println!("{}|   |{:^8}|{:^8}|{:^8}|{:^8}|   |", spacer, row[0], row[1], row[2], row[3]);
            }

            // print bottom area
            println!("{}|   |________|________|________|________|   |", spacer);
            println!("{}|                                           |", spacer);
            println!("{}*-------------------------------------------*", spacer);
        }
    }
}

fn main() {
    let game = tfe::Container {
        board: 0b0000000000000000000000000000000001000000000000000000000100101111u64,
        margin: 10
    };

    game.print();
}
