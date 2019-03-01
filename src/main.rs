extern crate clap;
use clap::{App, Arg};

#[macro_use]
extern crate json;

fn run() -> Option<i64> {
    let matches = App::new("rjo")
        .version("0.1")
        .author("Daisuke Kato <kato.daisuke429@gmail.com>")
        .about("rjo is inspired by jo and gjo")
        .arg(Arg::with_name("something").multiple(true))
        .get_matches();
    let iterator = matches.values_of("something");
    let mut data = object! {};
    for (i, el) in iterator.unwrap().enumerate() {
        data[i.to_string()] = el.into();
    }
    println!("{:#}", data);
    Some(0)
}

fn main() {
    let result = run();

    match result {
        Some(x) => println!("{:?}", x),
        None => println!("None none"),
    }
}
