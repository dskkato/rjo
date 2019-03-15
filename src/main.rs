use std::io::{self, Write};
use std::process;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate json;
use json::{JsonValue, Result};

mod app;
use app::{get_app, ARRAY};

#[cfg(test)]
mod tests;

fn parse_value(s: &str, disalbe_boolean: bool) -> JsonValue {
    if disalbe_boolean {
        match json::parse(s) {
            Ok(v) => {
                if v.is_boolean() {
                    s.into()
                } else {
                    v
                }
            }
            Err(_) => s.into(),
        }
    } else {
        match json::parse(s) {
            Ok(v) => v,
            Err(_) => s.into(),
        }
    }
}

fn do_object(args: &[String], disalbe_boolean: bool) -> Result<JsonValue> {
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
        data[key] = parse_value(value, disalbe_boolean);
    }
    Ok(data)
}

fn do_array(args: &[String], disalbe_boolean: bool) -> Result<JsonValue> {
    let mut data = array! {};
    for value in args {
        data.push(parse_value(value, disalbe_boolean))?;
    }
    Ok(data)
}

fn run(matches: clap::ArgMatches, app_settings: app::AppSettings) -> Result<bool> {
    let args = app_settings.args;
    let data = if matches.is_present(ARRAY) {
        do_array(&args, app_settings.disable_boolean).unwrap()
    } else {
        do_object(&args, app_settings.disable_boolean).unwrap()
    };

    let result = if app_settings.is_pretty {
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
    let app_settings = app::AppSettings::new(&matches);
    let result = run(matches, app_settings);

    match result {
        Ok(true) => process::exit(0),
        _ => process::exit(1),
    }
}
