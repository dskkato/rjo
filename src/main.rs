extern crate clap;
use clap::{App, Arg, SubCommand};

use std::env;

fn run() -> Option<i64> {
    let args: Vec<String> = env::args().collect();
    Some(0)
}

fn main() {
    let result = run();

    match result {
        Some(x) => println!("{:?}", x),
        None => println!("None none"),
    }
}
