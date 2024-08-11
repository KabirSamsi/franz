// Imports
mod ast;
mod codegen;
mod error;
mod parse;
mod preprocess;
mod songs;

use crate::ast::NoteComp;

macro_rules! notes {
    ($name:ident, $speed:expr) => {
        let _ =
            &codegen::compile_seq(stringify!($name), songs::$name(), $speed);
    };
}

fn main() {
    let rhythms = parse_assistant!(
        RhythmCompParser,
        "{2 * {qt; 2 * et; qt; wh.; 2 * qt; hf; hf.}}"
    );

    let pitches = parse_assistant!(
        PitchCompParser,
        "{2 * {g3; a3; b3; d4; e4; g4; e4; 2 * d4}}"
    );

    let notes = preprocess::apply_motif(
        preprocess::flatten_beat(rhythms),
        preprocess::flatten_pitch(pitches)
    );

    let mut results = Vec::new();

    for note in notes {
        results.push(NoteComp::Note(note))
    }

    let _ = codegen::compile_seq(
        "wish_you_were_here",
        NoteComp::Phrase(results),
        0.25
    );

    notes![innocent, 0.25];
    notes![anthem, 0.3];
    notes![apprasionata, 0.15];
    notes![imperial_march, 0.3];
    notes![anthem2, 0.3];
}
