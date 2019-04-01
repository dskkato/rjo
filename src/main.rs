use std::process;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate json;
use json::{JsonValue, Result};

mod app;
use app::{configure, get_app, Config};

#[cfg(test)]
mod tests;

const TRUE_STR: &str = "true";
const FALSE_STR: &str = "false";

fn parse_value(s: &str, disalbe_boolean: bool) -> JsonValue {
    if disalbe_boolean {
        if s == TRUE_STR {
            TRUE_STR.into()
        } else if s == FALSE_STR {
            FALSE_STR.into()
        } else {
            match json::parse(s) {
                Ok(v) => v,
                Err(_) => s.into(),
            }
        }
    } else {
        match json::parse(s) {
            Ok(v) => v,
            Err(_) => s.into(),
        }
    }
}

fn do_object(args: &[&str], disalbe_boolean: bool) -> Result<JsonValue> {
    let mut data = object! {};

    for el in args {
        let kv: Vec<&str> = el.splitn(2, '=').collect();
        if kv.len() != 2 {
            eprintln!("Warning: Argument \"{:}\" is not k=v. Skipped.", el);
            continue;
        }

        if kv[0].is_empty() {
            eprintln!("Warning: An empty key is not allowed \"{:}\". Skipped.", el);
            continue;
        }

        let (key, value) = (kv[0], kv[1]);
        data[key] = parse_value(value, disalbe_boolean);
    }
    Ok(data)
}

fn do_array(args: &[&str], disalbe_boolean: bool) -> Result<JsonValue> {
    let mut data = array! {};
    for value in args {
        data.push(parse_value(value, disalbe_boolean))?;
    }
    Ok(data)
}

fn run(config: Config) -> Result<bool> {
    let args = &config.args;
    let data = if config.is_array {
        match do_array(args, config.disable_boolean) {
            Ok(data) => data,
            Err(_) => return Ok(false),
        }
    } else {
        match do_object(args, config.disable_boolean) {
            Ok(data) => data,
            Err(_) => return Ok(false),
        }
    };

    let result = if config.is_pretty {
        json::stringify_pretty(data, 4)
    } else {
        json::stringify(data)
    };

    println!("{}", result);

    Ok(true)
}

fn main() {
    let result = {
        let matches = get_app().get_matches();
        let config = configure(&matches);
        run(config)
    };

    match result {
        Ok(true) => process::exit(0),
        _ => process::exit(1),
    }
}
