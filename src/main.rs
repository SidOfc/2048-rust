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

fn next_move(_board: u64, attempted: &Vec<Direction>) -> Direction {
    Direction::sample_without(attempted)
}

fn main() {
    let arguments = clap_app!(app =>
                        (name: "2048.rs")
                        (version: "0.1.0")
                        (author: "Sidney Liebrand <sidneyliebrand@gmail.com>")
                        (about: "2048 implemented using bit shifting based on github user nneonneo's c++ implementation")
                        (@arg quiet: -q --quiet "don't print output")
                        (@arg count: -c --count [COUNT] +takes_value {|val| {
                                if val.parse::<i32>().is_ok() {
                                    if val.parse::<i32>().unwrap() > 0 { return Ok(()) }
                                    Err(String::from("value must be >= 1"))
                                } else {
                                    Err(String::from("value must be a number!"))
                                }
                            }} "set the number of games played\n<COUNT> default: 1, min: 1\n ")
                        (@arg threads: -t --threads [THREADS] +takes_value {|val| {
                                if val.parse::<i32>().is_ok() {
                                    if val.parse::<i32>().unwrap() > 0 { return Ok(()) }
                                    Err(String::from("value must be >= 1"))
                                } else {
                                    Err(String::from("value must be a number!"))
                                }
                            }} "[<COUNT>/<THREADS>] games played per thread\n<THREADS> default: 1, min: 1\n ")
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
