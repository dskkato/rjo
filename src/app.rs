#[macro_use]
use clap::{App, AppSettings, Arg};

pub static WORD: &'static str = "word";
pub static ARRAY: &'static str = "array";

pub fn get_app() -> clap::App<'static, 'static> {
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
