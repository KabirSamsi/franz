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
    let _ = parse_assistant!(BExpParser, "!true && true");
    let _ = parse_assistant!(AExpParser, "12 + 20");
    let _ = parse_assistant!(AExpParser, "let helloWorld = 12 + 20");
    let _ = parse_assistant!(PitchParser, "c4_shp");
    let _ = parse_assistant!(PitchParser, "d5");
    let _ = parse_assistant!(NoteLenParser, "qt.");
    let _ = parse_assistant!(NoteParser, "(d5, qt.)");

    notes![innocent, 0.25];
    notes![anthem, 0.3];
    notes![apprasionata, 0.15];
    notes![darthvader, 0.3];
}
