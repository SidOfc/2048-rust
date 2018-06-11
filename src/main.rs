extern crate rand;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate clap;

mod tfe;
use tfe::Game;
use tfe::Direction;
use std::thread;
use std::cmp::Ordering;

// references:
//  - https://github.com/nneonneo/2048-ai/blob/master/2048.h
//  - https://github.com/nneonneo/2048-ai/blob/master/2048.cpp

// TODO:
// - normalize current scores of next_move
// - add bonus for empty tiles

// fn next_move
// translates each tile value so that: TS = (x + 1) * (y + 1) * 2 * tile
// this generates the following heatmap:
//
//      |  2  |  4  |  6  |  8  |
//      |  4  |  8  | 12  | 16  |
//      |  6  | 12  | 18  | 24  |
//      |  8  | 16  | 24  | 32  |
//
// actual tile values are multiplied by their position in this map
// the sum of all actual tile values is what eventually creates the final
// output of moving a board in a given direction.
//
// note that this grid creates a very strong bias towards moving either down or right
// because of the layout of the map, this is not convenient when the value is stuck
// in a less-than-ideal situation

fn next_move(board: u64, failed: &Vec<Direction>) -> Direction {
    let available = &Direction::without(failed);
    if available.len() == 0 { return Direction::None }

    let mut results: Vec<(&Direction, u64)> = available.iter().map(|dir| {
        let after = Game::execute(board, &dir);

        (dir, ((1 << (after >> 60))       *  2 + // x:0,y:0 => (0 + 1) * (0 + 1) * 2 = 1
               (1 << (after >> 56 & 0xF)) *  4 + // x:1,y:0 => (1 + 1) * (0 + 1) * 2 = 4
               (1 << (after >> 52 & 0xF)) *  6 + // x:2,y:0 => (2 + 1) * (0 + 1) * 2 = 6
               (1 << (after >> 48 & 0xF)) *  8 + // x:3,y:0 => (3 + 1) * (0 + 1) * 2 = 8
               (1 << (after >> 44 & 0xF)) *  4 + // x:0,y:1 => (0 + 1) * (1 + 1) * 2 = 3
               (1 << (after >> 40 & 0xF)) *  8 + // x:1,y:1 => (1 + 1) * (1 + 1) * 2 = 8
               (1 << (after >> 36 & 0xF)) * 12 + // x:2,y:1 => (2 + 1) * (1 + 1) * 2 = 12
               (1 << (after >> 32 & 0xF)) * 16 + // x:3,y:1 => (3 + 1) * (1 + 1) * 2 = 16
               (1 << (after >> 28 & 0xF)) *  6 + // x:0,y:2 => (0 + 1) * (2 + 1) * 2 = 6
               (1 << (after >> 24 & 0xF)) * 12 + // x:1,y:2 => (1 + 1) * (2 + 1) * 2 = 12
               (1 << (after >> 20 & 0xF)) * 18 + // x:2,y:2 => (2 + 1) * (2 + 1) * 2 = 18
               (1 << (after >> 16 & 0xF)) * 24 + // x:3,y:2 => (3 + 1) * (2 + 1) * 2 = 24
               (1 << (after >> 12 & 0xF))      + // x:0,y:3 => (0 + 1) * (3 + 1) * 2 = 8
               (1 << (after >>  8 & 0xF)) * 16 + // x:1,y:3 => (1 + 1) * (3 + 1) * 2 = 16
               (1 << (after >>  4 & 0xF)) * 24 + // x:2,y:3 => (2 + 1) * (3 + 1) * 2 = 24
               (1 << (after >>  0 & 0xF)) * 32)) // x:3,y:3 => (3 + 1) * (3 + 1) * 2 = 32
    }).collect();

    results.sort_by(|&a, &b| {
        if b.1 > a.1 { return Ordering::Greater }
        if b.1 < a.1 { return Ordering::Less }
        Ordering::Equal
    });

    results[0].0.clone()
}

fn main() {
    let arguments = clap_app!(app =>
                        (name: "2048.rs")
                        (version: "0.1.0")
                        (author: "Sidney Liebrand <sidneyliebrand@gmail.com>")
                        (about: "a 2048 implementation using bit shifting based on github user 'nneonneo'.")
                        (@arg quiet: -q --quiet "don't print score after each game")
                        (@arg count: -c --count [COUNT] +takes_value "set the number of games played")
                        (@arg threads: -t --threads [COUNT] +takes_value "set the number of threads,\n[count/threads] games per thread.")
                    ).get_matches();

    let verbose = !arguments.is_present("quiet");
    let count   = arguments.value_of("count").unwrap_or("1").parse::<i32>().unwrap();
    let threads = arguments.value_of("threads").unwrap_or("1").parse::<i32>().unwrap();
    let per_t   = (count / threads) as i32;

    let mut handles = vec![];

    for tcount in 0..threads {
        handles.push(thread::spawn(move || {
            for gcount in 0..per_t {
                let g = Game::play(|board, attempted| next_move(board, attempted));
                if verbose { println!("{:<6} | t:{:<5} | g:{:<5}", g.score(), &tcount, &gcount) }
            }
        }));
    }

    for h in handles { h.join().unwrap() }
}
