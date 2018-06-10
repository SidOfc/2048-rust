#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate clap;

extern crate rand;

// silence warning about unused methods
#[allow(dead_code)]

mod tfe;
use tfe::{Game, Helpers};

// references:
//  - https://github.com/nneonneo/2048-ai/blob/master/2048.h
//  - https://github.com/nneonneo/2048-ai/blob/master/2048.cpp

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
}
