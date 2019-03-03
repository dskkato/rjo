use std::process;

extern crate clap;
use clap::{App, Arg};

extern crate json;

fn parse_value(s: &str) -> json::JsonValue {
    match json::parse(s) {
        Ok(v) => v,
        Err(_) => json::JsonValue::String(s.into()),
    }
}

fn do_object(args: clap::Values) -> json::JsonValue {
    let mut data = json::JsonValue::new_object();

    for el in args {
        let kv: Vec<&str> = el.splitn(2, '=').collect();
        if kv.len() != 2 {
            panic!("Argument {:?} is not k=v", el);
        }

        if kv[0].len() == 0 {
            panic!("An empty key is not allowed {:?}", el);
        }

        let (key, value) = (kv[0], kv[1]);
        data[key] = parse_value(value);
    }
    data
}

fn do_array(args: clap::Values) -> json::JsonValue {
    let mut data = json::JsonValue::new_array();
    for (i, value) in args.enumerate() {
        data[i] = parse_value(value);
    }
    data
}

fn run() -> Option<i32> {
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

    let data = if matches.is_present("object") {
        Some(do_object(matches.values_of("object").unwrap()))
    } else if matches.is_present("array") {
        Some(do_array(matches.values_of("array").unwrap()))
    } else {
        return Some(1);
    };

    let result = if matches.is_present("pretty-print") {
        format!("{:#}", json::stringify_pretty(data.unwrap(), 4))
    } else {
        format!("{:#}", json::stringify(data.unwrap()))
    };

    println!("{:#}", result);
    None
}

fn main() {
    match run() {
        Some(x) => process::exit(x),
        None => print!(""),
    }
}
