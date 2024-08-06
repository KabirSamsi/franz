// Imports
mod ast;
mod codegen;
mod error;
mod parse;
// mod preprocess;
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
    parse_assistant!(PitchParser, "c4_shp");
    parse_assistant!(PitchParser, "d5");
    parse_assistant!(KeySigPitchParser, "d_shp");
    parse_assistant!(NoteLenParser, "qt.");
    parse_assistant!(NoteParser, "(d5, qt.)");

    parse_assistant!(ParamParser, "key= { b_flt, e_flt, a_flt };");
    parse_assistant!(ParamParser, "meter = 4/4;");
    parse_assistant!(ParamParser, "tempo = Andante;");

    notes![innocent, 0.25];
    notes![anthem, 0.3];
    notes![apprasionata, 0.15];
    notes![darthvader, 0.3];
}
