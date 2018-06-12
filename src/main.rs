extern crate rand;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate clap;

mod tfe;
use tfe::Game;
use tfe::Direction;
use std::thread;

// references:
//  - https://github.com/nneonneo/2048-ai/blob/master/2048.h
//  - https://github.com/nneonneo/2048-ai/blob/master/2048.cpp

fn next_move(board: u64, attempted: &Vec<Direction>) -> Direction {
    Direction::sample_without(attempted)
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
                if verbose { println!("{:<6} | t:{:<5} | g:{:<5}", Game::score(g.board), &tcount, &gcount) }
            }
        }));
    }

    for h in handles { h.join().unwrap() }
}
