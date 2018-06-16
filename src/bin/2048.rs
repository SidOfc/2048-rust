#[macro_use]
extern crate clap;

#[macro_use]
extern crate lazy_static;

extern crate tfe;

use tfe::Game;
use tfe::Direction;

use std::thread::spawn;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::cmp::Ordering;

struct Heuristic {
    scores: Vec<f64>
}

lazy_static! {
    static ref HEURISTICS: Heuristic = {
        let mut heuristics   = vec![0f64; 65536];
        let merge_multiplier = 1400.0;
        let empty_multiplier = 270.0;
        let sum_multiplier   = 11.0;
        let sum_power        = 3.5;
        let mono_multiplier  = 47.0;
        let mono_power       = 4.0;

        for row in 0 .. 65536 {
            // break row into cells
            let line = [
                (row >>  0) & 0xF,
                (row >>  4) & 0xF,
                (row >>  8) & 0xF,
                (row >> 12) & 0xF
            ];

            let mut prev   = 0.0;
            let mut sum    = 0.0;
            let mut empty  = 0.0;
            let mut merges = 0.0;
            let mut mono_l = 0.0;
            let mut mono_r = 0.0;

            for i in 0 .. 4 {
                let tile_pow = line[i] as f64;

                sum += tile_pow.powf(sum_power);

                if i > 0 {
                    let prev_pow = line[i - 1] as f64;
                    let strength = prev_pow.powf(mono_power) - tile_pow.powf(mono_power);

                    mono_l += strength;
                    mono_r += -strength;
                }

                if tile_pow == 0.0 {
                    empty += 1.0;
                } else if tile_pow == prev {
                    merges += 1.0;
                    prev    = 0.0;
                    continue;
                }

                prev = tile_pow;
            }

            let min_mono = if mono_l > mono_r { mono_r } else { mono_l };

            heuristics[row as usize] =
                empty_multiplier * empty    +
                merge_multiplier * merges   -
                mono_multiplier  * min_mono -
                sum_multiplier   * sum;
        }

        Heuristic { scores: heuristics }
    };
}

fn highest_tile(board: u64) -> u64 {
    let mut highest = 0;

    for i in 0 .. 16 {
        let current = board >> (i * 4) & 0xF;
        if current > highest { highest = current }
    }

    2 << highest
}

fn next_move(board: u64, attempted: &Vec<Direction>) -> Direction {
    let mut scores: Vec<(Direction, f64)> = vec![];
    for dir in Direction::without(attempted) {
        let result = Game::execute(board, &dir);
        let score  = Game::table_helper(result, &HEURISTICS.scores);
        scores.push((dir, score));
    }

    scores.sort_by(|a, b| {
        if b.1 > a.1 { return Ordering::Greater }
        if b.1 < a.1 { return Ordering::Less }
        Ordering::Equal
    });

    // println!("{:?}", scores);
    scores[0].0.clone()
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
    let end_c   = (threads * per_t) as usize;

    let (tx, rx): (Sender<u64>, Receiver<u64>) = mpsc::channel();
    let mut results: Vec<u64> = Vec::with_capacity(end_c - 1);

    for _ in 0 .. threads {
        let ttx = tx.clone();

        spawn(move || for _ in 0 .. per_t { ttx.send(Game::play(|b, failed| next_move(b, failed)).board).unwrap(); });
    }

    for _ in 0 .. end_c { results.push(rx.recv().unwrap()) }

    let scores: Vec<u64>     = (0 .. end_c).map(|i| Game::score(results[i])).collect();
    let best_tiles: Vec<u64> = (0 .. end_c).map(|i| highest_tile(results[i])).collect();
    let summed: u64          = scores.iter().sum();
    let avg_score            = summed as f64 / end_c as f64;
    let mut best_idx         = 0;
    let mut current_best     = 0;

    for i in 0 .. scores.len() {
        if scores[i] > current_best {
            current_best = scores[i];
            best_idx     = i;
        }
    }

    let best_game = results[best_idx];

    if verbose {
        println!("count: {}, threads: {}, per_t: {}", count, threads, per_t);
        println!("played: {}", end_c);
        println!("best_idx: {}", best_idx);
        println!("average score: {}", avg_score);
        println!("best board: {}", current_best);

        let mut best_copy = best_game.clone();

        println!();
        for _ in 0 .. 4 {
            for _ in 0 .. 4 {
                let pow = best_copy & 0xF;
                let val = if pow == 0 { 0 } else { 2 << pow };

                print!("{:5}", val);
                best_copy >>= 4;
            }
            println!();
        }
        println!();

        for n in &[2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768] {
            let count = best_tiles.clone().iter().filter(|t| *t >= n).count();
            let perc  = if count > 0 {  (count as f64 / end_c as f64) * 100.0 } else { 0f64 };
            println!("{:5}: ({:03.02}%) {} of {}", n, perc, count, end_c);
        }
        println!();
    }
}
