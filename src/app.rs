use clap::{App, AppSettings, Arg};

pub static WORD: &'static str = "word";
pub static ARRAY: &'static str = "array";
pub static PRETTY: &'static str = "pretty-print";
pub static DISABLE_BOOLEAN: &'static str = "disable boolean";

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
                .help("creates an array of words"),
        )
        .arg(
            Arg::with_name(PRETTY)
                .short("p")
                .long("pretty")
                .help("pretty-prints JSON on output"),
        )
        .arg(
            Arg::with_name(DISABLE_BOOLEAN)
                .short("B")
                .help("disable boolean true/false"),
        )
}
