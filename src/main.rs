// Imports
mod ast;
mod codegen;
mod error;
mod parse;
mod songs;

macro_rules! notes {
    ($name:ident, $speed:expr) => {
        let _ =
            &codegen::compile_seq(stringify!($name), songs::$name(), $speed);
    };
}

fn main() {
    parse_assistant!(BExpParser, "!true && true");
    parse_assistant!(AExpParser, "12 + 20");
    parse_assistant!(AExpParser, "let helloWorld = 12 + 20");
    parse_assistant!(KeySigPitchParser, "d_shp");
    parse_assistant!(NoteLenParser, "qt.");

    parse_assistant!(ParamParser, "key= { b_flt, e_flt, a_flt };");
    parse_assistant!(ParamParser, "meter = 4/4;");
    parse_assistant!(ParamParser, "tempo = Andante;");

    parse_assistant!(RhythmCompParser, "{qt; qt; qt.}");
    parse_assistant!(PitchCompParser, "{c4; c4; g4;}");

    parse_assistant!(PitchCompParser, "{c4; c4; g4;}");

    notes![innocent, 0.25];
    notes![anthem, 0.3];
    notes![apprasionata, 0.15];
    notes![imperial_march, 0.3];
    notes![anthem2, 0.3];
}
