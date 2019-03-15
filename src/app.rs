use clap::{App, AppSettings as ClapAppSettings, Arg, ArgMatches};

pub static WORD: &'static str = "word";
pub static ARRAY: &'static str = "array";
pub static PRETTY: &'static str = "pretty-print";
pub static DISABLE_BOOLEAN: &'static str = "disable boolean";

pub struct AppSettings {
    pub args: Vec<String>, // todo: use &str instead of String, if possible
    pub is_array: bool,
    pub is_pretty: bool,
    pub disable_boolean: bool,
}

impl AppSettings {
    pub fn new(matches: ArgMatches) -> AppSettings {
        AppSettings {
            args: matches.values_of(WORD).unwrap().map(String::from).collect(),
            is_array: matches.is_present(ARRAY),
            is_pretty: matches.is_present(PRETTY),
            disable_boolean: matches.is_present(DISABLE_BOOLEAN),
        }
    }
}

pub fn get_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .setting(ClapAppSettings::AllowNegativeNumbers)
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
