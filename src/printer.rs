#[cfg(windows)]
use ansi_term;
use atty::Stream;
use syntect::dumps;
use syntect::easy::HighlightLines;
use syntect::highlighting::Style;
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;

pub fn printer(s: &str) {
    if atty::is(Stream::Stdout) {
        #[cfg(windows)]
        let _enabled = ansi_term::enable_ansi_support();

        let ps = SyntaxSet::load_defaults_newlines();
        let th = dumps::from_binary(include_bytes!("../assets/Monokai.bin"));

        let syntax = ps.find_syntax_by_extension("json").unwrap();
        let mut h = HighlightLines::new(syntax, &th);

        let ranges: Vec<(Style, &str)> = h.highlight(s, &ps);
        let escaped = as_24_bit_terminal_escaped(&ranges, false);
        println!("{}", escaped);
        print!("\x1b[0m");
    } else {
        println!("{}", s);
    }
}
