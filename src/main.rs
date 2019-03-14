use std::process;

#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg};

#[macro_use]
extern crate json;
use json::{JsonValue, Result};

static WORD: &'static str = "word";
static ARRAY: &'static str = "array";

fn get_app() -> clap::App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .setting(AppSettings::AllowNegativeNumbers)
        .arg(
            Arg::with_name(WORD)
                .takes_value(true)
                .multiple(true)
                .required(true)
                .help("word is key=value"),
        )
        .arg(
            Arg::with_name(ARRAY)
                .short("a")
                .long(ARRAY)
                .help("creates an array of words"),
        )
        .arg(
            Arg::with_name("pretty-print")
                .short("p")
                .long("pretty")
                .help("pretty-prints JSON on output"),
        )
}

fn parse_value(s: &str) -> JsonValue {
    match json::parse(s) {
        Ok(v) => v,
        Err(_) => s.into(),
    }
}

#[cfg(test)]
mod parse_value {
    use super::*;

    #[test]
    fn test_return_object() {
        let s = "{\"a\":{\"b\":\"c\"}}";
        let o = object! {
            "a" => object! {
                "b" => "c"
            }
        };
        assert_eq!(o, parse_value(s));
    }

    #[test]
    fn test_return_str() {
        let s = "a";
        assert_eq!(JsonValue::String(s.to_owned()), parse_value(s));
    }

    #[test]
    fn test_return_true() {
        let s = "true";
        assert_eq!(JsonValue::Boolean(true), parse_value(s));
    }

    #[test]
    fn test_return_number() {
        let s = "123";
        assert_eq!(JsonValue::Number(123.into()), parse_value(s));
    }

}

#[cfg(test)]
mod do_object {
    use super::*;

    #[test]
    fn test_do_object() {
        let args = vec![crate_name!(), "a=b", "b=true", "c=1", "d=-1"];
        let matches = get_app().get_matches_from(args);

        let result = do_object(matches.values_of(WORD).unwrap());
        let expected = object! {
            "a" => "b",
            "b" => true,
            "c" => 1,
            "d" => -1,
        };
        assert_eq!(expected, result.unwrap());
    }
}

fn do_object(args: clap::Values) -> Result<JsonValue> {
    let mut data = object! {};

    for el in args {
        let kv: Vec<&str> = el.splitn(2, '=').collect();
        if kv.len() != 2 {
            panic!(format!("Argument {:?} is not k=v", el));
        }

        if kv[0].len() == 0 {
            panic!(format!("An empty key is not allowed {:?}", el));
        }

        let (key, value) = (kv[0], kv[1]);
        data[key] = parse_value(value);
    }
    Ok(data)
}

#[cfg(test)]
mod do_array {
    use super::*;

    #[test]
    fn test_do_array() {
        let args = vec![crate_name!(), "-a", "b", "true", "1", "-1"];
        let matches = get_app().get_matches_from(args);

        let result = do_array(matches.values_of(WORD).unwrap());
        let expected = array!["b", true, 1, -1];
        assert_eq!(expected, result.unwrap());
    }
}

fn do_array(args: clap::Values) -> Result<JsonValue> {
    let mut data = array! {};
    for value in args {
        println!("{}", value.clone());
        data.push(parse_value(value))?;
    }
    Ok(data)
}

fn run() -> Result<bool> {
    let matches = get_app().get_matches();

    let args = matches.values_of(WORD).unwrap();

    let data = match matches.is_present(ARRAY) {
        true => do_array(args).unwrap(),
        false => do_object(args).unwrap(),
    };

    let result = match matches.is_present("pretty-print") {
        true => json::stringify_pretty(data, 4),
        false => json::stringify(data),
    };

    println!("{}", result);
    return Ok(true);
}

fn main() {
    let result = run();

    match result {
        Ok(true) => process::exit(0),
        _ => process::exit(1),
    }
}
