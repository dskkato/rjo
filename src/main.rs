extern crate clap;
use clap::{App, Arg};

#[macro_use]
extern crate json;

fn do_object(args: clap::Values, data: &mut json::JsonValue) -> bool {
    for el in args {
        let kv: Vec<&str> = el.splitn(2, '=').collect();
        if kv.len() != 2 {
            panic!("Argument {:?} is not k=v", el);
        }

        if kv[0].len() == 0 {
            panic!("An empty key is not allowed {:?}", el);
        }

        let (key, value) = (kv[0], kv[1]);
        data[key] = value.into();
    }
    true
}

fn do_array(args: clap::Values, data: &mut json::JsonValue) -> bool {
    for (i, el) in args.enumerate() {
        data[i] = el.into();
    }
    true
}

fn run() -> Option<i64> {
    let matches = App::new("rjo")
        .version("0.1")
        .author("Daisuke Kato <kato.daisuke429@gmail.com>")
        .about("rjo is inspired by jo and gjo")
        .arg(
            Arg::with_name("object")
                .takes_value(true)
                .multiple(true)
                .required(true)
                .conflicts_with("array"),
        )
        .arg(
            Arg::with_name("array")
                .short("a")
                .long("array")
                .help("creates an array of words")
                .takes_value(true)
                .multiple(true)
                .conflicts_with("object"),
        )
        .arg(
            Arg::with_name("pretty-print")
                .short("p")
                .long("pretty")
                .help("pretty-prints")
                .takes_value(false),
        )
        .get_matches();

    let mut data = object! {};

    if matches.is_present("object") {
        do_object(matches.values_of("object").unwrap(), &mut data);
    } else if matches.is_present("array") {
        do_array(matches.values_of("array").unwrap(), &mut data);
    }

    if matches.is_present("pretty-print") {
        println!("{:#}", data);
    } else {
        println!("{:#}", data.dump());
    }
    None
}

fn main() {
    match run() {
        Some(x) => println!("{:?}", x),
        None => print!(""),
    }
}
