// Imports
mod ast;
mod codegen;
mod error;
mod parse;
mod preprocess;
mod songs;

use std::fs;

macro_rules! notes {
    ($name:ident, $speed:expr) => {
        let _ =
            &codegen::compile_seq(stringify!($name), songs::$name(), $speed);
    };
}

fn test_parse_song(name: &str) {
    let s = fs::read_to_string(format!("franz-programs/{name}.fz")).expect("");
    dbg!(parse_assistant!(ControlParser, &s));
}

fn main() {
    test_parse_song("imperial_march");
    test_parse_song("anthem");
    test_parse_song("anthem2");
    test_parse_song("innocent");

    notes![innocent, 0.25];
    notes![anthem, 0.3];
    notes![apprasionata, 0.15];
    notes![imperial_march, 0.3];
    notes![anthem2, 0.3];
}
