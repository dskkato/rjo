extern crate clap;
use clap::{App, Arg, SubCommand};

fn run() -> Option<i64> {
    let matches = App::new("rjo")
        .version("0.1")
        .author("Daisuke Kato <kato.daisuke429@gmail.com>")
        .about("rjo is inspired by jo and gjo")
        .arg(
            Arg::with_name("config")
                .short("c") 
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);
    Some(0)
}

fn main() {
    let result = run();

    match result {
        Some(x) => println!("{:?}", x),
        None => println!("None none"),
    }
}
