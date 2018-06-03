mod tfe;
use tfe::{Moves, Game, Helpers};

// references:
//  - https://github.com/nneonneo/2048-ai/blob/master/2048.h
//  - https://github.com/nneonneo/2048-ai/blob/master/2048.cpp

fn main() {
    let moves = Moves::generate();
    let mut g = Game { board: 0x2200_2200_1133_1122_u64, moves: moves };

    Helpers::print(g.board);
    g.move_down();
    Helpers::print(g.board);
}
