use std::process;

#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg};

#[macro_use]
extern crate json;
use json::{JsonValue, Result};

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

    #[test]
    fn test_return_str() {
        let s = String::from("aaa");
        assert_eq!(JsonValue::String(s.clone()), parse_value(&s));
    }

}

#[cfg(test)]
mod do_object {
    use super::*;

    #[test]
    fn test_do_object() {
        let s = vec!["a=b", "b=true", "c=1", "d=-1"];
        let result = do_object(&s).unwrap();
        let answer = object! {
            "a" => "b",
            "b" => true,
            "c" => 1,
            "d" => -1,
        };
        assert_eq!(answer, result);
    }
}

fn do_object(args: &[&str]) -> Result<JsonValue> {
    let mut data = object! {};

    for el in args {
        let kv: Vec<_> = el.splitn(2, '=').collect();
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
        let s = vec!["b", "true", "1", "-1"];
        let result = do_array(&s).unwrap();
        let answer = array!["b", true, 1, -1];
        assert_eq!(answer, result);
    }
}

fn do_array(args: &[&str]) -> Result<JsonValue> {
    let mut data = array! {};
    for value in args.iter() {
        data.push(parse_value(value))?;
    }
    Ok(data)
}

fn run() -> Result<bool> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .setting(AppSettings::AllowNegativeNumbers)
        .arg(
            Arg::with_name("object")
                .takes_value(true)
                .multiple(true)
                .required(true)
                .help("Creates a JSON object from k=v pairs"),
        )
        .arg(
            Arg::with_name("array")
                .short("a")
                .long("array")
                .help("Creates an array of words"),
        )
        .arg(
            Arg::with_name("pretty-print")
                .short("p")
                .long("pretty")
                .help("Pretty prints"),
        )
        .get_matches();

    let args: Vec<&str> = matches.values_of("object").unwrap().collect();

    let data = match matches.is_present("array") {
        true => do_array(&args).unwrap(),
        false => do_object(&args).unwrap(),
    };

    let result = match matches.is_present("pretty-print") {
        true => json::stringify_pretty(data, 4u16),
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
