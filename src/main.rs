// silence warning about unused methods
#[allow(dead_code)]

mod tfe;
use tfe::{Moves, Game, Helpers};

// references:
//  - https://github.com/nneonneo/2048-ai/blob/master/2048.h
//  - https://github.com/nneonneo/2048-ai/blob/master/2048.cpp

fn main() {
    let moves = Moves::generate();

    // up + down test
    // let mut gud = Game { board: 0x2211_2211_1122_1122_u64, moves: &moves };

    // left + right test
    let mut glr = Game { board: 0x2121_2121_1122_1122_u64, moves: &moves };

    Helpers::print(glr.board);
    glr.move_up();
    glr.move_left();
    Helpers::print(glr.board);
}
