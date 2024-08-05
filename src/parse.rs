use crate::ast;
use lalrpop_util::lalrpop_mod; // Add this line to import the `parser` module correctly

lalrpop_mod!(pub parser);

#[macro_export]
macro_rules! parse_assistant {
    ($parser_type:ident, $sequence:expr) => {
        $crate::parse::parser::$parser_type::new()
            .parse($sequence)
            .unwrap() // Update the path to `parser`
    };
}

pub fn parse_aexp(sequence: &str) -> ast::AExp {
    parse_assistant!(AExpParser, sequence)
}

pub fn parse_bexp(sequence: &str) -> ast::BExp {
    parse_assistant!(BExpParser, sequence)
}

pub fn parse_notelen(sequence: &str) -> ast::NoteLen {
    parse_assistant!(NoteLenParser, sequence)
}

pub fn parse_pitch(sequence: &str) -> ast::Pitch {
    parse_assistant!(PitchParser, sequence)
}

pub fn parse_note(sequence: &str) -> ast::Note {
    parse_assistant!(NoteParser, sequence)
}
