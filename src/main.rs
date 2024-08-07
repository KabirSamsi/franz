// Imports
mod ast;
mod codegen;
mod error;
mod parse;
// mod preprocess;
mod songs;
use crate::ast::NoteComp;

macro_rules! notes {
    ($name:ident, $speed:expr) => {
        let _ =
            &codegen::compile_seq(stringify!($name), songs::$name(), $speed);
    };
}

fn try_to_build_a_song() {
    let husn = NoteComp::Phrase(vec![
        parse_assistant!(NoteParser, "(d4, et)"),
        parse_assistant!(NoteParser, "(f4, et)"),
        parse_assistant!(NoteParser, "(d4, et)"),
        parse_assistant!(NoteParser, "(f4, et)"),
        parse_assistant!(NoteParser, "(d4, et)"),
        parse_assistant!(NoteParser, "(f4, et)"),
        parse_assistant!(NoteParser, "(d4, et)"),
        parse_assistant!(NoteParser, "(f4, qt)"),
        parse_assistant!(NoteParser, "(d4, et)"),
        parse_assistant!(NoteParser, "(f4, qt)"),
        parse_assistant!(NoteParser, "(d4, qt)"),
        parse_assistant!(NoteParser, "(c4, qt)"),
        parse_assistant!(NoteParser, "(f4, et)"),
        parse_assistant!(NoteParser, "(g4, qt)"),
        parse_assistant!(NoteParser, "(f4, et)"),
        parse_assistant!(NoteParser, "(g4, qt)"),
        parse_assistant!(NoteParser, "(f4, et)"),
        parse_assistant!(NoteParser, "(g4, qt)"),
        parse_assistant!(NoteParser, "(a4, et)"),
        parse_assistant!(NoteParser, "(g4, qt)"),
        parse_assistant!(NoteParser, "(f4, et)"),
        parse_assistant!(NoteParser, "(e4, qt)"),
        parse_assistant!(NoteParser, "(g4, et)"),
        parse_assistant!(NoteParser, "(g4, qt)"),
        parse_assistant!(NoteParser, "(f4, et)"),
        parse_assistant!(NoteParser, "(g4, qt)"),
        parse_assistant!(NoteParser, "(g4, et)"),
        parse_assistant!(NoteParser, "(a4, hf)"),
        parse_assistant!(NoteParser, "(g4, qt)"),
        parse_assistant!(NoteParser, "(a4, et)"),
        parse_assistant!(NoteParser, "(f4, et)"),
        parse_assistant!(NoteParser, "(e4, et)"),
        parse_assistant!(NoteParser, "(d4, qt)"),
        parse_assistant!(NoteParser, "(d4, hf)"),
        parse_assistant!(NoteParser, "(d4, hf)"),
        parse_assistant!(NoteParser, "(d4, hf)"),
        parse_assistant!(NoteParser, "(d4, et)"),
        parse_assistant!(NoteParser, "(f4, et)"),
        parse_assistant!(NoteParser, "(d4, et)"),
        parse_assistant!(NoteParser, "(f4, et)"),
        parse_assistant!(NoteParser, "(d4, et)"),
        parse_assistant!(NoteParser, "(f4, et)"),
        parse_assistant!(NoteParser, "(d4, et)"),
        parse_assistant!(NoteParser, "(f4, qt)"),
        parse_assistant!(NoteParser, "(d4, et)"),
        parse_assistant!(NoteParser, "(f4, qt)"),
        parse_assistant!(NoteParser, "(d4, hf)"),
        parse_assistant!(NoteParser, "(f4, et)"),
        parse_assistant!(NoteParser, "(g4, qt)"),
        parse_assistant!(NoteParser, "(f4, et)"),
        parse_assistant!(NoteParser, "(g4, qt)"),
        parse_assistant!(NoteParser, "(f4, et)"),
        parse_assistant!(NoteParser, "(g4, qt)"),
        parse_assistant!(NoteParser, "(a4, et)"),
        parse_assistant!(NoteParser, "(g4, qt)"),
        parse_assistant!(NoteParser, "(f4, et)"),
        parse_assistant!(NoteParser, "(e4, qt)"),
        parse_assistant!(NoteParser, "(g4, et)"),
        parse_assistant!(NoteParser, "(g4, qt)"),
        parse_assistant!(NoteParser, "(f4, et)"),
        parse_assistant!(NoteParser, "(g4, qt)"),
        parse_assistant!(NoteParser, "(g4, et)"),
        parse_assistant!(NoteParser, "(b4_flt, hf)"),
        parse_assistant!(NoteParser, "(a4, qt)"),
        parse_assistant!(NoteParser, "(g4, et)"),
        parse_assistant!(NoteParser, "(a4, et)"),
        parse_assistant!(NoteParser, "(g4, et)"),
        parse_assistant!(NoteParser, "(f4, qt)"),
        parse_assistant!(NoteParser, "(f4, hf)"),
        parse_assistant!(NoteParser, "(f4, hf)"),
        parse_assistant!(NoteParser, "(f4, hf)"),
    ]);

    let _ = codegen::compile_seq("husn", husn, 0.25);
}

fn main() {
    parse_assistant!(BExpParser, "!true && true");
    parse_assistant!(AExpParser, "12 + 20");
    parse_assistant!(AExpParser, "let helloWorld = 12 + 20");
    parse_assistant!(PitchParser, "c4_shp");
    parse_assistant!(PitchParser, "d5");
    parse_assistant!(KeySigPitchParser, "d_shp");
    parse_assistant!(NoteLenParser, "qt.");
    parse_assistant!(NoteParser, "(d5, qt)");

    parse_assistant!(ParamParser, "key= { b_flt, e_flt, a_flt };");
    parse_assistant!(ParamParser, "meter = 4/4;");
    parse_assistant!(ParamParser, "tempo = Andante;");

    try_to_build_a_song();
    notes![innocent, 0.25];
    notes![anthem, 0.3];
    notes![apprasionata, 0.15];
    notes![imperial_march, 0.3];
}
