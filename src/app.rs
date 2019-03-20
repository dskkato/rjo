use clap::{App, AppSettings, Arg, ArgMatches};

const WORD: &str = "word";
const ARRAY: &str = "array";
const PRETTY: &str = "pretty-print";
const DISABLE_BOOLEAN: &str = "disable boolean";

pub struct Config<'a> {
    pub args: Vec<&'a str>,
    pub is_array: bool,
    pub is_pretty: bool,
    pub disable_boolean: bool,
}

pub fn configure<'a>(matches: &'a ArgMatches) -> Config<'a> {
    Config {
        args: matches.values_of(WORD).unwrap().collect(),
        is_array: matches.is_present(ARRAY),
        is_pretty: matches.is_present(PRETTY),
        disable_boolean: matches.is_present(DISABLE_BOOLEAN),
    }
}

pub fn get_app() -> App<'static, 'static> {
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
                .help("Creates an array of words"),
        )
        .arg(
            Arg::with_name(PRETTY)
                .short("p")
                .long("pretty")
                .help("Pretty-prints JSON on output"),
        )
        .arg(
            Arg::with_name(DISABLE_BOOLEAN)
                .short("B")
                .help("Disable boolean true/false"),
        )
}
