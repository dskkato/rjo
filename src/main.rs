use std::io::{self, BufRead};
use std::process;

use json::{array, object};

mod app;
mod printer;

// for app test
#[cfg(test)]
mod tests;

const TRUE_STR: &str = "true";
const FALSE_STR: &str = "false";

fn parse_value(s: &str, disalbe_boolean: bool) -> json::JsonValue {
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

fn do_object(args: &[String], disalbe_boolean: bool) -> json::JsonValue {
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
    data
}

fn do_array(args: &[String], disalbe_boolean: bool) -> json::JsonValue {
    let mut data = array! {};
    for value in args {
        data.push(parse_value(value, disalbe_boolean)).unwrap();
    }
    data
}

fn run(config: app::Config) -> io::Result<()> {
    let args = if !config.args.is_empty() {
        config.args
    } else {
        let stdin = io::stdin();
        stdin.lock().lines().collect::<io::Result<Vec<String>>>()? // p. 427
    };

    let data = if config.is_array {
        do_array(&args, config.disable_boolean)
    } else {
        do_object(&args, config.disable_boolean)
    };

    let result = if config.is_pretty {
        json::stringify_pretty(data, 4)
    } else {
        json::stringify(data)
    };

    printer::printer(&result);

    Ok(())
}

fn main() {
    let result = {
        let matches = app::get_app().get_matches();
        let config = app::configure(&matches);
        run(config)
    };

    if result.is_err() {
        process::exit(1);
    }
}
