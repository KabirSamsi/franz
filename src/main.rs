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
    parse_assistant!(NoteLenParser, "qt.");
    parse_assistant!(NoteParser, "(d5, qt.)");

    notes![innocent, 0.25];
    notes![anthem, 0.3];
    notes![apprasionata, 0.15];
    notes![darthvader, 0.3];
}
