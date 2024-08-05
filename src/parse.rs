use crate::ast;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser);

macro_rules! parse_assistant {
    ($parser_type:ident, $sequence:expr) => {
        parser::$parser_type::new().parse($sequence).unwrap()
    };
}

pub fn parse_aexp(sequence: &str) -> ast::AExp {
    parse_assistant!(AExpParser, sequence)
}

pub fn parse_bexp(sequence: &str) -> ast::BExp {
    parse_assistant!(BExpParser, sequence)
}
