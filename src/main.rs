use std::process;

extern crate clap;
use clap::{App, Arg};

#[cfg(test)]
#[macro_use]
extern crate json;
#[cfg(not(test))]
extern crate json;
use json::{Error, JsonValue, Result};

fn parse_value(s: &str) -> JsonValue {
    match json::parse(s) {
        Ok(v) => v,
        Err(_) => JsonValue::String(s.into()),
    }
}

#[cfg(test)]
mod parse_value {
    use super::*;

    #[test]
    fn test_str_str() {
        let s = String::from("{\"a\":\"b\"}");
        let o = object! {
            "a" => "b"
        };
        assert_eq!(o, parse_value(&s));
    }

    #[test]
    fn test_str_object() {
        let s = String::from("{\"a\":{\"b\":\"c\"}}");
        let o = object! {
            "a" => object! {
                "b" => "c"
            }
        };
        assert_eq!(o, parse_value(&s));
    }

}

fn do_object(args: &[&str]) -> Result<JsonValue> {
    let mut data = JsonValue::new_object();

    for el in args {
        let kv: Vec<_> = el.splitn(2, '=').collect();
        if kv.len() != 2 {
            Error::WrongType(format!("Argument {:?} is not k=v", el));
        }

        if kv[0].len() == 0 {
            Error::WrongType(format!("An empty key is not allowed {:?}", el));
        }

        let (key, value) = (kv[0], kv[1]);
        data[key] = parse_value(value);
    }
    Ok(data)
}

fn do_array(args: &[&str]) -> Result<JsonValue> {
    let mut data = JsonValue::new_array();
    for value in args.iter() {
        data.push(parse_value(value))?;
    }
    Ok(data)
}

fn run() -> Result<bool> {
    let matches = App::new("rjo")
        .version("0.1")
        .author("Daisuke Kato <kato.daisuke429@gmail.com>")
        .about("rjo inspired by jo and gjo.")
        .arg(
            Arg::with_name("object")
                .takes_value(true)
                .multiple(true)
                .required(true)
                .help("Creates a JSON object from k=v pairs")
        )
        .arg(
            Arg::with_name("array")
                .short("a")
                .long("array")
                .help("Creates an array of words")
        )
        .arg(
            Arg::with_name("pretty-print")
                .short("p")
                .long("pretty")
                .help("Pretty prints"),
        )
        .get_matches();

    let args: Vec<_> = matches.values_of("object").unwrap().collect();

    let data = match matches.is_present("array") {
        true => do_array(&args).unwrap(),
        false => do_object(&args).unwrap(),
    };

    let result = match matches.is_present("pretty-print") {
        true => data.pretty(4u16),
        false => data.dump(),
    };

    println!("{:#}", result);
    return Ok(true);
}

fn main() {
    let result = run();

    match result {
        Ok(true) => process::exit(0),
        _ => process::exit(1),
    }
}
