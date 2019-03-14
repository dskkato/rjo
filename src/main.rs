use std::io::{self, Write};
use std::process;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate json;
use json::{JsonValue, Result};

mod app;
use app::{get_app, ARRAY, WORD};

#[cfg(test)]
mod tests;

fn parse_value(s: &str) -> JsonValue {
    match json::parse(s) {
        Ok(v) => v,
        Err(_) => s.into(),
    }
}

fn do_object(args: clap::Values) -> Result<JsonValue> {
    let mut data = object! {};

    for el in args {
        let kv: Vec<&str> = el.splitn(2, '=').collect();
        if kv.len() != 2 {
            panic!(format!("Argument {:?} is not k=v", el));
        }

        if kv[0].is_empty() {
            panic!(format!("An empty key is not allowed {:?}", el));
        }

        let (key, value) = (kv[0], kv[1]);
        data[key] = parse_value(value);
    }
    Ok(data)
}

fn do_array(args: clap::Values) -> Result<JsonValue> {
    let mut data = array! {};
    for value in args {
        data.push(parse_value(value))?;
    }
    Ok(data)
}

fn run(matches: clap::ArgMatches) -> Result<bool> {
    let args = matches.values_of(WORD).unwrap();

    let data = if matches.is_present(ARRAY) {
        do_array(args).unwrap()
    } else {
        do_object(args).unwrap()
    };

    let result = if matches.is_present("pretty-print") {
        json::stringify_pretty(data, 4)
    } else {
        json::stringify(data)
    };

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", result).expect("Failed to write");

    Ok(true)
}

fn main() {
    let matches = get_app().get_matches();
    let result = run(matches);

    match result {
        Ok(true) => process::exit(0),
        _ => process::exit(1),
    }
}
