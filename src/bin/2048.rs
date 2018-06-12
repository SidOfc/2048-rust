#[macro_use]
extern crate clap;

extern crate tfe;

use tfe::Game;
use tfe::Direction;

use std::thread::spawn;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

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

    // let verbose = !arguments.is_present("quiet");
    let count   = arguments.value_of("count").unwrap_or("1").parse::<i32>().unwrap();
    let threads = arguments.value_of("threads").unwrap_or("1").parse::<i32>().unwrap();
    let per_t   = (count / threads) as i32;
    let end_c   = (threads * per_t) as usize;

    let (tx, rx): (Sender<u64>, Receiver<u64>) = mpsc::channel();
    let mut results: Vec<u64> = Vec::with_capacity(end_c - 1);

    for _ in 0 .. threads {
        let ttx = tx.clone();

        spawn(move || for _ in 0 .. per_t { ttx.send(Game::play(|b, failed| next_move(b, failed)).board).unwrap() });
    }

    for _ in 0 .. end_c { results.push(rx.recv().unwrap()) }

    let avg  = results.iter().fold(0, |mut total, &board| {total += Game::score(board); total}) as f64 / end_c as f64;
    let best = results.iter().max().unwrap();

    println!("count: {}, threads: {}, per_t: {}", count, threads, per_t);
    println!("played: {}", end_c);
    println!("average score: {}", avg);
    println!("best board: {}", Game::score(*best));

    let mut best_copy1 = best.clone();
    let mut best_copy2 = best.clone();

    println!();
    for i in 0 .. 4 {
        for _ in 0 .. 4 {
            let pow = best_copy1 & 0xF;
            let val = if pow == 0 { 0 } else { 2 << pow };

            print!("{:5}", val);
            best_copy1 >>= 4;
        }

        print!("       ");
        for _ in 0 .. 4 {
            print!("  {:2}", best_copy2 & 0xF);
            best_copy2 >>= 4;
        }

        println!();
        if i == 1 { print!("                        =>   ") }
        println!();
    }
}
