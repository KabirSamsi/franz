// Imports
mod ast;
mod codegen;
mod error;
mod parse;
mod preprocess;
mod songs;

macro_rules! notes {
    ($name:ident, $speed:expr) => {
        let _ =
            &codegen::compile_seq(stringify!($name), songs::$name(), $speed);
    };
}

fn main() {
    parse_assistant!(BExpParser, "(true && false) || !(true || true)");

    parse_assistant!(
        RhythmCompParser,
        "{2 * qt; qt.; {!true ? hf : {qt; qt}}}"
    );

    parse_assistant!(PitchCompParser, "a4; b4; c4_shp; 2 * {d4; e4}");

    // notes![innocent, 0.25];
    // notes![anthem, 0.3];
    // notes![apprasionata, 0.15];
    // notes![imperial_march, 0.3];
    // notes![anthem2, 0.3];
}
